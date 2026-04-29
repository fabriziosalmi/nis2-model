# Ingestion Pipeline

**Crate**: `nis2-ingestion` | **Files**: `chunk.rs`, `parser.rs`, `types.rs` | **Tests**: 3

## What it does

Parses structured legal JSON files and splits them into semantic chunks with deterministic IDs.

## Types

`LegalReference` identifies a provision by directive, article, paragraph, and optional letter. `Chunk` pairs a `LegalReference` with its text content and a SHA-256 hash ID.

## Data sources

| File | Directive | Chunks |
|------|-----------|--------|
| `data/sources/nis2_articles.json` | NIS2 (EU 2022/2555) | 35 |
| `data/sources/dora_articles.json` | DORA (EU 2022/2554) | 14 |

## Tests

- `chunk_id_is_deterministic` -- same reference always produces the same hash
- `different_references_produce_different_ids` -- uniqueness guarantee
- `parse_json_articles` -- full parse roundtrip from JSON source files
