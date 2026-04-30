//! # nis2-chat
//!
//! Semantic Q&A cache for NIS2 compliance questions.
//!
//! Pre-computes answers to common questions and serves them via cosine
//! similarity matching. When no cached answer is close enough (below the
//! similarity threshold), falls back to the deterministic rule engine +
//! template report generator.

pub mod cache;
pub mod dataset;
pub mod it;

pub mod dataset_legal;
pub mod dataset_ops;
pub mod dataset_scenarios;
pub mod engine;
