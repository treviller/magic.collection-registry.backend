use std::env;
use std::error::Error;

use diesel::pg::Pg;
use diesel::{PgConnection, RunQueryDsl};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use magic_collection_registry_backend::provider::database::establish_connection_pool;

use crate::helpers::database::establish_connection_without_db;
use crate::helpers::fixtures::set::SetFixtures;
use crate::helpers::fixtures::token::TokenFixtures;
use crate::helpers::fixtures::user::UserFixtures;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

mod set;
mod token;
mod user;

pub fn load_fixtures() {
    reinitialize_database().expect("Failed to reinitialize database");

    let test_password_hash = "$argon2id$v=19$m=4096,t=3,p=1$njFuptpoWRAhlPsfwK5IYA$eJJGq0hW6/IMDKsVal7mxSK/YlOvI12JyxcETS5cYPQ";
    let db_pool = establish_connection_pool();
    let mut connection = db_pool.get().unwrap();

    apply_migrations(&mut connection).expect("Failed to apply migrations");

    UserFixtures::load(&mut connection, test_password_hash)
        .expect("Failed to load users fixtures in database");
    TokenFixtures::load(&mut connection, test_password_hash)
        .expect("Failed to load tokens fixtures in database");
    SetFixtures::load(&mut connection, test_password_hash)
        .expect("Failed to load sets fixtures in database");
}

fn reinitialize_database() -> Result<(), diesel::result::Error> {
    let mut connection = establish_connection_without_db();
    let database_name = env::var("DB_NAME").expect("DB_NAME must be set");

    diesel::sql_query(format!(r#"DROP DATABASE IF EXISTS "{}";"#, database_name))
        .execute(&mut connection)?;
    diesel::sql_query(format!(r#"CREATE DATABASE "{}";"#, database_name))
        .execute(&mut connection)?;

    Ok(())
}

fn apply_migrations(
    connection: &mut impl MigrationHarness<Pg>,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    connection.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}

trait Fixture {
    fn load(
        connection: &mut PgConnection,
        test_password_hash: &str,
    ) -> Result<(), diesel::result::Error>;
}
