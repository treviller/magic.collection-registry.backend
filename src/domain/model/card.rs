use chrono::NaiveDate;
use strum_macros::EnumString;
use uuid::Uuid;

#[derive(
    Debug,
    PartialEq,
    Clone,
    EnumString,
    serde::Serialize,
    serde::Deserialize,
    diesel_derive_enum::DbEnum,
)]
#[DieselTypePath = "crate::schema::sql_types::CardRarity"]
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
