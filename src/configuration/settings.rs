#[derive(serde::Deserialize, Clone)]
pub struct Settings {
    pub host: HostSettings,
    pub auth: AuthSettings,
    pub email: EmailSettings,
}

#[derive(serde::Deserialize, Clone)]
pub struct HostSettings {
    pub full: String,
    pub address: String,
    pub port: u16,
}

#[derive(serde::Deserialize, Clone)]
pub struct AuthSettings {
    pub jwt_ttl: u64,
}

#[derive(serde::Deserialize, Clone)]
pub struct EmailSettings {
    pub base_url: String,
    pub sender_email: String,
}
