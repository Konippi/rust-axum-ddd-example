use std::sync::Arc;

use sea_orm::Database;

use crate::domain::{entity::user::User, repository::user_repository::UserRepositoryImpl};

pub struct UserRepository {
    conn: Arc<Database>,
}

impl UserRepositoryImpl for UserRepository {
    async fn find_by_id(&self, id: i64) -> anyhow::Result<Option<User>> {
        let user = User::find()
            .filter(User::Id.eq(id))
            .one(self.conn.as_ref())
            .await
            .expect("Failed to fetch user");
        Ok(user)
    }
}
