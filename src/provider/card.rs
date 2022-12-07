use crate::domain::model::card::{Card, CardRarity};
use crate::routes::Pagination;

pub struct CardFilterParameters {
    pub name: Option<String>,
    pub language: Option<String>,
    pub rarity: Option<CardRarity>,
}

pub trait CardProvider {
    fn insert_cards(&self, cards_list: Vec<Card>);

    fn get_cards(
        &self,
        filters: CardFilterParameters,
        pagination: &Pagination,
    ) -> Result<Vec<Card>, diesel::result::Error>;
}
