use axum::{Router, routing::get};

use crate::{
    DiContainer,
    interface::{health_handler, user_handler},
};

pub fn health() -> Router<DiContainer> {
    Router::new().route("/", get(health_handler::health_check))
}

pub fn user() -> Router<DiContainer> {
    Router::new().route("/", get(user_handler::get_all))
}
