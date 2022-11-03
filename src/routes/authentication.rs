use actix_web::{post, web, HttpResponse};
use secrecy::Secret;

use crate::domain::authentication::AuthenticationService;

#[derive(serde::Deserialize)]
pub struct LoginData {
    login: String,
    password: Secret<String>,
}

#[tracing::instrument(name = "Login request", skip(request_data, authentication_service))]
#[post("/login")]
pub async fn login(
    request_data: web::Json<LoginData>,
    authentication_service: web::Data<AuthenticationService>,
) -> HttpResponse {
    let user = authentication_service
        .login_user(request_data.0.login, request_data.0.password)
        .expect("No error for now");

    HttpResponse::Ok().json(authentication_service.generate_jwt(user))
}
