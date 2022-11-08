use actix_web::http::header;
use actix_web::http::header::ContentType;
use actix_web::test;
use secrecy::ExposeSecret;

use magic_collection_registry_backend::authentication::AuthenticationService;
use magic_collection_registry_backend::configuration::loader::get_configuration;
use magic_collection_registry_backend::provider::memory::user::UserMemoryProvider;
use magic_collection_registry_backend::provider::user::UserProvider;

use crate::helpers;

#[derive(Debug, serde::Deserialize)]
pub struct LoginJsonResponse {
    access_token: String,
}

#[actix_web::test]
pub async fn login_should_return_200() {
    let configuration = get_configuration().expect("Failed to build configuration.");
    let authentication_service = AuthenticationService::new(configuration.auth.clone());

    let req = test::TestRequest::post()
        .uri("/api/login")
        .insert_header(ContentType::json())
        .set_json(serde_json::json!({
            "login": "user1",
            "password": "test"
        }));

    let response = helpers::init_test_app_and_make_request(configuration, req).await;
    assert!(response.status().is_success());

    let json: LoginJsonResponse = test::read_body_json(response).await;

    authentication_service
        .decode_jwt(&json.access_token)
        .expect("Access token should be a valid JWT");
}

#[actix_web::test]
pub async fn get_profile_should_return_200() {
    let configuration = get_configuration().expect("Failed to build configuration.");
    let authentication_service = AuthenticationService::new(configuration.auth.clone());
    let user_provider = UserMemoryProvider::new();
    let token = authentication_service
        .generate_jwt(user_provider.find_one_by_username("user1".into()).unwrap());

    let req = test::TestRequest::get()
        .uri("/api/profile")
        .insert_header(ContentType::json())
        .insert_header((
            header::AUTHORIZATION,
            format!("Bearer {}", token.access_token.expose_secret().as_str()),
        ));

    let response = helpers::init_test_app_and_make_request(configuration, req).await;
    assert!(response.status().is_success());
}
