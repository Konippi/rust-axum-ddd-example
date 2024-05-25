use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value}; // Import the IntoResponse trait

#[derive(Debug)]
pub struct ApiError {
    status: StatusCode,
    message: Json<Value>,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        (self.status, self.message).into_response()
    }
}

impl<E> From<E> for ApiError
where
    E: Into<anyhow::Error>,
{
    /// Helper to create an `ApiError` with a custom status code and message.
    fn from(error: E) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: Json(json!({
                "error": format!("{:#?}", error.into())
            })),
        }
    }
}

pub struct ApiResponse {
    status: StatusCode,
    payload: Json<Value>,
}

impl ApiResponse {
    pub fn new(status: StatusCode, payload: Json<Value>) -> Self {
        Self { status, payload }
    }
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> axum::response::Response {
        (self.status, self.payload).into_response()
    }
}

pub type ApiResult = anyhow::Result<ApiResponse, ApiError>;
