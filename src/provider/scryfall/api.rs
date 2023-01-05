#[derive(serde::Deserialize)]
pub struct ScryfallListResponse<T> {
    pub object: String,
    pub has_more: bool,
    pub data: Vec<T>,
}
