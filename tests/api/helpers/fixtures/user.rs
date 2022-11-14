use diesel::{insert_into, PgConnection, RunQueryDsl};
use uuid::Uuid;

use magic_collection_registry_backend::provider::database::user::DbUser;
use magic_collection_registry_backend::schema::users::dsl::*;

use crate::helpers::fixtures::Fixture;

pub static TEST_USER_ID_1: &str = "bbfddad7-940e-4a85-a35d-925c91b438ba";

pub struct UserFixtures;

impl UserFixtures {
    fn create_user(
        fixture_id: Option<Uuid>,
        fixture_username: String,
        test_password_hash: String,
    ) -> DbUser {
        let fixture_id = match fixture_id {
            Some(value) => value,
            None => Uuid::new_v4(),
        };

        DbUser {
            id: fixture_id,
            username: fixture_username,
            password: test_password_hash,
        }
    }
}

impl Fixture for UserFixtures {
    fn load(
        connection: &mut PgConnection,
        test_password_hash: &str,
    ) -> Result<(), diesel::result::Error> {
        let users_list: Vec<DbUser> = vec![
            UserFixtures::create_user(
                Some(Uuid::parse_str(TEST_USER_ID_1).unwrap()),
                "test@email.com".into(),
                test_password_hash.into(),
            ),
            UserFixtures::create_user(
                None,
                "x7iv7vqe2@mozmail.com".into(),
                test_password_hash.into(),
            ),
        ];

        for user in users_list.iter() {
            insert_into(users).values(user).execute(connection)?;
        }

        Ok(())
    }
}
