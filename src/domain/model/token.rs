use uuid::Uuid;

#[derive(Clone)]
pub enum TokenType {
    ResetPassword,
}

impl From<String> for TokenType {
    fn from(_: String) -> Self {
        TokenType::ResetPassword
    }
}

impl Into<String> for TokenType {
    fn into(self) -> String {
        "reset_password".into()
    }
}

#[derive(Clone)]
pub struct Token {
    pub id: Uuid,
    pub token_type: TokenType,
    pub user_id: Uuid,
}
