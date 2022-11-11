use diesel::prelude::*;
use diesel::{
    insert_into, AsChangeset, Identifiable, Insertable, QueryResult, Queryable, RunQueryDsl,
};
use uuid::Uuid;

use crate::domain::model::token::Token;
use crate::provider::database::establish_connection;
use crate::provider::token::TokenProvider;
use crate::schema::tokens;
use crate::schema::tokens::dsl::*;

#[derive(Queryable, Identifiable, AsChangeset, Insertable)]
#[diesel(table_name = tokens)]
pub struct DbToken {
    id: Uuid,
    // TODO use enum instead of String
    token_type: String,
    user_id: Uuid,
}

impl From<Token> for DbToken {
    fn from(token: Token) -> Self {
        Self {
            id: token.id,
            token_type: token.token_type.into(),
            user_id: token.user_id,
        }
    }
}

impl Into<Token> for DbToken {
    fn into(self) -> Token {
        Token {
            id: self.id,
            token_type: self.token_type.into(),
            user_id: self.user_id,
        }
    }
}

pub struct DbTokenProvider {}

impl TokenProvider for DbTokenProvider {
    fn save_token(&self, token: Token) {
        let mut connection = establish_connection();
        let token: DbToken = token.into();

        //TODO handle errors
        let _ = insert_into(tokens).values(&token).execute(&mut connection);
    }

    fn find_token_by_id(&self, searched_id: Uuid) -> Option<Token> {
        let mut connection = establish_connection();

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
