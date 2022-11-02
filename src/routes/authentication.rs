use actix_web::{post, web, HttpResponse};
use secrecy::Secret;

use crate::domain::authentication::{check_credentials, generate_jwt};

#[derive(serde::Deserialize)]
pub struct LoginData {
    login: String,
    password: Secret<String>,
}

#[tracing::instrument(name = "Login request", skip(request_data))]
#[post("/login")]
pub async fn login(request_data: web::Json<LoginData>) -> HttpResponse {
    check_credentials(request_data.0.password);

    HttpResponse::Ok().json(generate_jwt())
}
