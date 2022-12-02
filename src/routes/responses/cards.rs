use crate::dto::card::CardDto;
use crate::routes::responses::PaginatedListSuccessMeta;
use crate::routes::PageId;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CardsListResponse {
    pub meta: PaginatedListSuccessMeta,
    pub data: Vec<CardDto>,
}

impl CardsListResponse {
    pub fn new(cards: Vec<CardDto>, current_page: PageId) -> Self {
        Self {
            meta: PaginatedListSuccessMeta {
                success: true,
                total: cards.len(),
                previous_page: current_page.get_previous_page(),
                next_page: current_page.get_next_page(),
            },
            data: cards,
        }
    }
}
