use axum::{extract::State, http::StatusCode, Json};
use serde_json::json;

use crate::{
    DiContainer,
    model::api::{ApiResponse, ApiResult},
};

pub async fn get_all(State(DiContainer { use_case }): State<DiContainer>) -> ApiResult {
    let users = use_case.user_use_case.get_all_users().await?;
    Ok(ApiResponse::new(
        StatusCode::OK,
        Json(json!({"users": users})),
    ))
}
