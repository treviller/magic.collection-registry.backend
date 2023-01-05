use chrono::NaiveDate;
use strum_macros::EnumString;
use uuid::Uuid;

#[derive(Debug, PartialEq, Clone, EnumString, serde::Serialize, serde::Deserialize, sqlx::Type)]
#[sqlx(type_name = "card_rarity")]
#[sqlx(rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum CardRarity {
    Common,
    Uncommon,
    Rare,
    Special,
    Mythic,
    Bonus,
}

pub struct Card {
    pub id: Uuid,
    pub scryfall_id: String,
    pub name: String,
    pub lang: String,
    pub released_at: NaiveDate,
    pub set_id: Uuid,
    pub rarity: CardRarity,
}

impl Card {}
