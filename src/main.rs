use actix_web::{get, App, HttpResponse, HttpServer};
use tracing::subscriber::set_global_default;
use tracing_actix_web::TracingLogger;
use tracing_log::LogTracer;

use magic_collection_registry_backend::monitoring::get_subscriber;

#[tracing::instrument(name = "Hello world")]
#[get("/")]
async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    LogTracer::init().expect("Failed to initialize logger.");
    set_global_default(get_subscriber("info".into())).expect("Failed to set subscriber.");

    HttpServer::new(|| App::new().wrap(TracingLogger::default()).service(hello))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
