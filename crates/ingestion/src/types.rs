use serde::{Deserialize, Serialize};

/// Identifies which EU directive a legal provision belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Directive {
    /// Directive (EU) 2022/2555 — Network and Information Security
    Nis2,
    /// Regulation (EU) 2022/2554 — Digital Operational Resilience Act
    Dora,
}

impl std::fmt::Display for Directive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Directive::Nis2 => write!(f, "NIS2 (2022/2555)"),
            Directive::Dora => write!(f, "DORA (2022/2554)"),
        }
    }
}

/// A precise reference to a legal provision.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LegalReference {
    pub directive: Directive,
    pub article: u16,
    pub paragraph: u16,
    /// Optional lettered sub-point (a, b, c, …).
    pub letter: Option<char>,
}

impl std::fmt::Display for LegalReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, Art. {}, par. {}",
            self.directive, self.article, self.paragraph
        )?;
        if let Some(letter) = self.letter {
            write!(f, ", lett. {letter})")?;
        }
        Ok(())
    }
}

/// A single semantic chunk — the atomic unit for embedding and retrieval.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    /// Unique identifier (deterministic hash of reference).
    pub id: String,
    /// Full legal provenance.
    pub reference: LegalReference,
    /// Raw text content of this provision.
    pub text: String,
}
