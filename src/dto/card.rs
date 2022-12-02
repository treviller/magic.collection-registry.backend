use uuid::Uuid;

use crate::domain::model::card::Card;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CardDto {
    id: Uuid,
    name: String,
}

impl From<Card> for CardDto {
    fn from(card: Card) -> Self {
        Self {
            id: card.id,
            name: card.name,
        }
    }
}
