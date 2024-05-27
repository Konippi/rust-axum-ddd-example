use axum::{http::StatusCode, Json};

use crate::{
    model::api::{ApiResponse, ApiResult},
    usecase::auth_usecase::AuthUsecase,
};

// #[tracing::instrument]
// pub async fn signup() -> ApiResult {
//     let auth = AuthUsecase::signup("test".to_string()).unwrap();
//     Ok(ApiResponse::new(StatusCode::OK, Json(auth)))
// }
