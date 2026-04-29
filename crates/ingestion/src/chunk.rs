//! Semantic chunking — splits structured legal text into atomic [`Chunk`]s.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::types::{Chunk, LegalReference};

/// Generates a deterministic ID for a legal reference.
fn chunk_id(reference: &LegalReference) -> String {
    let mut hasher = DefaultHasher::new();
    reference.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

/// Creates a [`Chunk`] from a legal reference and its text content.
pub fn make_chunk(reference: LegalReference, text: impl Into<String>) -> Chunk {
    let id = chunk_id(&reference);
    Chunk {
        id,
        reference,
        text: text.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Directive;

    #[test]
    fn chunk_id_is_deterministic() {
        let r = LegalReference {
            directive: Directive::Nis2,
            article: 21,
            paragraph: 2,
            letter: Some('j'),
        };
        let c1 = make_chunk(r.clone(), "test");
        let c2 = make_chunk(r, "test");
        assert_eq!(c1.id, c2.id);
    }

    #[test]
    fn different_references_produce_different_ids() {
        let r1 = LegalReference {
            directive: Directive::Nis2,
            article: 21,
            paragraph: 2,
            letter: Some('j'),
        };
        let r2 = LegalReference {
            directive: Directive::Dora,
            article: 21,
            paragraph: 2,
            letter: Some('j'),
        };
        let c1 = make_chunk(r1, "test");
        let c2 = make_chunk(r2, "test");
        assert_ne!(c1.id, c2.id);
    }
}
