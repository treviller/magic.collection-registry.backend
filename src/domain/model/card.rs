use chrono::NaiveDate;
use uuid::Uuid;

pub struct Card {
    pub id: Uuid,
    pub scryfall_id: String,
    pub name: String,
    pub lang: String,
    pub released_at: NaiveDate,
    pub set_id: Uuid,
}

impl Card {}
