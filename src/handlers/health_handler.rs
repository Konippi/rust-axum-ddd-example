use axum::{http::StatusCode, Json};
use serde_json::json;

use crate::models::api::{ApiResponse, ApiResult};

#[tracing::instrument]
pub async fn health_check() -> ApiResult {
    Ok(ApiResponse::new(
        StatusCode::OK,
        Json(json!({"status": "ok"})),
    ))
}
