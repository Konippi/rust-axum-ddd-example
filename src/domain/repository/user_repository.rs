use axum::async_trait;

use crate::domain::model::user::user_entity::UserEntity;

#[async_trait]
pub trait UserRepository: Send + Sync + 'static {
    async fn select_all(&self) -> anyhow::Result<Vec<UserEntity>>;
    async fn find_by_id(&self, id: i32) -> anyhow::Result<Option<UserEntity>>;
}
