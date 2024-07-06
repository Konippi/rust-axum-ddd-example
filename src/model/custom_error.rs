use thiserror;

#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("{0}")]
    InvalidFullName(String),

    #[error("")]
    InvalidEmail(String),
}