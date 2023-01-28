use std::env;

use actix_cors::Cors;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use tera::Tera;
use tracing_actix_web::TracingLogger;

use crate::configuration::settings::Settings;
use crate::monitoring::{get_subscriber, initialize_subscriber};
use crate::provider::database::establish_connection_pool;
use crate::routes::authentication::{forgotten_password, get_profile, login, reset_password};
use crate::routes::cards::list_cards;
use crate::routes::sets::get_sets_list;

pub struct Application {
    server: Server,
}

impl Application {
    pub fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        load_environment_values();

        let subscriber = get_subscriber("info".into());
        initialize_subscriber(subscriber);

        Ok(Self {
            server: Application::create_server(configuration)?,
        })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        self.server.await
    }

    fn create_server(configuration: Settings) -> Result<Server, anyhow::Error> {
        let config_data = web::Data::new(configuration.clone());
        let db_pool = web::Data::new(establish_connection_pool());

        let tera = web::Data::new(initialize_tera());
        let server = HttpServer::new(move || {
            App::new()
                .wrap(TracingLogger::default())
                .wrap(Application::build_cors_configuration())
                .app_data(config_data.clone())
                .app_data(tera.clone())
                .app_data(db_pool.clone())
                .configure(configure_routing)
        })
        .bind((configuration.host.address, configuration.host.port))?
        .run();

        Ok(server)
    }

    fn build_cors_configuration() -> Cors {
        Cors::default()
            .allowed_origin(&env::var("FRONT_URL").expect("FRONT_URL value should be set"))
            .allow_any_method()
            .allow_any_header()
            .max_age(3600)
    }
}

pub fn configure_routing(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(login)
            .service(get_profile)
            .service(forgotten_password)
            .service(reset_password)
            .service(get_sets_list)
            .service(list_cards),
    );
}

pub fn initialize_tera() -> Tera {
    let tera = match Tera::new("src/templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            tracing::error!("Parsing error(s): {}", e);
            Tera::default()
        }
    };

    tera
}

pub fn load_environment_values() {
    dotenv().ok();
}
