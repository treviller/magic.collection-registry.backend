use diesel::{insert_into, PgConnection, RunQueryDsl};
use uuid::Uuid;

use magic_collection_registry_backend::domain::model::token::TokenType;
use magic_collection_registry_backend::provider::database::token::DbToken;
use magic_collection_registry_backend::schema::tokens::dsl::*;

use crate::helpers::fixtures::user::TEST_USER_ID_1;
use crate::helpers::fixtures::Fixture;

static TOKEN_TEST_ID: &str = "bbfddad7-940e-4a85-a35d-925c91b438bd";

pub struct TokenFixtures;

impl TokenFixtures {
    fn create_token(
        fixture_id: Option<Uuid>,
        fixture_token_type: TokenType,
        fixture_user_id: Uuid,
    ) -> DbToken {
        let fixture_id = match fixture_id {
            Some(value) => value,
            None => Uuid::new_v4(),
        };

        DbToken {
            id: fixture_id,
            token_type: fixture_token_type,
            user_id: fixture_user_id,
        }
    }
}

impl Fixture for TokenFixtures {
    fn load(
        connection: &mut PgConnection,
        _test_password_hash: &str,
    ) -> Result<(), diesel::result::Error> {
        let user_id_1 = Uuid::parse_str(TEST_USER_ID_1).unwrap();

        let tokens_list = vec![
            TokenFixtures::create_token(
                Some(Uuid::parse_str(TOKEN_TEST_ID).unwrap()),
                TokenType::ResetPassword,
                user_id_1,
            ),
            TokenFixtures::create_token(None, TokenType::ResetPassword, user_id_1),
        ];

        for token in tokens_list.iter() {
            insert_into(tokens).values(token).execute(connection)?;
        }

        Ok(())
    }
}
