use crate::domain::model::card::Card;
use crate::errors::domain::DomainError;
use crate::provider::card::CardFilterParameters;
use crate::provider::database;
use crate::provider::database::{DbConnection, PaginatedResult};
use crate::routes::PaginationParameters;

pub struct CardService<'a> {
    db_pool: &'a DbConnection,
}

impl<'a> CardService<'a> {
    pub fn new(db_pool: &'a DbConnection) -> Self {
        Self { db_pool }
    }

    pub async fn add_cards(&self, cards: Vec<Card>) {
        database::card::insert_cards(self.db_pool, cards).await;
    }

    pub async fn list_cards(
        &self,
        filters: CardFilterParameters,
        pagination: PaginationParameters,
    ) -> Result<PaginatedResult<Card>, DomainError> {
        Ok(database::card::get_cards(self.db_pool, &filters, pagination).await)
    }
}
