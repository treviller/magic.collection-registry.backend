use std::env;

use diesel::r2d2::ConnectionManager;
use diesel::{Connection, PgConnection};
use magic_collection_registry_backend::provider::database::DbConnection;
use r2d2::{CustomizeConnection, Pool};

#[derive(Debug, Clone, Copy)]
pub struct TestConnectionCustomizer;

impl<C, E> CustomizeConnection<C, E> for TestConnectionCustomizer
where
    C: Connection,
{
    fn on_acquire(&self, conn: &mut C) -> Result<(), E> {
        conn.begin_test_transaction()
            .expect("Failed to start test transaction.");

        Ok(())
    }
}

pub fn establish_connection_without_db() -> PgConnection {
    let database_url =
        env::var("DB_URL_WITHOUT_DATABASE").expect("DB_URL_WITHOUT_DATABASE must be set");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn establish_test_connection_pool() -> DbConnection {
    let database_url = env::var("DB_URL").expect("DB_URL must be set");
    let manager = ConnectionManager::new(database_url);

    Pool::builder()
        .max_size(1)
        .connection_customizer(Box::new(TestConnectionCustomizer))
        .build(manager)
        .unwrap()
}
