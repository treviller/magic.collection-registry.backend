use std::sync::Mutex;

use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use tracing_actix_web::TracingLogger;

use crate::configuration::settings::Settings;
use crate::monitoring::{get_subscriber, initialize_subscriber};
use crate::provider::memory::MemoryStorage;
use crate::routes::authentication::{forgotten_password, get_profile, login, reset_password};

pub struct Application {
    server: Server,
}

pub struct MutStorage {
    pub storage: Mutex<MemoryStorage>,
}

impl Application {
    pub fn build(configuration: Settings) -> Result<Self, std::io::Error> {
        let subscriber = get_subscriber("info".into());
        initialize_subscriber(subscriber);

        Ok(Self {
            server: Application::create_server(configuration)?,
        })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        self.server.await
    }

    fn create_server(configuration: Settings) -> Result<Server, std::io::Error> {
        let config_data = web::Data::new(configuration.clone());
        let memory_storage = web::Data::new(MutStorage {
            storage: Mutex::new(MemoryStorage::new()),
        });

        let server = HttpServer::new(move || {
            App::new()
                .wrap(TracingLogger::default())
                .app_data(config_data.clone())
                .app_data(memory_storage.clone())
                .configure(configure_routing)
        })
        .bind(("127.0.0.1", 8080))?
        .run();

        Ok(server)
    }
}

pub fn configure_routing(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(login)
            .service(get_profile)
            .service(forgotten_password)
            .service(reset_password),
    );
}
