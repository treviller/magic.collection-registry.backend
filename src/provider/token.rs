use uuid::Uuid;

use crate::domain::model::token::Token;

pub trait TokenProvider {
    fn save_token(&self, token: Token);
    fn find_token_by_id(&self, id: Uuid) -> Option<Token>;
}
