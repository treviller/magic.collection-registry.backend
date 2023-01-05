use std::env;

use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub mod card;
pub mod set;
pub mod token;
pub mod user;

pub type DbConnection = Pool<Postgres>;

pub fn establish_connection_pool() -> DbConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgPoolOptions::new()
        .max_connections(15)
        .connect_lazy(&database_url)
        .expect("Unable to establish database connection")
}
