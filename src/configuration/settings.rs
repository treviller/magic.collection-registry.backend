use secrecy::Secret;

#[derive(serde::Deserialize, Clone)]
pub struct Settings {
    pub auth: AuthSettings,
    pub email: EmailSettings,
}

#[derive(serde::Deserialize, Clone)]
pub struct AuthSettings {
    pub jwt_key: Secret<String>,
    pub jwt_ttl: u64,
}

#[derive(serde::Deserialize, Clone)]
pub struct EmailSettings {
    pub base_url: String,
    pub sender_email: String,
    pub api_key: Secret<String>,
    pub secret_key: Secret<String>,
}
