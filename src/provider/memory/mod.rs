use std::collections::HashMap;

use secrecy::Secret;
use uuid::Uuid;

use crate::domain::model::token::{Token, TokenType};
use crate::domain::model::user::User;

pub mod token;
pub mod user;

pub struct MemoryStorage {
    users: HashMap<Uuid, User>,
    tokens: HashMap<Uuid, Token>,
}

impl MemoryStorage {
    pub fn new() -> Self {
        let user_id_1 = Uuid::new_v4();
        let user_id_2 = Uuid::new_v4();
        let token_id_1 = Uuid::parse_str("bbfddad7-940e-4a85-a35d-925c91b438bd").unwrap();

        let mut users = HashMap::new();

        users.insert(user_id_1, User {
            id: user_id_1,
            username: "test@email.com".into(),
            password: Secret::new("$argon2id$v=19$m=4096,t=3,p=1$njFuptpoWRAhlPsfwK5IYA$eJJGq0hW6/IMDKsVal7mxSK/YlOvI12JyxcETS5cYPQ".into()),
        });
        users.insert(user_id_2, User {
            id: user_id_2,
            username: "x7iv7vqe2@mozmail.com".into(),
            password: Secret::new("$argon2id$v=19$m=4096,t=3,p=1$/rG29LlMB72wf9+m71cauQ$WL0HiAvPMMJooHzxuwweoINv+Rv8lU5yT8XOHWOrH5E".into()),
        });

        let mut tokens = HashMap::new();

        tokens.insert(
            token_id_1,
            Token {
                id: Uuid::new_v4(),
                token_type: TokenType::ResetPassword,
                user_id: user_id_1,
            },
        );

        Self { users, tokens }
    }
}
