use actix_web::{get, post, put, web, HttpResponse};
use anyhow::Context;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::Pool;
use secrecy::Secret;
use tera::Tera;
use uuid::Uuid;

use crate::authentication::AuthenticationService;
use crate::configuration::settings::Settings;
use crate::domain::model::user::User;
use crate::domain::model::user_email::UserEmail;
use crate::domain::token::TokenService;
use crate::domain::user::UserService;
use crate::dto::user::UserDto;
use crate::errors::api::forgotten_password::ForgottenPasswordError;
use crate::errors::api::login::LoginError;
use crate::errors::api::reset_password::ResetPasswordError;
use crate::provider::email::mailjet::MailjetClient;

#[derive(serde::Deserialize)]
pub struct LoginData {
    login: String,
    password: Secret<String>,
}

#[tracing::instrument(name = "Login request", skip(request_data, config))]
#[post("/login")]
pub async fn login(
    request_data: web::Json<LoginData>,
    config: web::Data<Settings>,
    db_pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> Result<HttpResponse, LoginError> {
    let authentication_service = AuthenticationService::new(config.auth.clone());
    let user_service = UserService::new(&db_pool);

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

#[derive(serde::Deserialize)]
pub struct ForgottenPasswordData {
    email: String,
}

#[tracing::instrument(name = "Reset password", skip(request_data, config, tera))]
#[post("/password-reset")]
pub async fn forgotten_password(
    request_data: web::Json<ForgottenPasswordData>,
    config: web::Data<Settings>,
    tera: web::Data<Tera>,
    db_pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> Result<HttpResponse, ForgottenPasswordError> {
    let token_service = TokenService::new(&config, &db_pool);
    let user_service = UserService::new(&db_pool);
    let email_client = MailjetClient::new(config.email.clone());

    let user_email =
        UserEmail::parse(request_data.0.email).map_err(ForgottenPasswordError::ValidationError)?;
    let user = match user_service.get_user_from_username(user_email.as_ref()) {
        Ok(u) => u,
        Err(_) => {
            tracing::info!("Unable to found user with email {}", user_email.as_ref());
            return Ok(HttpResponse::NoContent().finish());
        }
    };

    let token = token_service
        .generate_token_for_user(&user)
        .context("An error occurred during token generation.")
        .map_err(ForgottenPasswordError::UnexpectedError)?;

    let mut context = tera::Context::new();
    context.insert(
        "reset_link",
        &format!("{}/password-reset/{}", config.host, token.id),
    );

    email_client.send_email(
        &user_email,
        "Demande de réinitialisation de mot de passe".into(),
        &tera.render("mail/reset_password.html", &context).unwrap(),
        "Une demande de réinitialisation de mot de passe a été effectuée. Cliquez sur ce lien pour réinitialiser votre mot de passe.".into(),
    ).await.context("An error occurred during email sending.").map_err(ForgottenPasswordError::UnexpectedError)?;

    Ok(HttpResponse::NoContent().finish())
}

#[derive(serde::Deserialize)]
pub struct ResetPasswordData {
    password: Secret<String>,
}

#[tracing::instrument(name = "Reset password", skip(request_data, config))]
#[put("/password-reset/{token}")]
pub async fn reset_password(
    token: web::Path<String>,
    request_data: web::Json<ResetPasswordData>,
    config: web::Data<Settings>,
    db_pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> Result<HttpResponse, ResetPasswordError> {
    let mut token_service = TokenService::new(&config, &db_pool);
    let token_id = Uuid::parse_str(token.as_str())
        .context("Invalid token id")
        .map_err(ResetPasswordError::InvalidToken)?;

    token_service
        .reset_user_password(token_id, &request_data.0.password)
        .context("Unable to reset user password")
        .map_err(ResetPasswordError::InvalidToken)?;

    Ok(HttpResponse::NoContent().finish())
}

#[get("/profile")]
pub async fn get_profile(user: User) -> HttpResponse {
    HttpResponse::Ok().json(UserDto::from_user(user))
}
