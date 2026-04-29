# Overview

`nis2-model` is a Rust workspace that models the compliance logic of EU cybersecurity regulation (NIS2 Directive, DORA Regulation).

## What it does

Given a `CompanyProfile` (sector, employee count, annual revenue), the engine:

1. Determines whether the entity falls under NIS2 scope (Art. 2)
2. Classifies it as Essential (Art. 3(1)), Important (Art. 3(2)), or OutOfScope
3. Maps all 16 applicable obligations (Art. 20, 21, 23)
4. Calculates the maximum sanction under Art. 34
5. Sets incident reporting deadlines (24h / 72h / 30 days per Art. 23(4))
6. Generates a structured Italian-language compliance report

## Directives covered

| Directive | Coverage |
|-----------|----------|
| **NIS2** (EU 2022/2555) | Applicability, classification, Art. 20/21/23 obligations, Art. 34 sanctions |
| **DORA** (EU 2022/2554) | Indexed for semantic search (14 chunks from 5 articles) |

## Technology stack

| Component | Technology |
|-----------|------------|
| Language | Rust 2024 edition |
| Embeddings | BGE-Small-EN-v1.5 via fastembed (ONNX Runtime, 384 dimensions) |
| Vector store | LanceDB embedded (HNSW index) |
| Protocol | MCP (JSON-RPC 2.0 over stdio) |
| HTTP server | Axum 0.8 |
| Validation | JSON Schema via schemars + jsonschema |
| WASM plugins | Extism |
