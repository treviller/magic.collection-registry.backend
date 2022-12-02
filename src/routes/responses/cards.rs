use crate::dto::card::CardDto;
use crate::routes::responses::ListSuccessMeta;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CardsListResponse {
    pub meta: ListSuccessMeta,
    pub data: Vec<CardDto>,
}

impl CardsListResponse {
    pub fn new(cards: Vec<CardDto>) -> Self {
        Self {
            meta: ListSuccessMeta {
                success: true,
                total: cards.len(),
            },
            data: cards,
        }
    }
}
