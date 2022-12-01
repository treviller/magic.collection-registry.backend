use crate::domain::model::set::Set;
use crate::errors::domain::DomainError;
use crate::provider::database::set::DbSetProvider;
use crate::provider::database::DbConnection;
use crate::provider::set::SetProvider;

pub struct SetService<'a> {
    set_provider: DbSetProvider<'a>,
}

impl<'a> SetService<'a> {
    pub fn new(db_pool: &'a DbConnection) -> Self {
        Self {
            set_provider: DbSetProvider::new(db_pool),
        }
    }

    pub fn get_sets_list(&self) -> Result<Vec<Set>, DomainError> {
        Ok(self.set_provider.get_all_sets().unwrap())
    }
}
