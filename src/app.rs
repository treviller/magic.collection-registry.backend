use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use tracing_actix_web::TracingLogger;

use crate::configuration::settings::Settings;
use crate::container::ServiceContainer;
use crate::monitoring::{get_subscriber, initialize_subscriber};
use crate::routes::authentication::login;

pub struct Application {
    server: Server,
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
        let container = ServiceContainer::new(configuration);

        let server = HttpServer::new(move || {
            App::new()
                .wrap(TracingLogger::default())
                .configure(|cfg| configure_services(cfg, container.clone()))
                .configure(configure_routing)
        })
        .bind(("127.0.0.1", 8080))?
        .run();

        Ok(server)
    }
}

pub fn configure_routing(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
}

pub fn configure_services(cfg: &mut web::ServiceConfig, container: ServiceContainer) {
    cfg.app_data(container.authentication_service);
}
