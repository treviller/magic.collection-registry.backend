use uuid::Uuid;

#[derive(Clone, Debug, diesel_derive_enum::DbEnum)]
#[DieselTypePath = "crate::schema::sql_types::TokenType"]
pub enum TokenType {
    ResetPassword,
}

#[derive(Clone)]
pub struct Token {
    pub id: Uuid,
    pub token_type: TokenType,
    pub user_id: Uuid,
}
