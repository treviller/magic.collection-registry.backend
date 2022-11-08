use jsonwebtoken::{
    decode, encode, get_current_timestamp, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use secrecy::{ExposeSecret, Secret};
use serde::Serializer;

use crate::domain::model::user::User;

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
    pub fn new(jwt_key: Secret<String>, jwt_ttl: u64) -> Self {
        Self { jwt_key, jwt_ttl }
    }

    pub fn generate_jwt(&self, user: User) -> AuthTokens {
        let encoding_key = EncodingKey::from_secret(self.jwt_key.expose_secret().as_ref());
        let current_timestamp = get_current_timestamp();
        let claims = Claims {
            sub: user.username,
            iat: current_timestamp,
            exp: current_timestamp + self.jwt_ttl,
        };

        let jwt = encode(&Header::new(Algorithm::HS256), &claims, &encoding_key)
            .expect("An error occurred during JWT encoding");

        AuthTokens {
            access_token: Secret::new(jwt),
        }
    }

    pub fn decode_jwt(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let decoding_key = DecodingKey::from_secret(self.jwt_key.expose_secret().as_ref());
        let token_data =
            decode::<Claims>(token, &decoding_key, &Validation::new(Algorithm::HS256))?;

        Ok(token_data.claims)
    }

    pub fn check_credentials(&self, user: &User, _password: Secret<String>) -> bool {
        true
    }
}
