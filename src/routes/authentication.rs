use actix_web::{get, post, web, HttpResponse};
use secrecy::Secret;

use crate::authentication::AuthenticationService;
use crate::configuration::settings::Settings;
use crate::domain::model::user::User;
use crate::domain::user::UserService;
use crate::dto::user::UserDto;

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
) -> HttpResponse {
    let authentication_service = AuthenticationService::new(config.jwt_key.clone(), config.jwt_ttl);
    let user_service = UserService::new();

    let user = user_service
        .get_user_from_username(&request_data.0.login)
        .expect("Unable to found user with username");

    authentication_service.check_credentials(&user, request_data.0.password);

    HttpResponse::Ok().json(authentication_service.generate_jwt(user))
}

#[get("/api/profile")]
pub async fn get_profile(user: User) -> HttpResponse {
    HttpResponse::Ok().json(UserDto::from_user(user))
}
