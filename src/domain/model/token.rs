use uuid::Uuid;

#[derive(Clone, Debug, sqlx::Type)]
#[sqlx(type_name = "token_type")]
#[sqlx(rename_all = "snake_case")]
pub enum TokenType {
    ResetPassword,
}

#[derive(Clone)]
pub struct Token {
    pub id: Uuid,
    pub token_type: TokenType,
    pub user_id: Uuid,
}
