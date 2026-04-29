//! NIS2 MCP Server binary — runs over stdio.
//!
//! Usage:
//!   cargo run --bin mcp-server
//!
//! Then send JSON-RPC 2.0 messages on stdin, one per line.

use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Log to stderr so stdout stays clean for JSON-RPC
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse()?))
        .with_writer(std::io::stderr)
        .init();

    nis2_mcp_server::server::run_stdio().await
}
