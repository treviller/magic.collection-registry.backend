use std::sync::Mutex;

use secrecy::Secret;
use uuid::Uuid;

use crate::domain::model::user::User;
use crate::errors::domain::DomainError;
use crate::provider::memory::user::UserMemoryProvider;
use crate::provider::memory::MemoryStorage;
use crate::provider::user::UserProvider;

pub struct UserService<'a> {
    user_provider: UserMemoryProvider<'a>,
}

impl<'a> UserService<'a> {
    pub fn new(storage: &'a Mutex<MemoryStorage>) -> Self {
        Self {
            user_provider: UserMemoryProvider::new(storage),
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
