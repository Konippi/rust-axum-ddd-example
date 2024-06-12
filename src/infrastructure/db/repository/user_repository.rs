use std::sync::Arc;

use axum::async_trait;
use derive_new::new;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::domain::repository::user_repository::UserRepository;
use crate::infrastructure::db::entity::{prelude::User, user};
use crate::infrastructure::db::Db;

#[derive(new)]
pub struct UserRepositoryImpl {
    db: Arc<Db>,
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn select_all(&self) -> anyhow::Result<Vec<user::Model>> {
        let users = User::find()
            .all(&*self.db.conn)
            .await
            .expect("Failed to fetch all users");
        Ok(users)
    }

    async fn find_by_id(&self, id: i32) -> anyhow::Result<Option<user::Model>> {
        let user = User::find()
            .filter(user::Column::Id.eq(id))
            .one(&*self.db.conn)
            .await
            .expect("Failed to fetch user by id");
        match user {
            Some(u) => Ok(Some(u)),
            None => Ok(None),
        }
    }
}
