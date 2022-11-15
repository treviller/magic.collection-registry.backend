use crate::dto::set::SetDto;

use super::ListSuccessMeta;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SetListResponse {
    meta: ListSuccessMeta,
    data: Vec<SetDto>,
}

impl SetListResponse {
    pub fn new(sets: Vec<SetDto>) -> Self {
        Self {
            meta: ListSuccessMeta {
                success: true,
                total: sets.len(),
            },
            data: sets,
        }
    }
}
