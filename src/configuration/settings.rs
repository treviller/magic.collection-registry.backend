use secrecy::Secret;

#[derive(serde::Deserialize)]
pub struct Settings {
    pub jwt_key: Secret<String>,
}
