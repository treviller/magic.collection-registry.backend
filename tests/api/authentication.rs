use actix_web::dev::{Response, ServiceRequest};
use actix_web::http::header::ContentType;
use actix_web::{test, App, HttpRequest};
use tracing_actix_web::TracingLogger;

use magic_collection_registry_backend::app::{configure_routing, Application};
use magic_collection_registry_backend::monitoring::{get_subscriber, initialize_subscriber};
use magic_collection_registry_backend::routes::authentication::login;

#[actix_web::test]
pub async fn login_should_return_200() {
    let subscriber = get_subscriber("info".into());
    initialize_subscriber(subscriber);
    let app = App::new()
        .wrap(TracingLogger::default())
        .configure(configure_routing);
    let test_app = test::init_service(app).await;

    let req = test::TestRequest::post()
        .uri("/login")
        .insert_header(ContentType::json())
        .set_json(serde_json::json!({
            "login": "JohnDoe",
            "password": "test"
        }))
        .to_request();
    let response = test::call_service(&test_app, req).await;

    assert!(response.status().is_success())
}
