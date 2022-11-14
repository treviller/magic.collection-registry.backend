use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::Pool;
use secrecy::Secret;
use uuid::Uuid;

use crate::domain::model::user::User;
use crate::errors::domain::DomainError;
use crate::provider::database::user::DbUserProvider;
use crate::provider::user::UserProvider;

pub struct UserService<'a> {
    user_provider: DbUserProvider<'a>,
}

impl<'a> UserService<'a> {
    pub fn new(db_pool: &'a Pool<ConnectionManager<PgConnection>>) -> Self {
        Self {
            user_provider: DbUserProvider::new(db_pool),
        }
    }

    pub fn get_user_from_username(&self, username: &str) -> Result<User, DomainError> {
        match self.user_provider.find_one_by_username(username) {
            Some(user) => Ok(user),
            None => Err(DomainError("No user found.".into())),
        }
    }

    pub fn get_user_from_id(&self, user_id: Uuid) -> Result<User, DomainError> {
        match self.user_provider.find_one_by_id(user_id) {
            Some(user) => Ok(user),
            None => Err(DomainError("No user found.".into())),
        }
    }

    pub fn update_user_password(&mut self, user: &User, password: Secret<String>) {
        let mut updated_user = user.clone();

        updated_user.password = password;

        self.user_provider.update_user(updated_user);
    }
}
