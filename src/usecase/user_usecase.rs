use derive_new::new;

use crate::{
    domain::repository::user_repository::UserRepositoryImpl, infrastructure::db::entity::user,
};

pub trait UserUsecaseImpl {
    async fn get_all_users(&self) -> anyhow::Result<Vec<user::Model>>;
    async fn get_user_by_id(&self, id: i64) -> anyhow::Result<Option<user::Model>>;
}

#[derive(new)]
pub struct UserUsecase<R>
where
    R: UserRepositoryImpl,
{
    user_repository: R,
}

impl<R> UserUsecaseImpl for UserUsecase<R>
where
    R: UserRepositoryImpl,
{
    async fn get_all_users(&self) -> anyhow::Result<Vec<user::Model>> {
        let users = self.user_repository.select_all().await?;
        if users.is_empty() {
            return Ok(vec![]);
        }
        Ok(users)
    }

    async fn get_user_by_id(&self, id: i64) -> anyhow::Result<Option<user::Model>> {
        let user = self.user_repository.find_by_id(id).await?;
        if user.is_none() {
            return Ok(None);
        }
        Ok(user)
    }
}
