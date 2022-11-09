use std::sync::Mutex;

use actix_web::http::header;
use actix_web::http::header::ContentType;
use actix_web::test;
use secrecy::ExposeSecret;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

use magic_collection_registry_backend::authentication::AuthenticationService;
use magic_collection_registry_backend::configuration::loader::get_configuration;
use magic_collection_registry_backend::provider::memory::MemoryStorage;

use crate::helpers;
use crate::helpers::{generate_access_token, mock_email_server};

#[derive(Debug, serde::Deserialize)]
pub struct LoginJsonResponse {
    access_token: String,
}

#[actix_web::test]
pub async fn login_should_return_200() {
    let configuration = get_configuration().expect("Failed to build configuration.");
    let authentication_service = AuthenticationService::new(configuration.auth.clone());
    let memory_storage = Mutex::new(MemoryStorage::new());

    let req = test::TestRequest::post()
        .uri("/api/login")
        .insert_header(ContentType::json())
        .set_json(serde_json::json!({
            "login": "user1",
            "password": "test"
        }));

    let response =
        helpers::init_test_app_and_make_request(configuration, memory_storage, req).await;
    assert!(response.status().is_success());

    let json: LoginJsonResponse = test::read_body_json(response).await;

    authentication_service
        .decode_jwt(&json.access_token)
        .expect("Access token should be a valid JWT");
}

#[actix_web::test]
pub async fn get_profile_should_return_200() {
    let configuration = get_configuration().expect("Failed to build configuration.");
    let memory_storage = Mutex::new(MemoryStorage::new());

    let req = test::TestRequest::get()
        .uri("/api/profile")
        .insert_header(ContentType::json())
        .insert_header((
            header::AUTHORIZATION,
            format!(
                "Bearer {}",
                generate_access_token(&configuration, &memory_storage)
                    .expose_secret()
                    .as_str()
            ),
        ));

    let response =
        helpers::init_test_app_and_make_request(configuration, memory_storage, req).await;
    assert!(response.status().is_success());
}

#[actix_web::test]
pub async fn get_profile_without_jwt_should_return_401() {
    let configuration = get_configuration().expect("Failed to build configuration.");
    let memory_storage = Mutex::new(MemoryStorage::new());

    let req = test::TestRequest::get()
        .uri("/api/profile")
        .insert_header(ContentType::json());

    let response =
        helpers::init_test_app_and_make_request(configuration, memory_storage, req).await;
    assert_eq!(response.status().as_u16(), 401);
}

#[actix_web::test]
pub async fn forgotten_password_should_send_email() {
    let mut configuration = get_configuration().expect("Failed to build configuration.");
    let email_server = mock_email_server(&mut configuration).await;
    let memory_storage = Mutex::new(MemoryStorage::new());

    Mock::given(path("/send"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&email_server)
        .await;

    let req = test::TestRequest::post()
        .uri("/api/password-reset")
        .insert_header(ContentType::json())
        .set_json(serde_json::json!({
            "email": "test@email.com"
        }));

    let response =
        helpers::init_test_app_and_make_request(configuration, memory_storage, req).await;
    assert!(response.status().is_success());
}
