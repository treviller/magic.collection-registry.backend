use crate::domain::model::set::Set;
use crate::errors::domain::DomainError;
use crate::provider::database;
use crate::provider::database::DbConnection;

pub struct SetService<'a> {
    db_pool: &'a DbConnection,
}

impl<'a> SetService<'a> {
    pub fn new(db_pool: &'a DbConnection) -> Self {
        Self { db_pool }
    }

    pub async fn get_sets_list(&self) -> Result<Vec<Set>, DomainError> {
        Ok(database::set::get_all_sets(self.db_pool).await.unwrap())
    }

    pub async fn add_sets(&self, sets: Vec<Set>) {
        database::set::insert_sets(self.db_pool, sets).await;
    }
}
