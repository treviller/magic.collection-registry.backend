use uuid::Uuid;

#[derive(Clone)]
pub enum TokenType {
    ResetPassword,
}

#[derive(Clone)]
pub struct Token {
    pub id: Uuid,
    pub token_type: TokenType,
    pub user_id: Uuid,
}
