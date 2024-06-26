use std::sync::Arc;

use crate::{
    infrastructure::db::{Db, repository::user_repository::UserRepositoryImpl},
    use_case::use_case::UseCase,
};

#[derive(Clone)]
pub struct DiContainer {
    pub use_case: UseCase,
}

impl DiContainer {
    pub async fn new() -> Self {
        // Initialize the database
        let db = Arc::new(Db::new().await);
        let use_case = UseCase::new(Arc::new(UserRepositoryImpl::new(db.clone())));
        Self { use_case }
    }
}
