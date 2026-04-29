//! CLI tool: Search the legal text vector store.
//!
//! Usage: cargo run --bin search -- "autenticazione multi-fattore"

use anyhow::Result;
use tracing_subscriber::EnvFilter;

use nis2_vectordb::embed::{EmbedModel, Embedder};
use nis2_vectordb::lance::LanceStore;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse()?))
        .init();

    let query = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "misure di sicurezza informatica".to_string());

    let embedder = Embedder::new(EmbedModel::BgeSmall)?;
    let store = LanceStore::open("data/lancedb", embedder).await?;

    println!("🔍 Query: \"{query}\"\n");

    let results = store.search(&query, 5).await?;

    for (i, r) in results.iter().enumerate() {
        println!(
            "{}. [score: {:.4}] {}\n   {}…\n",
            i + 1,
            r.score,
            r.chunk.reference,
            &r.chunk.text[..r.chunk.text.len().min(120)]
        );
    }

    Ok(())
}
