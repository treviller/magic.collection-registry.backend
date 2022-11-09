use std::sync::Mutex;

use uuid::Uuid;

use crate::domain::model::user::User;
use crate::provider::memory::MemoryStorage;
use crate::provider::user::UserProvider;

pub struct UserMemoryProvider<'a> {
    storage: &'a Mutex<MemoryStorage>,
}

impl<'a> UserMemoryProvider<'a> {
    pub fn new(storage: &'a Mutex<MemoryStorage>) -> Self {
        Self { storage }
    }
}

impl<'a> UserProvider for UserMemoryProvider<'a> {
    fn find_one_by_username(&self, username: &str) -> Option<User> {
        self.storage
            .lock()
            .unwrap()
            .users
            .iter()
            .find_map(|(_id, user)| {
                if user.username == username {
                    return Some(user.clone());
                }

                None
            })
    }

    fn find_one_by_id(&self, user_id: Uuid) -> Option<User> {
        self.storage
            .lock()
            .unwrap()
            .users
            .iter()
            .find_map(|(id, user)| {
                if id == &user_id {
                    return Some(user.clone());
                }

                None
            })
    }

    fn update_user(&mut self, updated_user: User) {
        if let Some(user) = self.storage.lock().unwrap().users.get_mut(&updated_user.id) {
            *user = updated_user;
        }
    }
}
