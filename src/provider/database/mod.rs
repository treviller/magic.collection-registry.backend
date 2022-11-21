use std::env;

use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::Pool;

pub mod set;
pub mod token;
pub mod user;

pub fn establish_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    let database_url = env::var("DB_URL").expect("DB_URL must be set");
    let manager = ConnectionManager::new(database_url);

    Pool::builder().max_size(15).build(manager).unwrap()
}
