use std::env;

use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::Pool;

pub mod card;
pub mod set;
pub mod token;
pub mod user;

pub type DbConnection = Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection_pool() -> DbConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::new(database_url);

    Pool::builder().max_size(15).build(manager).unwrap()
}
