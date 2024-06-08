use crate::domain::entity::user;

pub trait UserRepositoryImpl: Send + Sync {
    async fn select_all(&self) -> anyhow::Result<Vec<user::Model>>;
    async fn find_by_id(&self, id: i64) -> anyhow::Result<Option<user::Model>>;
}
