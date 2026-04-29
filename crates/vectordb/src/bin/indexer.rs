//! CLI tool: Index legal texts into LanceDB vector store.
//!
//! Usage: cargo run --bin indexer -- --data-dir data/sources --db-path data/lancedb

use std::path::PathBuf;

use anyhow::Result;
use tracing::info;
use tracing_subscriber::EnvFilter;

use nis2_ingestion::parser::load_articles_from_json;
use nis2_vectordb::embed::{EmbedModel, Embedder};
use nis2_vectordb::lance::LanceStore;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse()?))
        .init();

    let data_dir = PathBuf::from("data/sources");
    let db_path = "data/lancedb";

    info!("Initializing embedder (BGE-Small-EN-v1.5)…");
    let embedder = Embedder::new(EmbedModel::BgeSmall)?;

    info!("Opening LanceDB at {db_path}…");
    let store = LanceStore::open(db_path, embedder).await?;

    // Load and index NIS2 articles
    let nis2_path = data_dir.join("nis2_articles.json");
    if nis2_path.exists() {
        info!("Loading NIS2 articles from {}", nis2_path.display());
        let chunks = load_articles_from_json(&nis2_path)?;
        info!("Parsed {} NIS2 chunks", chunks.len());
        let indexed = store.index_chunks(&chunks).await?;
        info!("Indexed {indexed} NIS2 chunks");
    }

    // Load and index DORA articles
    let dora_path = data_dir.join("dora_articles.json");
    if dora_path.exists() {
        info!("Loading DORA articles from {}", dora_path.display());
        let chunks = load_articles_from_json(&dora_path)?;
        info!("Parsed {} DORA chunks", chunks.len());
        let indexed = store.index_chunks(&chunks).await?;
        info!("Indexed {indexed} DORA chunks");
    }

    info!("Indexing complete.");
    Ok(())
}
