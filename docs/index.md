---
layout: home

hero:
  name: nis2-model
  text: Reference implementation for NIS2/DORA compliance logic
  tagline: Rust workspace. 6 crates. 61 tests. Fully local.
  actions:
    - theme: brand
      text: Getting Started
      link: /guide/getting-started
    - theme: alt
      text: Architecture
      link: /architecture/overview
    - theme: alt
      text: GitHub
      link: https://github.com/fabriziosalmi/nis2-model

features:
  - title: NIS2 and DORA Coverage
    details: Implements applicability checks (Art. 2/3), 16 obligations (Art. 20/21/23), sanctions (Art. 34), and incident reporting deadlines for EU Directive 2022/2555.
  - title: Deterministic Output
    details: Every output is derived from structured data via boolean decision trees. The template backend maps compliance JSON to report text with no generative model involved.
  - title: Fully Local
    details: No cloud APIs, no keys, no data exfiltration. BGE-Small embeddings via ONNX Runtime, LanceDB embedded storage, all inference on-device.
  - title: Native Performance
    details: Written in Rust 2024 edition. 6-crate workspace. 61 tests. Compiles to a single native binary per target.
  - title: MCP Integration
    details: 4 tools exposed via Model Context Protocol (JSON-RPC 2.0 over stdio). Compatible with Claude Desktop and any MCP client.
  - title: REST API
    details: Axum HTTP server with 6 endpoints. Health check, full evaluation, applicability, sanctions, obligations, and report generation.
---
