use derive_new::new;
use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::infrastructure::db::entity::users;

#[derive(new, Debug, Validate, Serialize, Deserialize)]
pub struct UserDto {
    #[garde(skip)]
    pub id: i32,

    #[garde(ascii, length(min = 1, max = 255))]
    pub first_name: String,

    #[garde(ascii, length(min = 1, max = 255))]
    pub last_name: String,

    #[garde(email)]
    pub email: String,
}
