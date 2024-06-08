use axum::{routing::get, Router};

use crate::{
    interface::{health_handler, user_handler},
    AppState,
};

pub fn health() -> Router<AppState> {
    Router::new().route("/", get(health_handler::health_check))
}

pub fn user() -> Router<AppState> {
    Router::new().route("/", get(user_handler::get_all))
}
