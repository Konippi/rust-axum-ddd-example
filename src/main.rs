mod tracing;

use std::env;

use axum::Router;
use dotenv;

#[tokio::main]
async fn main() -> std::io::Result<()> {
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

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
