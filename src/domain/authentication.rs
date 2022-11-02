use jsonwebtoken::{encode, get_current_timestamp, Algorithm, EncodingKey, Header};
use secrecy::{ExposeSecret, Secret};
use serde::Serializer;

#[derive(serde::Serialize)]
pub struct AuthTokens {
    #[serde(serialize_with = "serialize_jwt")]
    access_token: Secret<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Claims {
    sub: String,
    iat: u64,
    exp: u64,
}

fn serialize_jwt<S>(value: &Secret<String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(value.expose_secret())
}

pub fn check_credentials(_password: Secret<String>) -> bool {
    true
}

pub fn generate_jwt() -> AuthTokens {
    let encoding_key = EncodingKey::from_secret("testkey".as_ref());
    let current_timestamp = get_current_timestamp();
    let claims = Claims {
        sub: "123456789".into(),
        iat: current_timestamp,
        exp: current_timestamp + 3600,
    };

    let jwt = encode(&Header::new(Algorithm::HS256), &claims, &encoding_key)
        .expect("An error occurred during JWT encoding");

    AuthTokens {
        access_token: Secret::new(jwt),
    }
}
