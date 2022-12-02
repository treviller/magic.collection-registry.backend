pub mod cards;
pub mod sets;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ListSuccessMeta {
    pub total: usize,
    pub success: bool,
}
