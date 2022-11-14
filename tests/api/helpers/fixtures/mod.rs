use std::env;
use std::error::Error;

use diesel::pg::Pg;
use diesel::{Connection, PgConnection, RunQueryDsl};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;

use crate::helpers::fixtures::token::TokenFixtures;
use crate::helpers::fixtures::user::UserFixtures;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

mod token;
mod user;

pub fn load_fixtures() {
    load_env_values();
    reinitialize_database().expect("Failed to reinitialize database");

    let mut connection = establish_connection();
    let test_password_hash = "$argon2id$v=19$m=4096,t=3,p=1$njFuptpoWRAhlPsfwK5IYA$eJJGq0hW6/IMDKsVal7mxSK/YlOvI12JyxcETS5cYPQ";

    apply_migrations(&mut connection).expect("Failed to apply migrations");

    UserFixtures::load(&mut connection, test_password_hash)
        .expect("Failed to load users fixtures in database");
    TokenFixtures::load(&mut connection, test_password_hash)
        .expect("Failed to load tokens fixtures in database");
}

fn load_env_values() {
    dotenv().ok();
    dotenvy::from_filename(".env.test").ok();
}

fn establish_connection_without_db() -> PgConnection {
    let database_url =
        env::var("DB_URL_WITHOUT_DATABASE").expect("DB_URL_WITHOUT_DATABASE must be set");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn establish_connection() -> PgConnection {
    let database_url = env::var("DB_URL").expect("DB_URL must be set");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
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
