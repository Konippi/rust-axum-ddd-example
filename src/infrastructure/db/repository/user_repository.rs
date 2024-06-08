use std::sync::Arc;

use derive_new::new;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::domain::repository::user_repository::UserRepositoryImpl;
use crate::infrastructure::db::entity::{prelude::User, user};

#[derive(new)]
pub struct UserRepository {
    conn: Arc<DatabaseConnection>,
}

impl UserRepositoryImpl for UserRepository {
    async fn select_all(&self) -> anyhow::Result<Vec<user::Model>> {
        let users = User::find()
            .all(&*self.conn)
            .await
            .expect("Failed to fetch all users");
        Ok(users)
    }

    async fn find_by_id(&self, id: i64) -> anyhow::Result<Option<user::Model>> {
        let user = User::find()
            .filter(user::Column::Id.eq(id))
            .one(&*self.conn)
            .await
            .expect("Failed to fetch user by id");
        match user {
            Some(u) => Ok(Some(u)),
            None => Ok(None),
        }
    }
}
