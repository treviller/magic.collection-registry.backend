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
                    password: Secret::new("$argon2id$v=19$m=4096,t=3,p=1$njFuptpoWRAhlPsfwK5IYA$eJJGq0hW6/IMDKsVal7mxSK/YlOvI12JyxcETS5cYPQ".into()),
                },
                User {
                    username: "user2".into(),
                    password: Secret::new("$argon2id$v=19$m=4096,t=3,p=1$/rG29LlMB72wf9+m71cauQ$WL0HiAvPMMJooHzxuwweoINv+Rv8lU5yT8XOHWOrH5E".into()),
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
