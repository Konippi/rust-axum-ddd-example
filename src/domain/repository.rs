use std::sync::Arc;

use sqlx::{Pool, Postgres};

pub mod user_repository;

pub struct Repository {
    pub pool: Arc<Pool<Postgres>>,
}
