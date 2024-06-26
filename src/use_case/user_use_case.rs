use crate::{
    domain::{
        model::user::{self, user_dto::UserDto, user_entity::UserEntity},
        repository::user_repository::UserRepository,
    },
    infrastructure::db::entity::users,
};
use axum::async_trait;
use derive_new::new;
use std::sync::Arc;

#[async_trait]
pub trait UserUseCase: Send + Sync + 'static {
    async fn get_all_users(&self) -> anyhow::Result<Vec<UserDto>>;
    async fn get_user_by_id(&self, id: i32) -> anyhow::Result<users::Model>;
}

#[derive(new)]
pub struct UserUseCaseImpl {
    user_repository: Arc<dyn UserRepository>,
}

#[async_trait]
impl UserUseCase for UserUseCaseImpl {
    async fn get_all_users(&self) -> anyhow::Result<Vec<UserDto>> {
        let users = self.user_repository.select_all().await?;
        Ok(users.into_iter().map(UserEntity::into).collect())
    }

    async fn get_user_by_id(&self, id: i32) -> anyhow::Result<users::Model> {
        let user = self.user_repository.find_by_id(id).await?;
        match user {
            Some(u) => Ok(u),
            None => Err(anyhow::Error::msg("User not found")),
        }
    }
}
