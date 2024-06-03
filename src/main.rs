mod config;
mod domain;
mod infrastructure;
mod interface;
mod model;
mod router;
mod usecase;

use std::sync::Arc;

use axum::Router;
use config::{Config, CONFIG};
use infrastructure::{db::Db, opentelemetry::OpenTelemetry};
use tower_http::{timeout::TimeoutLayer, trace::TraceLayer};

pub struct AppState {
    db: Arc<Db>,
}

#[tokio::main]
async fn main() {
    // Load environment variables
    Config::init();

    // Initialize OpenTelemetry
    OpenTelemetry::init();

    // Pool connection to DB
    let db_conn = Arc::new(Db::init().await);
    let app = Router::new()
        .nest("/health", router::health())
        .layer((
            TraceLayer::new_for_http(),
            TimeoutLayer::new(std::time::Duration::from_secs(10)),
        ))
        .with_state(db_conn);

    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", CONFIG.server_host, CONFIG.server_port))
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
