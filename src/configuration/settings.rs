use secrecy::Secret;

#[derive(serde::Deserialize, Clone)]
pub struct Settings {
    pub auth: AuthSettings,
}

#[derive(serde::Deserialize, Clone)]
pub struct AuthSettings {
    pub jwt_key: Secret<String>,
    pub jwt_ttl: u64,
}
