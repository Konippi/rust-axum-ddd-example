use crate::model::{ApiResponse, ApiResult};
use axum::{http::StatusCode, Json};
use serde_json::json;

#[tracing::instrument]
pub async fn health_check() -> ApiResult {
    Ok(ApiResponse::new(
        StatusCode::OK,
        Json(json!({"message": "OK"})),
    ))
}
