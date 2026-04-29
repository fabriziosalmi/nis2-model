<div align="center">

# NIS2 Compliance Engine

**Deterministic, locally-executable compliance engine for EU cybersecurity regulation**

[![License: EUPL-1.2](https://img.shields.io/badge/License-EUPL--1.2-blue.svg)](https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12)
[![Rust](https://img.shields.io/badge/Rust-2024_Edition-orange.svg)](https://www.rust-lang.org/)
[![Tests](https://img.shields.io/badge/Tests-61%20passing-brightgreen.svg)](#testing)

*Zero hallucination · Zero cloud dependency · Deterministic output*

[Documentation](docs/) · [Quick Start](#quick-start) · [Architecture](#architecture) · [MCP Integration](#mcp-integration)

</div>

---

## Overview

Production-grade compliance engine for the [**NIS2 Directive (EU 2022/2555)**](https://eur-lex.europa.eu/eli/dir/2022/2555) and [**DORA Regulation (EU 2022/2554)**](https://eur-lex.europa.eu/eli/reg/2022/2554). Runs entirely locally — no API keys, no cloud, no data exfiltration.

### What it does

Given a company profile (sector, size, revenue), the engine:

1. **Determines applicability** — NIS2 in-scope or out-of-scope
2. **Classifies the entity** — Essential (Art. 3(1)) or Important (Art. 3(2))
3. **Maps all 16 obligations** — Art. 20 governance + Art. 21(2)(a–j) cybersecurity + Art. 23 incident reporting
4. **Calculates sanctions** — Art. 34 maximum penalty (€10M / 2% or €7M / 1.4%)
5. **Generates a formal report** — in Italian, with exact article citations, zero invented content
6. **Retrieves relevant legal text** — semantic search over 49 indexed NIS2/DORA provisions

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    NIS2 Compliance Engine                │
├──────────┬──────────┬──────────┬──────────┬──────────────┤
│ Ingestion│ VectorDB │  Rules   │   MCP    │     SLM      │
│          │          │          │  Server  │              │
│ JSON     │ LanceDB  │ Boolean  │ JSON-RPC │ Template +   │
│ parser   │ + HNSW   │ decision │ stdio    │ GBNF grammar │
│ semantic │ BGE-Small│ trees    │ 4 tools  │ zero-halluc. │
│ chunking │ 384-dim  │ Art.21/23│          │ Italian      │
└──────────┴──────────┴──────────┴──────────┴──────────────┘
```

| Crate | Purpose | Tests |
|-------|---------|-------|
| [`nis2-ingestion`](crates/ingestion/) | Legal text parsing, semantic chunking by article/paragraph/letter | 3 |
| [`nis2-vectordb`](crates/vectordb/) | Embedded vector store (LanceDB), BGE-Small embeddings, HNSW search | 1 |
| [`nis2-rules`](crates/rules/) | Deterministic rule engine, JSON Schema validation, Extism WASM adapters | 20 |
| [`nis2-mcp-server`](crates/mcp-server/) | Model Context Protocol server (MCP 2024-11-05 spec) | 13 |
| [`nis2-slm`](crates/slm/) | Constrained report generation, GBNF grammar, prompt engineering | 18 |
| [`nis2-api`](crates/api/) | REST API layer (axum), benchmark suite | 6 |

## Quick Start

### Prerequisites

- **Rust** ≥ 1.85 (2024 edition)
- ~500MB RAM for ONNX Runtime (BGE-Small model)

### Build & Test

```bash
# Clone
git clone https://github.com/fabriziosalmi/nis2-model.git
cd nis2-model

# Build all crates
cargo build --workspace

# Run all 61 tests
cargo test --workspace
```

### REST API Server

```bash
# Start API on http://localhost:8080
cargo run --bin api-server

# Custom port
cargo run --bin api-server -- --port 3000
```

### Index Legal Texts

```bash
# Populate the LanceDB vector store (49 chunks: 35 NIS2 + 14 DORA)
cargo run --bin indexer
```

### Semantic Search

```bash
# Search for relevant legal provisions
cargo run --bin search -- "autenticazione multi-fattore"
# → Art. 21(2)(j): uso di soluzioni di autenticazione a più fattori...

cargo run --bin search -- "obblighi segnalazione incidenti"
# → Art. 21(2)(b): gestione degli incidenti
# → Art. 23(4)(b): notifica entro 72 ore
```

### Generate Compliance Report

```bash
# Full Italian compliance report from template engine
cargo run --bin report

# Raw JSON output (programmatic)
cargo run --bin demo
```

### MCP Server

```bash
# Start MCP server (JSON-RPC over stdio)
cargo run --bin mcp-server

# Example: send a tool call
echo '{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"verifica_applicabilita","arguments":{"settore":"energy","dipendenti":120,"fatturato_mln_eur":50.0}}}' \
  | cargo run --bin mcp-server 2>/dev/null
```

## MCP Integration

The engine exposes 4 MCP Tools via JSON-RPC 2.0:

| Tool | Description |
|------|-------------|
| `verifica_applicabilita` | Check NIS2 applicability (sector + size → in-scope/out-of-scope) |
| `calcola_sanzione` | Calculate Art. 34 maximum penalty |
| `lista_obblighi` | List all 16 applicable obligations with legal text |
| `valuta_compliance` | Full compliance evaluation (JSON output) |

### Claude Desktop Configuration

```json
{
  "mcpServers": {
    "nis2-compliance": {
      "command": "cargo",
      "args": ["run", "--bin", "mcp-server", "--manifest-path", "/path/to/nis2-model/Cargo.toml"]
    }
  }
}
```

## Rule Engine

The rule engine implements deterministic boolean decision trees:

```
Input: CompanyProfile { sector, employees, revenue }
  │
  ├─ Annex I sector + size threshold → Essential (Art. 3(1))
  ├─ Annex II sector + size threshold → Important (Art. 3(2))
  ├─ Digital infra (any size) → Essential (Art. 2(2))
  └─ Otherwise → Out of Scope
  │
  ├─ 16 obligations mapped (Art. 20, 21, 23)
  ├─ Sanction calculation (Art. 34)
  └─ Incident deadlines: 24h / 72h / 30d (Art. 23(4))
```

All inputs and outputs are validated against auto-generated JSON Schema at runtime.

## Zero-Hallucination Guarantee

The report generator enforces:

- Every article reference derived from the obligation catalog
- Every monetary figure calculated by the rule engine
- Forbidden-word filter: *"potrebbe"*, *"forse"*, *"probabilmente"* — blocked
- Deterministic formula: *"Sulla base dei dati forniti, risulta che…"*
- GBNF grammar constrains token-level output to legal citation patterns

## Testing

```bash
# Full suite (61 tests)
cargo test --workspace

# Individual crates
cargo test -p nis2-rules        # 20 tests
cargo test -p nis2-slm          # 18 tests
cargo test -p nis2-mcp-server   # 13 tests
cargo test -p nis2-api          #  6 tests
cargo test -p nis2-ingestion    #  3 tests
cargo test -p nis2-vectordb     #  1 test
```

### Benchmark

```bash
cargo run --bin bench --release
```

## Project Structure

```
nis2-model/
├── Cargo.toml              # Workspace manifest
├── crates/
│   ├── ingestion/          # Legal text parser + semantic chunker
│   ├── vectordb/           # LanceDB + fastembed embeddings
│   ├── rules/              # Deterministic rule engine
│   │   ├── engine.rs       # Applicability + sanctions
│   │   ├── obligations.rs  # Art. 20/21/23 obligation catalog
│   │   ├── validation.rs   # JSON Schema validation
│   │   └── adapters.rs     # Extism WASM adapters
│   ├── mcp-server/         # MCP JSON-RPC server
│   │   ├── protocol.rs     # MCP protocol types
│   │   ├── tools.rs        # Tool implementations
│   │   └── server.rs       # stdio transport
│   ├── slm/                # SLM inference engine
│   │   ├── prompt.rs       # ChatML prompt engineering
│   │   ├── grammar.rs      # GBNF constrained decoding
│   │   ├── inference.rs    # Backend trait + template engine
│   │   └── report.rs       # Italian report generator
│   └── api/                # REST API (axum) + benchmark suite
├── data/
│   └── sources/            # NIS2 + DORA articles (JSON)
├── docs/                   # VitePress documentation
└── mcp_config.json         # MCP client configuration
```

## Legal References

- **NIS2**: Directive (EU) 2022/2555 — [EUR-Lex](https://eur-lex.europa.eu/eli/dir/2022/2555)
- **DORA**: Regulation (EU) 2022/2554 — [EUR-Lex](https://eur-lex.europa.eu/eli/reg/2022/2554)
- **EU Recommendation 2003/361/EC** — SME size thresholds

## License

Licensed under the [European Union Public License 1.2](https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12) (EUPL-1.2).
