use std::sync::Mutex;

use uuid::Uuid;

use crate::domain::model::token::Token;
use crate::provider::memory::MemoryStorage;
use crate::provider::token::TokenProvider;

pub struct MemoryTokenProvider<'a> {
    storage: &'a Mutex<MemoryStorage>,
}

impl<'a> MemoryTokenProvider<'a> {
    pub fn new(storage: &'a Mutex<MemoryStorage>) -> Self {
        Self { storage }
    }
}

impl<'a> TokenProvider for MemoryTokenProvider<'a> {
    fn save_token(&self, token: Token) {
        self.storage.lock().unwrap().tokens.insert(token.id, token);
    }

    fn find_token_by_id(&self, token_id: Uuid) -> Option<Token> {
        self.storage
            .lock()
            .unwrap()
            .tokens
            .iter()
            .find_map(|(stored_id, token)| {
                if stored_id == &token_id {
                    return Some(token.clone());
                }

                None
            })
    }
}
