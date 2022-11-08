use crate::domain::model::user::User;
use crate::errors::domain::DomainError;
use crate::provider::memory::user::UserMemoryProvider;
use crate::provider::user::UserProvider;

pub struct UserService {
    user_provider: UserMemoryProvider,
}

impl UserService {
    pub fn new() -> Self {
        Self {
            user_provider: UserMemoryProvider::new(),
        }
    }

    pub fn get_user_from_username(&self, username: &str) -> Result<User, DomainError> {
        match self.user_provider.find_one_by_username(username) {
            Some(user) => Ok(user),
            None => Err(DomainError("No user found.".into())),
        }
    }
}
