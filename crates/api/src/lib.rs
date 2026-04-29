//! # nis2-api
//!
//! REST API layer (axum) -- HTTP interface to the NIS2 compliance pipeline.
//! Provides JSON endpoints for compliance evaluation, obligation listing,
//! sanction calculation, and report generation.

pub mod routes;
pub mod handlers;
