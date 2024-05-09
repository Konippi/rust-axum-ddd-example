use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::{json, Value};

pub struct ApiResponse {
    status: StatusCode,
    response: Json<Value>,
}

impl ApiResponse {
    /// Helper to create an `ApiResponse` with a custom status code and message.
    pub fn new(status: StatusCode, response: Json<Value>) -> Self {
        Self { status, response }
    }
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> Response {
        (self.status, self.response).into_response()
    }
}

pub struct ApiError {
    status: StatusCode,
    message: Json<Value>,
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

impl IntoResponse for ApiError {
    /// Convert the `ApiError` into an `axum::Response`.
    fn into_response(self) -> Response {
        (self.status, self.message).into_response()
    }
}

pub type ApiResult = anyhow::Result<ApiResponse, ApiError>;
