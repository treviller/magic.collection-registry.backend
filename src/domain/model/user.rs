use std::future::Future;
use std::pin::Pin;

use actix_web::dev::Payload;
use actix_web::http::header;
use actix_web::web::Data;
use actix_web::{FromRequest, HttpRequest};
use anyhow::{anyhow, Context};
use secrecy::Secret;

use crate::authentication::AuthenticationService;
use crate::configuration::settings::Settings;
use crate::domain::user::UserService;
use crate::errors::jwt::JwtError;

#[derive(Clone)]
pub struct User {
    pub username: String,
    pub password: Secret<String>,
}

impl FromRequest for User {
    type Error = JwtError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let req = req.clone();

        Box::pin(async move {
            let auth_settings = req
                .app_data::<Data<Settings>>()
                .map(|settings| settings.auth.clone())
                .unwrap();

            let authentication_service = AuthenticationService::new(auth_settings);
            let authorization_header_value = match req.headers().get(header::AUTHORIZATION) {
                Some(value) => value,
                None => {
                    return Err(JwtError::InvalidToken(anyhow!(
                        "No authorization header found."
                    )));
                }
            };

            let authorization_value = authorization_header_value
                .to_str()
                .context("Authorization header contains invalid characters")?;

            let jwt_claims = authentication_service
                .decode_jwt(authorization_value)
                .context("Failed to decode and validate the jwt")
                .map_err(JwtError::InvalidToken)?;

            let user_service = UserService::new();

            user_service
                .get_user_from_username(&jwt_claims.sub)
                .context("Failed to found user from jwt")
                .map_err(JwtError::InvalidToken)
        })
    }
}
