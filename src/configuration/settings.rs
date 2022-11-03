use secrecy::Secret;

#[derive(serde::Deserialize, Clone)]
pub struct Settings {
    pub jwt_key: Secret<String>,
    pub jwt_ttl: u64,
}
