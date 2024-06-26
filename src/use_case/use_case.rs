use std::sync::Arc;

use crate::domain::repository::user_repository::UserRepository;

use super::user_use_case::{UserUseCase, UserUseCaseImpl};

#[derive(Clone)]
pub struct UseCase {
    pub user_use_case: Arc<dyn UserUseCase>,
}

impl UseCase {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        let user_use_case = Arc::new(UserUseCaseImpl::new(user_repository));
        Self { user_use_case }
    }
}
