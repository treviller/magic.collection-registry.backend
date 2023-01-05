use sqlx::Postgres;
use uuid::Uuid;

use crate::domain::model::token::{Token, TokenType};
use crate::provider::database::DbConnection;

#[derive(sqlx::FromRow)]
pub struct DbToken {
    pub id: Uuid,
    pub token_type: TokenType,
    pub user_id: Uuid,
}

impl From<Token> for DbToken {
    fn from(token: Token) -> Self {
        Self {
            id: token.id,
            token_type: token.token_type,
            user_id: token.user_id,
        }
    }
}

impl Into<Token> for DbToken {
    fn into(self) -> Token {
        Token {
            id: self.id,
            token_type: self.token_type,
            user_id: self.user_id,
        }
    }
}

pub async fn save_token(db_pool: &DbConnection, token: Token) {
    let token: DbToken = token.into();

    sqlx::query("INSERT INTO tokens (id, token_type, user_id) VALUES ($1, $2, $3)")
        .bind(token.id)
        .bind(token.token_type)
        .bind(token.user_id)
        .execute(db_pool)
        .await
        .expect("No error okay");
}

pub async fn find_token_by_id(db_pool: &DbConnection, searched_id: Uuid) -> Option<Token> {
    let result: Option<DbToken> =
        sqlx::query_as::<Postgres, DbToken>("SELECT * FROM tokens WHERE id = $1 LIMIT 1")
            .bind(searched_id)
            .fetch_optional(db_pool)
            .await
            .expect("No error okay");

    // TODO handle errors
    match result {
        Some(token) => Some(token.into()),
        None => None,
    }
}
