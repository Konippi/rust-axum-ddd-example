mod tracing;

use std::{env, future};

use axum::Router;
use dotenv;

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install CTRL+C signal handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

#[tokio::main]
async fn main() {
    // Load environment variables
    dotenv::from_filename(format!(
        ".env.{}",
        env::var("SERVER_ENV").unwrap_or_else(|_| "local".to_string())
    ))
    .expect("Failed to load .env file");

    // Initialize tracing
    tracing::init();

    let app = Router::new();
    let listener = tokio::net::TcpListener::bind(format!(
        "{}:{}",
        env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
        env::var("SERVER_PORT").unwrap_or_else(|_| "3000".to_string())
    ))
    .await
    .unwrap();

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap()
}
