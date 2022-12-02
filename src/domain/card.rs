use crate::domain::model::card::Card;
use crate::errors::domain::DomainError;
use crate::provider::card::CardProvider;
use crate::provider::database::card::DbCardProvider;
use crate::provider::database::DbConnection;

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

    pub fn list_cards(&self) -> Result<Vec<Card>, DomainError> {
        Ok(self.card_provider.get_cards().unwrap())
    }
}
