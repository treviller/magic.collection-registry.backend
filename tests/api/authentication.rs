use actix_web::http::header::ContentType;
use actix_web::{test, App};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use tracing_actix_web::TracingLogger;

use magic_collection_registry_backend::app::{configure_routing, configure_services};
use magic_collection_registry_backend::configuration::loader::get_configuration;
use magic_collection_registry_backend::domain::authentication::Claims;
use magic_collection_registry_backend::monitoring::{get_subscriber, initialize_subscriber};

#[derive(Debug, serde::Deserialize)]
pub struct LoginJsonResponse {
    access_token: String,
}

#[actix_web::test]
pub async fn login_should_return_200() {
    let subscriber = get_subscriber("info".into());
    initialize_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to build configuration.");

    let app = App::new()
        .wrap(TracingLogger::default())
        .configure(configure_routing)
        .configure(|cfg| configure_services(cfg, configuration));
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
    assert!(response.status().is_success());

    let json: LoginJsonResponse = test::read_body_json(response).await;

    let decoding_key = "testkey".as_ref();

    let claims = decode::<Claims>(
        json.access_token.as_str(),
        &DecodingKey::from_secret(decoding_key),
        &Validation::new(Algorithm::HS256),
    )
    .expect("Access token should be a valid JWT");
}
