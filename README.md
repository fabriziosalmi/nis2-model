# NIS2 Deterministic Compliance Engine

A fully deterministic, locally-executable compliance engine for the [NIS2 Directive (EU 2022/2555)](https://eur-lex.europa.eu/eli/dir/2022/2555) and [DORA Regulation (EU 2022/2554)](https://eur-lex.europa.eu/eli/reg/2022/2554).

**Zero hallucination. Zero cloud dependency. Mathematically constrained output.**

## Architecture

```
EU Legal Texts → Ingestion → Vector DB (HNSW) → Rule Engine → MCP Server → Local SLM → Report
```

| Crate | Purpose |
|-------|---------|
| `nis2-ingestion` | PDF/HTML parsing, semantic chunking by legal hierarchy |
| `nis2-vectordb` | Embedded vector store (LanceDB) with HNSW indexing |
| `nis2-rules` | Deterministic boolean decision trees, JSON Schema validated |
| `nis2-mcp-server` | Model Context Protocol server (Anthropic spec, JSON-RPC) |
| `nis2-slm` | Local SLM inference with grammar-constrained generation |
| `nis2-api` | REST API layer (axum) for frontend integration |

## Quick Start

```bash
# Build all crates
cargo build --workspace

# Run tests
cargo test --workspace
```

## Integration Targets

- **nis2-public** — WASM-compiled compliance checker for browser embedding
- **certmate** — REST API backend for audit workflows

## License

EUPL-1.2
