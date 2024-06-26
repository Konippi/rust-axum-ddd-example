use derive_new::new;

use crate::infrastructure::db::entity::users;

use super::user_dto::UserDto;

#[derive(new, Debug)]
pub struct UserEntity {
    pub id: i32,
    pub name: FullName,
    pub email: Email,
}

impl From<users::Model> for UserEntity {
    fn from(user_model: users::Model) -> Self {
        UserEntity {
            id: user_model.id,
            name: FullName::new(user_model.first_name, user_model.last_name),
            email: Email::new(user_model.email),
        }
    }
}

impl Into<UserDto> for UserEntity {
    fn into(self) -> UserDto {
        UserDto {
            id: self.id,
            first_name: self.name.get_first_name(),
            last_name: self.name.get_last_name(),
            email: self.email,
        }
    }
}
