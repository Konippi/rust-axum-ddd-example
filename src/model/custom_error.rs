use thiserror;

#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("")]
    InvalidFullName(String)
    InvalidEmail
}