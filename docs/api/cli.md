# CLI

The workspace provides 7 binaries.

## indexer

Populates the LanceDB vector store from JSON source files.

```bash
cargo run --bin indexer
```

Reads `data/sources/nis2_articles.json` (35 chunks) and `data/sources/dora_articles.json` (14 chunks). Writes to `data/lancedb/`.

## search

Semantic search over indexed legal texts.

```bash
cargo run --bin search -- "<query>"
```

Returns the 5 most similar chunks with cosine similarity scores.

## demo

Prints the raw `ComplianceStatus` JSON for a hardcoded company profile.

```bash
cargo run --bin demo
```

## report

Generates a full Italian compliance report using the `TemplateBackend`.

```bash
cargo run --bin report
```

## mcp-server

Starts the MCP JSON-RPC server on stdio.

```bash
cargo run --bin mcp-server
```

## api-server

Starts the Axum HTTP server.

```bash
cargo run --bin api-server                 # port 8080
cargo run --bin api-server -- --port 3000  # custom port
```

## bench

Runs the benchmark suite measuring latency of rule engine evaluation, report generation, MCP tool dispatch, and JSON serialization.

```bash
cargo run --bin bench --release
```
