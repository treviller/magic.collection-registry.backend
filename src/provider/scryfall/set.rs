use chrono::NaiveDate;
use uuid::Uuid;

use crate::domain::model::set::{Set, SetCode, SetType};

#[derive(serde::Deserialize)]
pub struct ScryfallSet {
    pub id: Uuid,
    pub code: String,
    pub mtgo_code: Option<String>,
    pub tcgplayer_id: Option<i32>,
    pub name: String,
    pub set_type: SetType,
    pub released_at: NaiveDate,
    pub block_code: Option<String>,
    pub block: Option<String>,
    pub parent_set_code: Option<String>,
    pub card_count: i32,
    pub printed_size: Option<i32>,
    pub digital: bool,
    pub foil_only: bool,
    pub nonfoil_only: bool,
    pub scryfall_uri: String,
    pub uri: String,
    pub icon_svg_uri: String,
    pub search_uri: String,
}

impl ScryfallSet {
    pub fn into_set(self) -> Result<Set, String> {
        let printed_size = match self.printed_size {
            Some(size) => size as u16,
            None => 0,
        };

        Ok(Set {
            id: self.id,
            code: SetCode::parse(self.code)?,
            name: self.name,
            set_type: self.set_type,
            released_at: self.released_at,
            block_code: self.block_code,
            block: self.block,
            parent_set_id: None, //TODO
            card_count: self.card_count as u16,
            printed_size,
            foil_only: self.foil_only,
            non_foil_only: self.nonfoil_only,
            icon_svg_uri: self.icon_svg_uri,
        })
    }
}
