mod tracing;

use axum::Router;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = Router::new();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
