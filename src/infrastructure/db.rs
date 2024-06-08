use crate::config::CONFIG;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::{sync::Arc, time::Duration};

pub mod entity;
pub mod repository;

#[derive(Debug, Clone)]
pub struct Db {
    pub conn: Arc<DatabaseConnection>,
}

impl Db {
    pub async fn new() -> Self {
        let conn = Self::connect().await;
        // self.migrate().await.unwrap();
        Self { conn }
    }

    async fn connect() -> Arc<DatabaseConnection> {
        let mut opt: ConnectOptions = ConnectOptions::new(CONFIG.database_url.to_string());
        opt.max_connections(100)
            .connect_timeout(Duration::from_secs(5))
            .sqlx_logging(false);
        Arc::new(
            Database::connect(opt)
                .await
                .expect("Failed to connect to DB"),
        )
    }
}
