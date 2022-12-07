use crate::domain::model::card::{Card, CardRarity};
use crate::errors::domain::DomainError;
use crate::provider::card::{CardFilterParameters, CardProvider};
use crate::provider::database::card::DbCardProvider;
use crate::provider::database::DbConnection;
use crate::routes::cards::QueryParameters;
use crate::routes::Pagination;

pub struct CardService<'a> {
    card_provider: DbCardProvider<'a>,
}

impl<'a> CardService<'a> {
    pub fn new(db_pool: &'a DbConnection) -> Self {
        Self {
            card_provider: DbCardProvider::new(db_pool),
        }
    }

    pub fn add_cards(&self, cards: Vec<Card>) {
        self.card_provider.insert_cards(cards);
    }

    pub fn list_cards(
        &self,
        filters: CardFilterParameters,
        pagination: &Pagination,
    ) -> Result<Vec<Card>, DomainError> {
        Ok(self.card_provider.get_cards(filters, pagination).unwrap())
    }
}
