use crate::dto::card::CardDto;
use crate::routes::responses::ListSuccessMeta;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CardsListResponse {
    meta: ListSuccessMeta,
    data: Vec<CardDto>,
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
