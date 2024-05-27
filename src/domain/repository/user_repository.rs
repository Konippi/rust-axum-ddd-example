use crate::entity::user::User;
use anyhow::Result;
use sqlx::Error;

pub trait UserRepositoryImpl: Send + Sync {
    async fn find_by_id(&self, id: i64) -> Result<User, Error>;
}
