# System Overview

The engine is a Cargo workspace with 7 crates organized as a unidirectional pipeline.

## Crates

| Crate | Source files | Tests | Purpose |
|-------|-------------|-------|---------|
| `nis2-ingestion` | `chunk.rs`, `parser.rs`, `types.rs` | 3 | JSON parser, semantic chunking by legal hierarchy |
| `nis2-vectordb` | `embed.rs`, `lance.rs`, `store.rs` | 1 | LanceDB storage, BGE-Small embeddings, HNSW search |
| `nis2-rules` | `engine.rs`, `obligations.rs`, `schema.rs`, `validation.rs`, `adapters.rs` | 20 | Boolean decision trees, obligation catalog, JSON Schema validation |
| `nis2-mcp-server` | `protocol.rs`, `tools.rs`, `server.rs` | 13 | MCP JSON-RPC 2.0 server over stdio |
| `nis2-slm` | `prompt.rs`, `grammar.rs`, `inference.rs`, `report.rs` | 18 | Prompt construction, GBNF grammar, template report generation |
| `nis2-api` | `handlers.rs`, `routes.rs` | 6 | Axum REST API, benchmark binary |
| `nis2-chat` | `engine.rs`, `cache.rs`, `dataset*.rs` | 20 | Semantic cache, BM25 search, session tracking, multilingual Q&A |

## Data flow

```
data/sources/*.json -> ingestion (parse+chunk) -> vectordb (embed+index)
                                                        |
CompanyProfile -> rules (evaluate) -> ComplianceStatus -> slm (report)
                       |                    |
                  mcp-server            api-server
                  (stdio)              (HTTP :8080)
```

## Binaries

| Binary | Crate | Purpose |
|--------|-------|---------|
| `indexer` | vectordb | Populate LanceDB from JSON sources |
| `search` | vectordb | Semantic search over indexed provisions |
| `demo` | rules | Print ComplianceStatus JSON |
| `report` | slm | Generate Italian compliance report |
| `mcp-server` | mcp-server | MCP JSON-RPC server |
| `api-server` | api | Axum HTTP server |
| `bench` | api | Benchmark suite |
| `chat` | chat | Interactive CLI chatbot |
