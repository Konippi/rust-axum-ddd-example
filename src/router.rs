use axum::{
    routing::{get, post},
    Router,
};

use crate::handlers::{auth_handler, health_handler};

pub fn health() -> Router {
    Router::new().route("/", get(health_handler::health_check))
}

// pub fn auth() -> Router {
//     Router::new().route("/signup", post(auth_handler::signup))
// }
