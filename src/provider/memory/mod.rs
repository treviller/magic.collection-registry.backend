use std::collections::HashMap;

use secrecy::Secret;
use uuid::Uuid;

use crate::domain::model::user::User;

pub mod user;

pub struct MemoryStorage {
    users: HashMap<Uuid, User>,
}

impl MemoryStorage {
    pub fn new() -> Self {
        let user_id_1 = Uuid::new_v4();
        let user_id_2 = Uuid::new_v4();

        let mut users = HashMap::new();

        users.insert(user_id_1, User {
            id: user_id_1,
            username: "user1".into(),
            password: Secret::new("$argon2id$v=19$m=4096,t=3,p=1$njFuptpoWRAhlPsfwK5IYA$eJJGq0hW6/IMDKsVal7mxSK/YlOvI12JyxcETS5cYPQ".into()),
        });
        users.insert(user_id_2, User {
            id: user_id_2,
            username: "user2".into(),
            password: Secret::new("$argon2id$v=19$m=4096,t=3,p=1$/rG29LlMB72wf9+m71cauQ$WL0HiAvPMMJooHzxuwweoINv+Rv8lU5yT8XOHWOrH5E".into()),
        });

        Self { users, tokens }
        Self { users }
    }
}
