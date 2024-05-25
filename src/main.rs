mod handlers;
mod models;
mod router;
mod tracer;
mod usecases;

use axum::Router;
use tower_http::{timeout::TimeoutLayer, trace::TraceLayer};

#[tokio::main]
async fn main() {
    // Load environment variables
    dotenv::from_filename(format!(
        ".env.{}",
        std::env::var("SERVER_ENV").unwrap_or_else(|_| "local".to_string())
    ))
    .expect("Failed to load .env file");

    // Initialize tracing
    tracer::init();

    let app = Router::new()
        .nest("/health", router::health())
        // .nest("/auth", router::auth())
        .layer((
            TraceLayer::new_for_http(),
            TimeoutLayer::new(std::time::Duration::from_secs(10)),
        ));
    let listener = tokio::net::TcpListener::bind(format!(
        "{}:{}",
        std::env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
        std::env::var("SERVER_PORT").unwrap_or_else(|_| "8080".to_string())
    ))
    .await
    .unwrap();

    tracing::info!("Running on {} 🚀", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap()
}

/// Signal handler for graceful shutdown
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

    // Shutdown OpenTelemetry
    tokio::task::spawn_blocking(opentelemetry::global::shutdown_tracer_provider)
        .await
        .unwrap();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
