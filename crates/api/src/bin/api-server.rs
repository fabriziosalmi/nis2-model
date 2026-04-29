//! NIS2 REST API server binary.
//!
//! Usage:
//!   cargo run --bin api-server
//!   cargo run --bin api-server -- --port 3000

use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse()?))
        .init();

    let port = std::env::args()
        .position(|a| a == "--port")
        .and_then(|i| std::env::args().nth(i + 1))
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(8080);

    let app = nis2_api::routes::build_router();
    let addr = format!("0.0.0.0:{port}");

    tracing::info!("NIS2 API server listening on http://{addr}");
    tracing::info!("Endpoints:");
    tracing::info!("  GET  /api/v1/health");
    tracing::info!("  POST /api/v1/evaluate");
    tracing::info!("  POST /api/v1/applicability");
    tracing::info!("  POST /api/v1/sanctions");
    tracing::info!("  POST /api/v1/obligations");
    tracing::info!("  POST /api/v1/report");

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
