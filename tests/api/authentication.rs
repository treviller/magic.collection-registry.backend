use actix_web::http::header;
use actix_web::http::header::ContentType;
use actix_web::{test, web, App};
use once_cell::sync::Lazy;
use secrecy::ExposeSecret;
use tracing_actix_web::TracingLogger;

use magic_collection_registry_backend::app::configure_routing;
use magic_collection_registry_backend::authentication::AuthenticationService;
use magic_collection_registry_backend::configuration::loader::get_configuration;
use magic_collection_registry_backend::monitoring::{get_subscriber, initialize_subscriber};
use magic_collection_registry_backend::provider::memory::user::UserMemoryProvider;
use magic_collection_registry_backend::provider::user::UserProvider;

static TRACING: Lazy<()> = Lazy::new(|| {
    let subscriber = get_subscriber("trace".into());
    initialize_subscriber(subscriber);
});

#[derive(Debug, serde::Deserialize)]
pub struct LoginJsonResponse {
    access_token: String,
}

#[actix_web::test]
pub async fn login_should_return_200() {
    Lazy::force(&TRACING);

    let configuration = get_configuration().expect("Failed to build configuration.");
    let config_data = web::Data::new(configuration.clone());

    let app = App::new()
        .wrap(TracingLogger::default())
        .app_data(config_data.clone())
        .configure(configure_routing);
    let test_app = test::init_service(app).await;

    let req = test::TestRequest::post()
        .uri("/api/login")
        .insert_header(ContentType::json())
        .set_json(serde_json::json!({
            "login": "user1",
            "password": "test"
        }))
        .to_request();
    let response = test::call_service(&test_app, req).await;
    assert!(response.status().is_success());

    let authentication_service = AuthenticationService::new(configuration.auth);
    let json: LoginJsonResponse = test::read_body_json(response).await;

    authentication_service
        .decode_jwt(&json.access_token)
        .expect("Access token should be a valid JWT");
}

#[actix_web::test]
pub async fn get_profile_should_return_200() {
    Lazy::force(&TRACING);

    let configuration = get_configuration().expect("Failed to build configuration.");
    let config_data = web::Data::new(configuration.clone());
    let authentication_service = AuthenticationService::new(configuration.auth);

    let app = App::new()
        .wrap(TracingLogger::default())
        .app_data(config_data.clone())
        .configure(configure_routing);

    let test_app = test::init_service(app).await;

    let user_provider = UserMemoryProvider::new();
    let token = authentication_service
        .generate_jwt(user_provider.find_one_by_username("user1".into()).unwrap());

    let req = test::TestRequest::get()
        .uri("/api/profile")
        .insert_header(ContentType::json())
        .insert_header((
            header::AUTHORIZATION,
            format!("Bearer {}", token.access_token.expose_secret().as_str()),
        ))
        .to_request();
    let response = test::call_service(&test_app, req).await;
    assert!(response.status().is_success());
}
