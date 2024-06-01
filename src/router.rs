use axum::{routing::get, Router};

use crate::interface::health_handler;

pub fn health() -> Router {
    Router::new().route("/", get(health_handler::health_check))
}
