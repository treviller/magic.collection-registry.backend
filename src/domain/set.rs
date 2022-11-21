use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::Pool;

use crate::domain::model::set::Set;
use crate::errors::domain::DomainError;
use crate::provider::database::set::DbSetProvider;
use crate::provider::set::SetProvider;

pub struct SetService<'a> {
    set_provider: DbSetProvider<'a>,
}

impl<'a> SetService<'a> {
    pub fn new(db_pool: &'a Pool<ConnectionManager<PgConnection>>) -> Self {
        Self {
            set_provider: DbSetProvider::new(db_pool),
        }
    }

    pub fn get_sets_list(&self) -> Result<Vec<Set>, DomainError> {
        Ok(self.set_provider.get_all_sets().unwrap())
    }
}
