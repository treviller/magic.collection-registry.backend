use secrecy::Secret;

use crate::domain::model::user::User;
use crate::provider::user::UserProvider;

pub struct UserMemoryProvider {
    users: Vec<User>,
}

impl UserMemoryProvider {
    pub fn new() -> Self {
        Self {
            users: vec![
                User {
                    username: "user1".into(),
                    password: Secret::new("test".into()),
                },
                User {
                    username: "user2".into(),
                    password: Secret::new("toto".into()),
                },
            ],
        }
    }
}

impl UserProvider for UserMemoryProvider {
    fn find_one_by_username(&self, username: &str) -> Option<User> {
        self.users.iter().find_map(|user| {
            if user.username == username {
                return Some(user.clone());
            }

            None
        })
    }
}
