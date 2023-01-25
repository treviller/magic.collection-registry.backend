use uuid::Uuid;

use crate::domain::model::card::{Card, CardRarity};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CardDto {
    pub id: Uuid,
    pub name: String,
    pub language: String,
    pub rarity: CardRarity,
    pub preview_image: Option<String>,
}

impl From<Card> for CardDto {
    fn from(card: Card) -> Self {
        Self {
            id: card.id,
            name: card.name,
            language: card.lang,
            rarity: card.rarity,
            preview_image: card.preview_image,
        }
    }
}
