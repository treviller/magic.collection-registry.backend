pub mod cards;
pub mod sets;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ListSuccessMeta {
    total: usize,
    success: bool,
}
