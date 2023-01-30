use crate::domain::model::card::Card;
use crate::dto::card::CardDto;
use crate::provider::database::PaginatedResult;
use crate::routes::responses::PaginatedListSuccessMeta;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CardsListResponse {
    pub meta: PaginatedListSuccessMeta,
    pub data: Vec<CardDto>,
}

impl CardsListResponse {
    pub fn new(paginated_cards: PaginatedResult<Card>) -> Self {
        Self {
            meta: PaginatedListSuccessMeta {
                success: true,
                total: paginated_cards.total_elements,
                previous_page: paginated_cards.get_previous_page_index(),
                next_page: paginated_cards.get_next_page_index(),
            },
            data: paginated_cards
                .elements
                .into_iter()
                .map(|card| card.into())
                .collect(),
        }
    }
}
