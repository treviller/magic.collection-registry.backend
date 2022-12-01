use crate::provider::scryfall::set::ScryfallSet;

#[derive(serde::Deserialize)]
pub struct ScryfallSetListResponse {
    pub object: String,
    pub has_more: bool,
    pub data: Vec<ScryfallSet>,
}
