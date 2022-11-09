use anyhow::Context;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use jsonwebtoken::{
    decode, encode, get_current_timestamp, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use secrecy::{ExposeSecret, Secret};
use serde::Serializer;

use crate::configuration::settings::AuthSettings;
use crate::domain::model::user::User;
use crate::errors::auth::AuthError;
use crate::errors::jwt::JwtError;

#[derive(serde::Serialize)]
pub struct AuthTokens {
    #[serde(serialize_with = "serialize_jwt")]
    pub access_token: Secret<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Claims {
    pub sub: String,
    pub iat: u64,
    pub exp: u64,
}

fn serialize_jwt<S>(value: &Secret<String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(value.expose_secret())
}

pub struct AuthenticationService {
    jwt_key: Secret<String>,
    jwt_ttl: u64,
}

impl AuthenticationService {
    pub fn new(settings: AuthSettings) -> Self {
        Self {
            jwt_key: settings.jwt_key,
            jwt_ttl: settings.jwt_ttl,
        }
    }

    pub fn generate_jwt(&self, user: User) -> Result<AuthTokens, AuthError> {
        let encoding_key = EncodingKey::from_secret(self.jwt_key.expose_secret().as_ref());
        let current_timestamp = get_current_timestamp();
        let claims = Claims {
            sub: user.username,
            iat: current_timestamp,
            exp: current_timestamp + self.jwt_ttl,
        };

        let jwt = encode(&Header::new(Algorithm::HS256), &claims, &encoding_key)
            .context("Failed to encode JWT.")
            .map_err(AuthError::UnexpectedError)?;

        Ok(AuthTokens {
            access_token: Secret::new(jwt),
        })
    }

    pub fn decode_jwt(&self, token: &str) -> Result<Claims, JwtError> {
        let decoding_key = DecodingKey::from_secret(self.jwt_key.expose_secret().as_ref());
        let token_data = decode::<Claims>(token, &decoding_key, &Validation::new(Algorithm::HS256))
            .context("Failed to decode JWT")
            .map_err(JwtError::InvalidToken)?;

        Ok(token_data.claims)
    }

    pub fn hash_password(&self, password: &Secret<String>) -> Secret<String> {
        let salt = SaltString::generate(&mut rand::thread_rng());
        let password_hash = Argon2::default()
            .hash_password(password.expose_secret().as_bytes(), &salt)
            .unwrap()
            .to_string();

        Secret::new(password_hash)
    }

    pub fn check_credentials(
        &self,
        user: &User,
        password: Secret<String>,
    ) -> Result<(), AuthError> {
        let expected_password_hash = PasswordHash::new(&user.password.expose_secret())
            .context("Failed to parse hash in PHC string format")
            .map_err(AuthError::UnexpectedError)?;

        Argon2::default()
            .verify_password(password.expose_secret().as_bytes(), &expected_password_hash)
            .context("Invalid password.")
            .map_err(AuthError::InvalidCredentials)
    }
}
