use axum::{extract::State, http::StatusCode, Json};
use serde_json::json;

use crate::{
    model::api::{ApiResponse, ApiResult},
    AppState,
};

#[tracing::instrument]
pub async fn get_all(State(state): State<AppState>) -> ApiResult {}
