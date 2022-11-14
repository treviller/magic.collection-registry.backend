use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::{
    insert_into, AsChangeset, Identifiable, Insertable, QueryResult, Queryable, RunQueryDsl,
};
use r2d2::Pool;
use uuid::Uuid;

use crate::domain::model::token::{Token, TokenType};
use crate::provider::token::TokenProvider;
use crate::schema::tokens;
use crate::schema::tokens::dsl::*;

#[derive(Queryable, Identifiable, AsChangeset, Insertable)]
#[diesel(table_name = tokens)]
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

pub struct DbTokenProvider<'a> {
    db_pool: &'a Pool<ConnectionManager<PgConnection>>,
}

impl<'a> DbTokenProvider<'a> {
    pub fn new(db_pool: &'a Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { db_pool }
    }
}

impl<'a> TokenProvider for DbTokenProvider<'a> {
    fn save_token(&self, token: Token) {
        let mut connection = self.db_pool.get().unwrap();
        let token: DbToken = token.into();

        //TODO handle errors
        let _ = insert_into(tokens).values(&token).execute(&mut connection);
    }

    fn find_token_by_id(&self, searched_id: Uuid) -> Option<Token> {
        let mut connection = self.db_pool.get().unwrap();

        let result: QueryResult<DbToken> = tokens
            .filter(id.eq(searched_id))
            .limit(1)
            .get_result::<DbToken>(&mut connection);

        // TODO handle errors
        match result {
            Ok(token) => Some(token.into()),
            Err(_) => None,
        }
    }
}
