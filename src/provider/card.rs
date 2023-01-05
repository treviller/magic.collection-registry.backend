use crate::domain::model::card::CardRarity;

pub struct CardFilterParameters {
    pub name: Option<String>,
    pub language: Option<String>,
    pub rarity: Option<CardRarity>,
}
