use std::sync::Arc;

use crate::config::CONFIG;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

#[derive(Debug)]
pub struct Db {
    pub pool: Arc<Pool<Postgres>>,
}

impl Db {
    pub async fn create_pool() -> Arc<Pool<Postgres>> {
        let pool = PgPoolOptions::new()
            .max_connections(25)
            .connect(&CONFIG.database_url)
            .await
            .unwrap_or_else(|e| {
                panic!("Failed to connect to DB: {}", e);
            });
        Arc::new(pool)
    }
}
