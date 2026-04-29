//! # nis2-mcp-server
//!
//! Model Context Protocol server implementing the Anthropic MCP specification.
//! Exposes the deterministic NIS2 rule engine as callable Tools over JSON-RPC 2.0.
//!
//! ## Transport
//! - **stdio** (default): reads JSON-RPC from stdin, writes to stdout
//!
//! ## Tools
//! - `verifica_applicabilita` — Check if NIS2 applies to a company
//! - `calcola_sanzione` — Calculate maximum sanction
//! - `lista_obblighi` — List all applicable obligations
//! - `valuta_compliance` — Full compliance evaluation

pub mod protocol;
pub mod server;
pub mod tools;
