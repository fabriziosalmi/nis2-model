//! Vector store abstraction.
//!
//! Defines the trait contract for vector storage backends.
//! Sprint 1 Task 1.5 will provide the LanceDB implementation.

use anyhow::Result;
use nis2_ingestion::Chunk;

/// A search result with its similarity score.
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub chunk: Chunk,
    /// Cosine similarity score (0.0–1.0).
    pub score: f32,
}

/// Trait abstraction over vector storage backends.
pub trait VectorStore {
    /// Insert a chunk with its embedding vector.
    fn insert(&mut self, chunk: &Chunk, embedding: &[f32]) -> Result<()>;

    /// Search for the top-k most similar chunks to a query embedding.
    fn search(&self, query_embedding: &[f32], top_k: usize) -> Result<Vec<SearchResult>>;
}
