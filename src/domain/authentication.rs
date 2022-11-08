use jsonwebtoken::{
    decode, encode, get_current_timestamp, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use secrecy::{ExposeSecret, Secret};
use serde::Serializer;

use crate::domain::model::user::User;
use crate::errors::domain::DomainError;
use crate::provider::memory::user::UserMemoryProvider;
use crate::provider::user::UserProvider;

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
    user_provider: UserMemoryProvider,
    jwt_key: Secret<String>,
    jwt_ttl: u64,
}

impl AuthenticationService {
    pub fn new(user_provider: UserMemoryProvider, jwt_key: Secret<String>, jwt_ttl: u64) -> Self {
        Self {
            user_provider,
            jwt_key,
            jwt_ttl,
        }
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

    pub fn login_user(&self, username: String, password: Secret<String>) -> Result<User, String> {
        let user = match self.user_provider.find_one_by_username(username) {
            Some(user) => user,
            None => return Err("No user found.".into()),
        };

        self.check_credentials(&user, password);

        Ok(user)
    }

    pub fn check_credentials(&self, user: &User, _password: Secret<String>) -> bool {
        true
    }
}
