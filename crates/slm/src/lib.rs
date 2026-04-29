//! # nis2-slm
//!
//! Local SLM inference with grammar-constrained generation.
//! Transforms structured compliance JSON into natural-language Italian reports.
//!
//! ## Architecture
//! - [`prompt`] — System prompt and template construction
//! - [`grammar`] — GBNF grammar for constrained decoding
//! - [`inference`] — Inference trait + backends
//! - [`report`] — Deterministic report generator (template-based fallback)

pub mod grammar;
pub mod inference;
pub mod prompt;
pub mod report;
