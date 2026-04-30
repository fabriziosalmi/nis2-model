<div align="center">

# NIS2 Compliance Assistant

**A deterministic compliance engine and web interface for the NIS2 Directive and DORA Regulation.**

[![License: EUPL-1.2](https://img.shields.io/badge/License-EUPL--1.2-blue.svg)](https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12)
[![Rust](https://img.shields.io/badge/Rust-2024_Edition-orange.svg)](https://www.rust-lang.org/)
[![Tests](https://img.shields.io/badge/Tests-61%20passing-brightgreen.svg)](#testing)

</div>

---

Welcome to the NIS2 Compliance Assistant. This project is a hybrid platform that unifies a deterministic rules engine with an interactive web application.

It is designed to evaluate organizational data against legal texts using a strict, pre-compiled Rust logic engine, providing consistent and repeatable compliance assessments.

## Architecture

This project consists of two tightly integrated layers:

### 1. The Interactive Assistant (Web UI)
A browser-based application deployed via VitePress. 
- **Semantic Chatbot:** Driven by a pre-computed dataset of verified Q&A entries across 5 languages.
- **Guided Assessment:** Interactive paths to map an organization's profile directly against legal requirements.
- **Local Evaluation:** Evaluates rules locally and securely within the browser via WebAssembly. No compliance data is transmitted to external providers.

### 2. The Deterministic Engine (Rust Core)
The core logic of the platform. A Rust workspace that models the compliance constraints of the [NIS2 Directive (EU 2022/2555)](https://eur-lex.europa.eu/eli/dir/2022/2555) and indexes the [DORA Regulation (EU 2022/2554)](https://eur-lex.europa.eu/eli/reg/2022/2554).
- **Rule Evaluation:** Given a company profile (sector, size, revenue), it evaluates applicability, maps the 16 core obligations, and calculates maximum sanctions.
- **Integration Ready:** Exposes the logic via WebAssembly (WASM), REST APIs, an MCP (Model Context Protocol) server, and CLI reporting tools.

---

## For Developers: The Rust Engine

The workspace is composed of several specialized crates:

| Crate | Purpose | Tests |
|-------|---------|-------|
| [`nis2-ingestion`](crates/ingestion/) | Parses `data/sources/*.json` into semantic chunks (35 NIS2 + 14 DORA) | 3 |
| [`nis2-vectordb`](crates/vectordb/) | Indexes chunks in LanceDB with BGE-Small embeddings (384-dim, HNSW) | 1 |
| [`nis2-rules`](crates/rules/) | Boolean decision trees: applicability, obligations, Art. 34 sanctions | 20 |
| [`nis2-mcp-server`](crates/mcp-server/) | Exposes 4 assessment tools via MCP JSON-RPC 2.0 over stdio | 13 |
| [`nis2-slm`](crates/slm/) | Generates localized compliance reports via template substitution | 18 |
| [`nis2-api`](crates/api/) | Axum REST API endpoints and benchmark binary | 6 |
| [`nis2-chat`](crates/chat/) | Dataset generation and tools for the web interface | 1 |

### Build and Test

```bash
cargo build --workspace
cargo test --workspace
```

### Rule Engine Logic

```text
CompanyProfile { sector, employees, annual_revenue_eur_m }
  |
  +-- Annex I sector (11) + size >= medium --> Essential (Art. 3(1))
  +-- Annex II sector (7) + size >= medium --> Important (Art. 3(2))
  +-- digital_infrastructure / ict_service_management_b2b / public_administration --> Essential (any size)
  +-- otherwise --> OutOfScope
  |
  +-- Maps 16 obligations: Art. 20 (2) + Art. 21(2)(a-j) (10) + Art. 23 (4)
  +-- Sanction: Essential = max(10M, 2% revenue) | Important = max(7M, 1.4% revenue)
  +-- Incident deadlines: 24h / 72h / 30d
```
*Note: Size threshold evaluates 50+ employees OR 10M+ EUR revenue (EU Rec. 2003/361/EC).*

### Binaries and Tools

| Binary | Command | What it does |
|--------|---------|-------------|
| `indexer` | `cargo run --bin indexer` | Populates LanceDB from `data/sources/` |
| `search` | `cargo run --bin search -- "query"` | Semantic search over indexed provisions |
| `demo` | `cargo run --bin demo` | Prints ComplianceStatus JSON |
| `report` | `cargo run --bin report` | Generates localized markdown compliance report |
| `mcp-server` | `cargo run --bin mcp-server` | MCP JSON-RPC server (stdio) |
| `api-server` | `cargo run --bin api-server` | HTTP server on port 8080 |
| `export_dataset`| `cargo run --bin export_dataset`| Exports Q&A dataset for VitePress |
| `bench` | `cargo run --bin bench --release` | Benchmark suite |

---

## Limitations

This tool evaluates a subset of the NIS2 Directive. Before relying on its outputs, understand these constraints:

| Area | Status |
|------|--------|
| Articles covered | Art. 2, 3, 20, 21, 23, 34 |
| Articles NOT covered | Art. 7-9, 13-14, 26-29, 32-33, 35 |
| National transpositions | Not implemented (e.g. Italian D.Lgs. 138/2024) — EU Directive only |
| Size thresholds | Simplified (employees OR revenue) — linked/partner enterprise rules not implemented |
| Sub-sector / Balance Sheet | Accepted as input but not used for deterministic logic |

---

## Legal Disclaimer

> **This tool does NOT provide legal advice.** All outputs are automated classifications generated by deterministic software. They do not replace consultation with a qualified legal professional. See the full [Terms of Use](docs/legal/terms.md) and [Legal Disclaimer](docs/legal/disclaimer.md).

## References

- NIS2: Directive (EU) 2022/2555 -- [EUR-Lex](https://eur-lex.europa.eu/eli/dir/2022/2555)
- DORA: Regulation (EU) 2022/2554 -- [EUR-Lex](https://eur-lex.europa.eu/eli/reg/2022/2554)
- Size thresholds: EU Recommendation 2003/361/EC

## License

[EUPL-1.2](https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12)
