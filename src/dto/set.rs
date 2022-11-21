use chrono::NaiveDate;
use uuid::Uuid;

use crate::domain::model::set::{Set, SetType};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SetDto {
    id: Uuid,
    code: String,
    set_type: SetType,
    release_at: NaiveDate,
    card_count: u16,
    icon_svg_uri: String,
}

impl SetDto {
    pub fn new(set: Set) -> Self {
        Self {
            id: set.id,
            code: set.code.into(),
            set_type: set.set_type,
            release_at: set.released_at,
            card_count: set.card_count,
            icon_svg_uri: set.icon_svg_uri,
        }
    }
}
