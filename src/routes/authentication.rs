use actix_web::{get, post, web, HttpResponse};
use anyhow::Context;
use secrecy::{ExposeSecret, Secret};

use crate::authentication::AuthenticationService;
use crate::configuration::settings::Settings;
use crate::domain::model::user::User;
use crate::domain::user::UserService;
use crate::dto::user::UserDto;
use crate::errors::api::login::LoginError;

#[derive(serde::Deserialize)]
pub struct LoginData {
    login: String,
    password: Secret<String>,
}

#[tracing::instrument(name = "Login request", skip(request_data, config))]
#[post("/api/login")]
pub async fn login(
    request_data: web::Json<LoginData>,
    config: web::Data<Settings>,
) -> Result<HttpResponse, LoginError> {
    let authentication_service = AuthenticationService::new(config.auth.clone());
    let user_service = UserService::new();

    let user = user_service
        .get_user_from_username(&request_data.0.login)
        .context("Unable to found user with username")
        .map_err(LoginError::InvalidCredentials)?;

    authentication_service
        .check_credentials(&user, request_data.0.password)
        .map_err(|e| LoginError::from(e))?;

    let tokens = authentication_service
        .generate_jwt(user)
        .map_err(|e| LoginError::from(e))?;

    Ok(HttpResponse::Ok().json(tokens))
}

#[get("/api/profile")]
pub async fn get_profile(user: User) -> HttpResponse {
    HttpResponse::Ok().json(UserDto::from_user(user))
}
