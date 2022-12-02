pub mod cards;
pub mod sets;

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
