use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::domain::model::set::SetType;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SetDto {
    id: Uuid,
    code: String,
    set_type: SetType,
    release_at: DateTime<Utc>,
    card_count: u16,
    icon_svg_uri: String,
}
