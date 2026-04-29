# Getting Started

## Prerequisites

- **Rust** 1.85+ (2024 edition)
- ~500 MB RAM for ONNX Runtime (BGE-Small model, downloaded on first run)

## Build

```bash
git clone https://github.com/fabriziosalmi/nis2-model.git
cd nis2-model
cargo build --workspace
```

## Run tests

```bash
cargo test --workspace   # 61 tests across 6 crates
```

## Index legal texts

The vector store must be populated before semantic search works:

```bash
cargo run --bin indexer
```

This parses `data/sources/nis2_articles.json` (35 chunks) and `data/sources/dora_articles.json` (14 chunks) into LanceDB at `data/lancedb/`.

## Semantic search

```bash
cargo run --bin search -- "autenticazione multi-fattore"
```

## Generate a compliance report

```bash
cargo run --bin report   # Italian compliance report (template backend)
cargo run --bin demo     # Raw ComplianceStatus JSON
```

## Start the REST API server

```bash
cargo run --bin api-server                 # listens on 0.0.0.0:8080
cargo run --bin api-server -- --port 3000  # custom port
```

## Start the MCP server

```bash
cargo run --bin mcp-server   # JSON-RPC over stdio
```

See [MCP Tools](/api/mcp-tools) for tool definitions.

## Run benchmarks

```bash
cargo run --bin bench --release
```
