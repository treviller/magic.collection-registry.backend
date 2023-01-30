use std::env;

use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

use crate::routes::PaginationParameters;

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

pub struct PaginatedResult<T> {
    pub elements: Vec<T>,
    pub total_elements: i64,
    pagination: PaginationParameters,
    last_page_index: i64,
}

impl<T> PaginatedResult<T> {
    pub fn new(elements: Vec<T>, total_elements: i64, pagination: PaginationParameters) -> Self {
        let last_page_index =
            (total_elements as f64 / (pagination.get_size() as f64)).ceil() as i64;

        Self {
            elements,
            total_elements,
            last_page_index,
            pagination,
        }
    }

    pub fn get_previous_page_index(&self) -> Option<String> {
        if self.pagination.get_current_page_index() <= 1 {
            return None;
        }

        let previous_page_index = self.pagination.get_current_page_index() - 1;

        return Some(previous_page_index.to_string());
    }

    pub fn get_next_page_index(&self) -> Option<String> {
        if self.pagination.get_current_page_index() >= self.last_page_index {
            return None;
        }

        let next_page_index = self.pagination.get_current_page_index() + 1;

        return Some(next_page_index.to_string());
    }
}
