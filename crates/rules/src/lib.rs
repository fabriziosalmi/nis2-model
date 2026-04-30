//! # nis2-rules
//!
//! Deterministic rule engine for NIS2/DORA compliance evaluation.
//! JSON Schema validated boolean decision trees — zero ambiguity.
//!
//! ## Modules
//! - [`schema`] — Type definitions with JSON Schema derivation
//! - [`engine`] — Core evaluation logic (applicability, classification, sanctions)
//! - [`obligations`] — NIS2 Art. 20/21/23 obligation catalog
//! - [`validation`] — Runtime JSON Schema validation
//! - [`adapters`] — Extism WASM plugin adapters

#[cfg(not(target_arch = "wasm32"))]
pub mod adapters;
pub mod engine;
pub mod obligations;
pub mod schema;
pub mod validation;

#[cfg(target_arch = "wasm32")]
pub mod wasm;
