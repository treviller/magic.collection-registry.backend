use uuid::Uuid;

use crate::domain::model::card::Card;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CardDto {
    pub id: Uuid,
    pub name: String,
    pub language: String,
}

impl From<Card> for CardDto {
    fn from(card: Card) -> Self {
        Self {
            id: card.id,
            name: card.name,
            language: card.lang,
        }
    }
}
