//! # nis2-ingestion
//!
//! PDF/HTML parsing and semantic chunking of EU legal texts.
//! Each chunk represents one atomic legal unit (paragraph/comma).

pub mod chunk;
pub mod parser;
pub mod types;

pub use types::{Chunk, Directive, LegalReference};
