use actix_web::{post, web, HttpResponse};
use secrecy::Secret;

use crate::domain::authentication::{check_credentials, AuthenticationService};

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
    check_credentials(request_data.0.password);

    HttpResponse::Ok().json(authentication_service.generate_jwt())
}
