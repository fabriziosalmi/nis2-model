# Vector Store

**Crate**: `nis2-vectordb` | **Files**: `embed.rs`, `lance.rs`, `store.rs` | **Tests**: 1

## What it does

Indexes legal text chunks as 384-dimensional vectors using BGE-Small-EN-v1.5 (via fastembed/ONNX Runtime) and stores them in LanceDB with an HNSW index. Provides semantic search to retrieve the most relevant provisions for a natural-language query.

## Components

- `Embedder` wraps `fastembed::TextEmbedding` behind a `Mutex` (fastembed requires `&mut self`)
- `LanceStore` manages the LanceDB connection and provides `index()` and `search()` methods

## Arrow schema

| Column | Type |
|--------|------|
| `id` | Utf8 |
| `directive` | Utf8 |
| `article` | Utf8 |
| `paragraph` | Utf8 |
| `letter` | Utf8 (nullable) |
| `text` | Utf8 |
| `vector` | FixedSizeList(Float32, 384) |

## Binaries

- `cargo run --bin indexer` -- populates `data/lancedb/` from `data/sources/`
- `cargo run --bin search -- "<query>"` -- returns the 5 most similar chunks with similarity scores

## Test

- `embedding_dimensions_and_semantic_similarity` -- verifies 384 dimensions and that semantically similar texts score higher
