use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use crate::config::CONFIG;

#[derive(Debug)]
pub struct Db {
    pub conn: DatabaseConnection,
}

impl Db {
    pub async fn init() -> Self {
        let mut opt: ConnectOptions = ConnectOptions::new(CONFIG.database_url.to_string());
        opt.max_connections(100)
            .connect_timeout(Duration::from_secs(5))
            .sqlx_logging(false);
        let conn = Database::connect(opt)
            .await
            .expect("Failed to connect to DB");
        Self { conn }
    }
}
