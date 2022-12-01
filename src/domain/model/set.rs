use chrono::NaiveDate;
use strum_macros::EnumString;
use uuid::Uuid;

pub struct SetCode(String);

impl SetCode {
    pub fn parse(value: String) -> Result<Self, String> {
        if value.len() < 3 || value.len() > 6 {
            return Err(format!("{} is not a valid set code", value));
        }

        Ok(Self(value))
    }
}

impl Into<String> for SetCode {
    fn into(self) -> String {
        self.0
    }
}

#[derive(
    Clone, Debug, EnumString, serde::Serialize, serde::Deserialize, diesel_derive_enum::DbEnum,
)]
#[DieselTypePath = "crate::schema::sql_types::SetType"]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum SetType {
    Core,
    Expansion,
    Masters,
    Alchemy,
    Masterpiece,
    Arsenal,
    FromTheVault,
    Spellbook,
    PremiumDeck,
    DuelDeck,
    DraftInnovation,
    TreasureChest,
    Commander,
    Planechase,
    Archenemy,
    Vanguard,
    Funny,
    Starter,
    Box,
    Promo,
    Token,
    Memorabilia,
}

pub struct Set {
    pub id: Uuid,
    pub code: SetCode,
    pub name: String,
    pub set_type: SetType,
    pub released_at: NaiveDate,
    // Maybe keep only of those two values
    pub block_code: Option<String>,
    pub block: Option<String>,
    pub parent_set_id: Option<Uuid>,
    pub card_count: u16,
    // From my understood, this is the size of the set displayed on cards ( 0 / xxx )
    pub printed_size: u16,
    pub foil_only: bool,
    pub non_foil_only: bool,
    pub icon_svg_uri: String,
}
