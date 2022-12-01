use std::env;

use diesel::r2d2::ConnectionManager;
use diesel::{Connection, PgConnection};
use r2d2::{CustomizeConnection, Pool};

use magic_collection_registry_backend::provider::database::DbConnection;

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
    let database_url = env::var("DATABASE_URL_WITHOUT_DATABASE")
        .expect("DATABASE_URL_WITHOUT_DATABASE must be set");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn establish_test_connection_pool() -> DbConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::new(database_url);

    Pool::builder()
        .max_size(1)
        .connection_customizer(Box::new(TestConnectionCustomizer))
        .build(manager)
        .unwrap()
}
