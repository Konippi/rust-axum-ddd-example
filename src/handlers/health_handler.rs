use crate::model::ApiResult;
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

#[tracing::instrument]
pub async fn health_check() -> ApiResult<impl IntoResponse> {
    Ok((StatusCode::OK, Json(json!({"message": "OK"}))))
}
