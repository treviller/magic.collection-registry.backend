use secrecy::{ExposeSecret, Secret};

use crate::dto::user::UserDto;
use crate::routes::responses::AuthenticatedMeta;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct LoginResponse {
    pub meta: AuthenticatedMeta,
    pub data: UserDto,
}

impl LoginResponse {
    pub fn new(access_token: Secret<String>, refresh_token: String, user: UserDto) -> Self {
        Self {
            meta: AuthenticatedMeta {
                success: true,
                access_token: access_token.expose_secret().into(),
                refresh_token,
            },
            data: user,
        }
    }
}
