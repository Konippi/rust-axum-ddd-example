use crate::domain::entity::user::User;
use anyhow::Result;

pub trait UserRepositoryImpl: Send + Sync {
    async fn find_by_id(&self, id: i64) -> Result<Option<User>>;
}
