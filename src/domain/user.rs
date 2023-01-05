use secrecy::Secret;
use uuid::Uuid;

use crate::domain::model::user::User;
use crate::errors::domain::DomainError;
use crate::provider::database;
use crate::provider::database::DbConnection;

pub struct UserService<'a> {
    db_pool: &'a DbConnection,
}

impl<'a> UserService<'a> {
    pub fn new(db_pool: &'a DbConnection) -> Self {
        Self { db_pool }
    }

    pub async fn get_user_from_username(&self, username: &str) -> Result<User, DomainError> {
        match database::user::find_one_by_username(self.db_pool, username).await {
            Some(user) => Ok(user),
            None => Err(DomainError("No user found.".into())),
        }
    }

    pub async fn get_user_from_id(&self, user_id: Uuid) -> Result<User, DomainError> {
        match database::user::find_one_by_id(self.db_pool, user_id).await {
            Some(user) => Ok(user),
            None => Err(DomainError("No user found.".into())),
        }
    }

    pub async fn update_user_password(&mut self, user: &User, password: Secret<String>) {
        database::user::update_user_password(self.db_pool, user.id, password).await;
    }
}
