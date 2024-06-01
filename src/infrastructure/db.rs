use std::sync::Arc;

use sea_orm::{ConnectOptions, Database, DbConn};

use crate::config::CONFIG;

#[derive(Debug)]
pub struct Db {
    pub pool: Database,
}

impl Db {
    pub async fn create_connection() -> Arc<DbConn> {
        let mut opt = ConnectOptions::new(CONFIG.database_url.to_string());
        opt.sqlx_logging(false);
        let db = Database::connect(opt)
            .await
            .expect("Failed to connect to database");
        Arc::new(db)
    }
}
