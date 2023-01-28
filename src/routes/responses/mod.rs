pub mod authentication;
pub mod cards;
pub mod sets;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AuthenticatedMeta {
    pub success: bool,
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ListSuccessMeta {
    pub total: usize,
    pub success: bool,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PaginatedListSuccessMeta {
    pub total: usize,
    pub success: bool,
    pub next_page: Option<String>,
    pub previous_page: Option<String>,
}
