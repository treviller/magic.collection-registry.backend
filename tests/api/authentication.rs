use actix_web::http::header;
use actix_web::http::header::ContentType;
use actix_web::test;
use secrecy::ExposeSecret;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

use magic_collection_registry_backend::authentication::AuthenticationService;
use magic_collection_registry_backend::configuration::loader::get_configuration;
use magic_collection_registry_backend::provider::database::DbConnection;
use magic_collection_registry_backend::routes::responses::authentication::LoginResponse;

use crate::helpers;
use crate::helpers::{generate_access_token, mock_email_server};

#[sqlx::test(fixtures("users", "tokens"))]
pub async fn login_should_return_200(db_pool: DbConnection) {
    let configuration = get_configuration().expect("Failed to build configuration.");
    let authentication_service = AuthenticationService::new(configuration.auth.clone());

    let req = test::TestRequest::post()
        .uri("/api/login")
        .insert_header(ContentType::json())
        .set_json(serde_json::json!({
            "login": "test@email.com",
            "password": "test"
        }));

    let response = helpers::init_test_app_and_make_request(db_pool, configuration, req).await;
    assert!(response.status().is_success());

    let json: LoginResponse = test::read_body_json(response).await;

    authentication_service
        .decode_jwt(&json.meta.access_token)
        .expect("Access token should be a valid JWT");
}

#[sqlx::test(fixtures("users", "tokens"))]
pub async fn get_profile_should_return_200(db_pool: DbConnection) {
    let configuration = get_configuration().expect("Failed to build configuration.");

    let req = test::TestRequest::get()
        .uri("/api/profile")
        .insert_header(ContentType::json())
        .insert_header((
            header::AUTHORIZATION,
            format!(
                "Bearer {}",
                generate_access_token(&db_pool, &configuration)
                    .await
                    .expose_secret()
                    .as_str()
            ),
        ));

    let response = helpers::init_test_app_and_make_request(db_pool, configuration, req).await;
    assert!(response.status().is_success());
}

#[sqlx::test(fixtures("users", "tokens"))]
pub async fn get_profile_without_jwt_should_return_401(db_pool: DbConnection) {
    let configuration = get_configuration().expect("Failed to build configuration.");

    let req = test::TestRequest::get()
        .uri("/api/profile")
        .insert_header(ContentType::json());

    let response = helpers::init_test_app_and_make_request(db_pool, configuration, req).await;
    assert_eq!(response.status().as_u16(), 401);
}

#[sqlx::test(fixtures("users", "tokens"))]
pub async fn forgotten_password_should_send_email(db_pool: DbConnection) {
    let mut configuration = get_configuration().expect("Failed to build configuration.");
    let email_server = mock_email_server(&mut configuration).await;

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

    let response = helpers::init_test_app_and_make_request(db_pool, configuration, req).await;
    assert!(response.status().is_success());
}

#[sqlx::test(fixtures("users", "tokens"))]
pub async fn reset_password_should_return_200_with_valid_token(db_pool: DbConnection) {
    let token = "bbfddad7-940e-4a85-a35d-925c91b438bd";
    let configuration = get_configuration().expect("Failed to build configuration.");

    let req = test::TestRequest::put()
        .uri(&format!("/api/password-reset/{}", token))
        .insert_header(ContentType::json())
        .set_json(serde_json::json!({
            "password": "nosecret"
        }));

    let response = helpers::init_test_app_and_make_request(db_pool, configuration, req).await;
    assert!(response.status().is_success());
}

#[sqlx::test(fixtures("users", "tokens"))]
pub async fn reset_password_should_return_400_with_invalid_token(db_pool: DbConnection) {
    let token = "invalidtoken";
    let configuration = get_configuration().expect("Failed to build configuration.");

    let req = test::TestRequest::put()
        .uri(&format!("/api/password-reset/{}", token))
        .insert_header(ContentType::json())
        .set_json(serde_json::json!({
            "password": "nosecret"
        }));

    let response = helpers::init_test_app_and_make_request(db_pool, configuration, req).await;
    assert_eq!(response.status().as_u16(), 400);
}

#[sqlx::test(fixtures("users", "tokens"))]
pub async fn reset_password_should_return_400_with_invalid_payload(db_pool: DbConnection) {
    let token = "bbfddad7-940e-4a85-a35d-925c91b438bd";
    let configuration = get_configuration().expect("Failed to build configuration.");

    let req = test::TestRequest::put()
        .uri(&format!("/api/password-reset/{}", token))
        .insert_header(ContentType::json())
        .set_json(serde_json::json!({
            "notusedkey": "blabla"
        }));

    let response = helpers::init_test_app_and_make_request(db_pool, configuration, req).await;
    assert_eq!(response.status().as_u16(), 400);
}
