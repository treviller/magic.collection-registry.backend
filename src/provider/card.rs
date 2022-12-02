use crate::domain::model::card::{Card, CardRarity};

pub trait CardProvider {
    fn insert_cards(&self, cards_list: Vec<Card>);

    fn get_cards(
        &self,
        language: Option<String>,
        name: Option<String>,
        rarity: Option<CardRarity>,
    ) -> Result<Vec<Card>, diesel::result::Error>;
}
