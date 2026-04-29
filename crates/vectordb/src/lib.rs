//! # nis2-vectordb
//!
//! Vector storage and HNSW semantic search over legal text embeddings.
//! Backed by LanceDB (embedded, serverless) with fastembed for local BGE embeddings.

pub mod embed;
pub mod lance;
pub mod store;
