<div align="center">

# nis2-model

**Reference implementation for NIS2/DORA compliance logic in Rust**

[![License: EUPL-1.2](https://img.shields.io/badge/License-EUPL--1.2-blue.svg)](https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12)
[![Rust](https://img.shields.io/badge/Rust-2024_Edition-orange.svg)](https://www.rust-lang.org/)
[![Tests](https://img.shields.io/badge/Tests-61%20passing-brightgreen.svg)](#testing)

</div>

---

A Rust workspace that models the compliance logic of the [NIS2 Directive (EU 2022/2555)](https://eur-lex.europa.eu/eli/dir/2022/2555) and indexes provisions from the [DORA Regulation (EU 2022/2554)](https://eur-lex.europa.eu/eli/reg/2022/2554).

Given a company profile (sector, size, revenue), it evaluates applicability, maps obligations, calculates sanctions, and generates a structured report. All logic runs locally with no external dependencies.

## What is in this repo

| Crate | What it does | Tests |
|-------|-------------|-------|
| [`nis2-ingestion`](crates/ingestion/) | Parses `data/sources/*.json` into semantic chunks (35 NIS2 + 14 DORA) | 3 |
| [`nis2-vectordb`](crates/vectordb/) | Indexes chunks in LanceDB with BGE-Small embeddings (384-dim, HNSW) | 1 |
| [`nis2-rules`](crates/rules/) | Boolean decision trees: applicability, 16 obligations, Art. 34 sanctions | 20 |
| [`nis2-mcp-server`](crates/mcp-server/) | Exposes 4 tools via MCP JSON-RPC 2.0 over stdio | 13 |
| [`nis2-slm`](crates/slm/) | Generates Italian reports from ComplianceStatus via template substitution | 18 |
| [`nis2-api`](crates/api/) | Axum REST API with 6 endpoints, benchmark binary | 6 |

## Build and test

```bash
cargo build --workspace
cargo test --workspace   # 61 tests
```

## Binaries

| Binary | Command | What it does |
|--------|---------|-------------|
| `indexer` | `cargo run --bin indexer` | Populates LanceDB from `data/sources/` |
| `search` | `cargo run --bin search -- "query"` | Semantic search over indexed provisions |
| `demo` | `cargo run --bin demo` | Prints ComplianceStatus JSON |
| `report` | `cargo run --bin report` | Generates Italian compliance report |
| `mcp-server` | `cargo run --bin mcp-server` | MCP JSON-RPC server (stdio) |
| `api-server` | `cargo run --bin api-server` | HTTP server on port 8080 |
| `bench` | `cargo run --bin bench --release` | Benchmark suite |

## MCP tools

4 tools available when using the MCP server:

| Tool | Input | Output |
|------|-------|--------|
| `verifica_applicabilita` | sector, employees, revenue | Applicability + classification |
| `calcola_sanzione` | sector, employees, revenue | Art. 34 maximum sanction |
| `lista_obblighi` | sector, employees, revenue | 16 obligations with legal text |
| `valuta_compliance` | full company profile (8 fields) | Complete ComplianceStatus JSON |

## REST API endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | `/api/v1/health` | Status, version |
| POST | `/api/v1/evaluate` | Full ComplianceStatus from CompanyProfile |
| POST | `/api/v1/applicability` | Applicability check (3 fields) |
| POST | `/api/v1/sanctions` | Art. 34 sanction calculation |
| POST | `/api/v1/obligations` | List of 16 obligations |
| POST | `/api/v1/report` | Italian markdown report |

## Rule engine logic

```
CompanyProfile { sector, employees, annual_revenue_eur_m }
  |
  +-- Annex I sector (11) + size >= medium --> Essential (Art. 3(1))
  +-- Annex II sector (7) + size >= medium --> Important (Art. 3(2))
  +-- digital_infrastructure / ict_service_management_b2b / public_administration --> Essential (any size)
  +-- otherwise --> OutOfScope
  |
  +-- 16 obligations: Art. 20 (2) + Art. 21(2)(a-j) (10) + Art. 23 (4)
  +-- Sanction: Essential = max(10M, 2% revenue) | Important = max(7M, 1.4% revenue)
  +-- Incident deadlines: 24h / 72h / 30d
```

Size threshold: 50+ employees OR 10M+ EUR revenue (EU Rec. 2003/361/EC).

## Testing

```bash
cargo test -p nis2-rules        # 20 tests
cargo test -p nis2-slm          # 18 tests
cargo test -p nis2-mcp-server   # 13 tests
cargo test -p nis2-api          #  6 tests
cargo test -p nis2-ingestion    #  3 tests
cargo test -p nis2-vectordb     #  1 test
```

## Project structure

```
nis2-model/
├── Cargo.toml
├── crates/
│   ├── ingestion/    chunk.rs, parser.rs, types.rs
│   ├── vectordb/     embed.rs, lance.rs, store.rs
│   ├── rules/        engine.rs, obligations.rs, schema.rs, validation.rs, adapters.rs
│   ├── mcp-server/   protocol.rs, tools.rs, server.rs
│   ├── slm/          prompt.rs, grammar.rs, inference.rs, report.rs
│   └── api/          handlers.rs, routes.rs
├── data/sources/     nis2_articles.json, dora_articles.json
└── docs/             VitePress documentation site
```

## References

- NIS2: Directive (EU) 2022/2555 -- [EUR-Lex](https://eur-lex.europa.eu/eli/dir/2022/2555)
- DORA: Regulation (EU) 2022/2554 -- [EUR-Lex](https://eur-lex.europa.eu/eli/reg/2022/2554)
- Size thresholds: EU Recommendation 2003/361/EC

## License

[EUPL-1.2](https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12)
