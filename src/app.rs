use actix_web::dev::{Server, Service, ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::{test, web, App, Error, HttpServer};
use tracing::subscriber::set_global_default;
use tracing_actix_web::TracingLogger;
use tracing_log::LogTracer;

use crate::monitoring::{get_subscriber, initialize_subscriber};
use crate::routes::authentication::login;

pub struct Application {
    server: Server,
}

impl Application {
    pub fn build() -> Result<Self, std::io::Error> {
        let subscriber = get_subscriber("info".into());
        initialize_subscriber(subscriber);

        Ok(Self {
            server: Application::create_server()?,
        })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        self.server.await
    }

    fn create_server() -> Result<Server, std::io::Error> {
        let server = HttpServer::new(|| {
            App::new()
                .wrap(TracingLogger::default())
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
