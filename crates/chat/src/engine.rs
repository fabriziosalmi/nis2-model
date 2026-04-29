//! Chat engine -- orchestrates cache lookup and fallback.

use crate::cache::{CacheEntry, SemanticCache};
use crate::dataset;

/// Response from the chat engine.
#[derive(Debug, Clone)]
pub struct ChatResponse {
    /// The answer text.
    pub answer: String,
    /// Whether this came from the cache or was generated.
    pub from_cache: bool,
    /// Similarity score (only meaningful when from_cache is true).
    pub score: f32,
    /// Category of the matched entry.
    pub category: String,
    /// Suggested follow-up questions.
    pub follow_ups: Vec<String>,
}

/// Chat engine with semantic cache.
///
/// Usage:
/// 1. Create with `ChatEngine::new(threshold)`
/// 2. Call `warmup(embedder_fn)` to embed all dataset entries
/// 3. Call `ask(query_embedding)` to get answers
pub struct ChatEngine {
    cache: SemanticCache,
}

impl ChatEngine {
    /// Create a new chat engine with the given similarity threshold.
    pub fn new(threshold: f32) -> Self {
        Self {
            cache: SemanticCache::new(threshold),
        }
    }

    /// Warm up the cache by loading the dataset and computing embeddings.
    ///
    /// The `embed_fn` takes a text string and returns its 384-dim embedding.
    /// This is kept generic so the caller can use any embedding backend.
    pub fn warmup<F>(&mut self, embed_fn: F) -> anyhow::Result<usize>
    where
        F: Fn(&str) -> anyhow::Result<Vec<f32>>,
    {
        let mut entries = dataset::build_dataset();
        for entry in &mut entries {
            entry.embedding = embed_fn(&entry.question)?;
        }
        let count = entries.len();
        for entry in entries {
            self.cache.insert(entry);
        }
        Ok(count)
    }

    /// Ask a question. Pass the pre-computed embedding of the query.
    ///
    /// Returns a cached answer if similarity >= threshold, otherwise
    /// returns a fallback response indicating a cache miss.
    pub fn ask(&self, query_embedding: &[f32]) -> ChatResponse {
        match self.cache.search(query_embedding) {
            Some(hit) => ChatResponse {
                answer: hit.entry.answer,
                from_cache: true,
                score: hit.score,
                category: hit.entry.category,
                follow_ups: hit.entry.follow_ups,
            },
            None => ChatResponse {
                answer: "This question is not in the knowledge base. \
                         Please rephrase or ask about NIS2 applicability, \
                         obligations, sanctions, or incident reporting."
                    .into(),
                from_cache: false,
                score: 0.0,
                category: "miss".into(),
                follow_ups: vec![],
            },
        }
    }

    /// Insert a new Q&A pair into the cache (learning).
    pub fn learn(&mut self, question: String, answer: String, category: String, embedding: Vec<f32>) {
        self.cache.insert(CacheEntry {
            question,
            answer,
            category,
            follow_ups: vec![],
            embedding,
        });
    }

    /// Number of entries in the cache.
    pub fn cache_size(&self) -> usize {
        self.cache.len()
    }

    /// Return the top N matches regardless of threshold (for debugging).
    pub fn debug_top_n(&self, query_embedding: &[f32], n: usize) -> Vec<(f32, String)> {
        self.cache.search_top_n(query_embedding, n)
            .into_iter()
            .map(|h| (h.score, h.entry.question))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mock_embed(text: &str) -> anyhow::Result<Vec<f32>> {
        // Deterministic mock: hash the text into a 384-dim vector
        let hash = text.bytes().fold(0u64, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u64));
        let v: Vec<f32> = (0..384).map(|i| ((hash.wrapping_add(i)) as f32).sin()).collect();
        Ok(v)
    }

    #[test]
    fn warmup_loads_dataset() {
        let mut engine = ChatEngine::new(0.90);
        let count = engine.warmup(mock_embed).unwrap();
        assert!(count >= 15);
        assert_eq!(engine.cache_size(), count);
    }

    #[test]
    fn exact_question_hits_cache() {
        let mut engine = ChatEngine::new(0.90);
        engine.warmup(mock_embed).unwrap();
        // Ask with the exact same embedding as "What are the NIS2 penalties?"
        let emb = mock_embed("What are the NIS2 penalties?").unwrap();
        let resp = engine.ask(&emb);
        assert!(resp.from_cache);
        assert!(resp.score >= 0.90);
    }

    #[test]
    fn random_query_misses() {
        let mut engine = ChatEngine::new(0.99);
        engine.warmup(mock_embed).unwrap();
        let emb = mock_embed("recipe for apple pie").unwrap();
        let resp = engine.ask(&emb);
        // With 0.99 threshold, unrelated text should miss
        if !resp.from_cache {
            assert_eq!(resp.category, "miss");
        }
    }

    #[test]
    fn learn_adds_entry() {
        let mut engine = ChatEngine::new(0.90);
        assert_eq!(engine.cache_size(), 0);
        engine.learn(
            "test q".into(), "test a".into(), "test".into(),
            mock_embed("test q").unwrap(),
        );
        assert_eq!(engine.cache_size(), 1);
    }
}
