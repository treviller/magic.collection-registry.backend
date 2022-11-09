use actix_web::{get, post, web, HttpResponse};
use anyhow::Context;
use secrecy::Secret;

use crate::authentication::AuthenticationService;
use crate::configuration::settings::Settings;
use crate::domain::model::user::User;
use crate::domain::user::UserService;
use crate::dto::user::UserDto;
use crate::errors::api::forgotten_password::ForgottenPasswordError;
use crate::errors::api::login::LoginError;
use crate::provider::email::mailjet::MailjetClient;

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

#[derive(serde::Deserialize)]
pub struct ForgottenPasswordData {
    email: String,
}

#[tracing::instrument(name = "Reset password", skip(request_data, config))]
#[post("/api/password-reset")]
pub async fn forgotten_password(
    request_data: web::Json<ForgottenPasswordData>,
    config: web::Data<Settings>,
) -> Result<HttpResponse, ForgottenPasswordError> {
    let email_client = MailjetClient::new(config.email.clone());
    let html_content = r#"<p><span>Une demande de réinitialisation de mot de passe a été effectuée.</span><span>Cliquez sur ce <a href="" target="_blank">lien</a> pour réinitialiser votre mot de passe.</p>"#;

    email_client.send_email(
        request_data.0.email.as_str(),
        "Demande de réinitialisation de mot de passe".into(),
        html_content,
        "Une demande de réinitialisation de mot de passe a été effectuée. Cliquez sur ce lien pour réinitialiser votre mot de passe.".into(),
    ).await.context("An error occurred during email sending.").map_err(ForgottenPasswordError::UnexpectedError)?;

    Ok(HttpResponse::Ok().finish())
}

#[get("/api/profile")]
pub async fn get_profile(user: User) -> HttpResponse {
    HttpResponse::Ok().json(UserDto::from_user(user))
}
