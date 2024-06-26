use std::time::Duration;

use axum::Router;
use tower_http::{timeout::TimeoutLayer, trace::TraceLayer};

use crate::{
    config::{Config, CONFIG},
    di_container::DiContainer,
    infrastructure::opentelemetry::OpenTelemetry,
};

mod di_container;
mod config;
mod domain;
mod infrastructure;
mod interface;
mod model;
mod router;
mod use_case;


#[tokio::main]
async fn main() {
    // Load environment variables
    Config::init();

    // Initialize OpenTelemetry
    OpenTelemetry::init();

    let di_container = DiContainer::new().await;
    let app = Router::new()
        .nest("/health", router::health())
        .nest("/users", router::user())
        .layer((
            TraceLayer::new_for_http(),
            TimeoutLayer::new(Duration::from_secs(10)),
        ))
        .with_state(di_container);

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
            .unwrap_or_else(|| panic!("Failed to install CTRL+C signal handler."));
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .unwrap_or_else(|| panic!("Failed to install SIGTERM signal handler."))
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
