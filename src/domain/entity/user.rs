use chrono::{Local, NaiveDateTime};
use sqlx;

pub type Credential = String;

#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub credential: Credential,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl User {
    pub fn new(id: i64, email: &str, credential: &str) -> Self {
        User {
            id,
            email: email.to_string(),
            credential: credential.to_string(),
            created_at: Local::now().naive_local(),
            updated_at: Local::now().naive_local(),
        }
    }
}
