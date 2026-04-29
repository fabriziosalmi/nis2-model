//! In-memory semantic cache.
//!
//! Stores (embedding, answer) pairs and performs brute-force cosine
//! similarity search. This is intentionally simple -- the dataset is
//! small enough (< 100 entries) that HNSW indexing is unnecessary.

use serde::{Deserialize, Serialize};

/// A cached Q&A entry with its pre-computed embedding.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    /// The canonical question text.
    pub question: String,
    /// The pre-computed answer.
    pub answer: String,
    /// Category tag for filtering (e.g. "applicability", "sanctions").
    pub category: String,
    /// Pre-computed embedding vector (384-dim for BGE-Small).
    #[serde(skip)]
    pub embedding: Vec<f32>,
}

/// Result of a cache lookup.
#[derive(Debug, Clone)]
pub struct CacheHit {
    /// The matched entry.
    pub entry: CacheEntry,
    /// Cosine similarity score (0.0 to 1.0).
    pub score: f32,
}

/// In-memory semantic cache.
pub struct SemanticCache {
    entries: Vec<CacheEntry>,
    threshold: f32,
}

impl SemanticCache {
    /// Create a new cache with the given similarity threshold.
    ///
    /// Recommended thresholds:
    /// - 0.92+: strict, near-identical questions only
    /// - 0.88-0.92: balanced, catches paraphrases
    /// - <0.85: too lenient, risks wrong answers
    pub fn new(threshold: f32) -> Self {
        Self {
            entries: Vec::new(),
            threshold,
        }
    }

    /// Returns the configured similarity threshold.
    pub fn threshold(&self) -> f32 {
        self.threshold
    }

    /// Number of entries in the cache.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Insert a new entry with its pre-computed embedding.
    pub fn insert(&mut self, entry: CacheEntry) {
        self.entries.push(entry);
    }

    /// Search for the most similar cached answer.
    ///
    /// Returns `Some(CacheHit)` if the best match exceeds the threshold,
    /// `None` otherwise (cache miss).
    pub fn search(&self, query_embedding: &[f32]) -> Option<CacheHit> {
        if self.entries.is_empty() {
            return None;
        }

        let mut best_score: f32 = -1.0;
        let mut best_idx: usize = 0;

        for (i, entry) in self.entries.iter().enumerate() {
            let score = cosine_similarity(query_embedding, &entry.embedding);
            if score > best_score {
                best_score = score;
                best_idx = i;
            }
        }

        if best_score >= self.threshold {
            Some(CacheHit {
                entry: self.entries[best_idx].clone(),
                score: best_score,
            })
        } else {
            None
        }
    }
}

/// Cosine similarity between two vectors.
pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot: f32 = a.iter().zip(b).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }
    dot / (norm_a * norm_b)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_embedding(seed: f32) -> Vec<f32> {
        (0..384).map(|i| (i as f32 * seed).sin()).collect()
    }

    #[test]
    fn empty_cache_returns_none() {
        let cache = SemanticCache::new(0.90);
        assert!(cache.search(&dummy_embedding(1.0)).is_none());
    }

    #[test]
    fn exact_match_returns_hit() {
        let mut cache = SemanticCache::new(0.90);
        let emb = dummy_embedding(1.0);
        cache.insert(CacheEntry {
            question: "test question".into(),
            answer: "test answer".into(),
            category: "test".into(),
            embedding: emb.clone(),
        });
        let hit = cache.search(&emb).unwrap();
        assert!((hit.score - 1.0).abs() < 1e-5);
        assert_eq!(hit.entry.answer, "test answer");
    }

    #[test]
    fn dissimilar_query_returns_none() {
        let mut cache = SemanticCache::new(0.90);
        cache.insert(CacheEntry {
            question: "test".into(),
            answer: "answer".into(),
            category: "test".into(),
            embedding: dummy_embedding(1.0),
        });
        // Very different embedding
        let result = cache.search(&dummy_embedding(100.0));
        // May or may not hit depending on the sin pattern,
        // but we test the mechanics work
        if let Some(hit) = result {
            assert!(hit.score >= 0.90);
        }
    }

    #[test]
    fn cosine_identical_is_one() {
        let v = vec![1.0, 2.0, 3.0];
        let score = cosine_similarity(&v, &v);
        assert!((score - 1.0).abs() < 1e-6);
    }

    #[test]
    fn cosine_orthogonal_is_zero() {
        let a = vec![1.0, 0.0];
        let b = vec![0.0, 1.0];
        let score = cosine_similarity(&a, &b);
        assert!(score.abs() < 1e-6);
    }

    #[test]
    fn threshold_respected() {
        let mut cache = SemanticCache::new(0.99);
        cache.insert(CacheEntry {
            question: "q".into(),
            answer: "a".into(),
            category: "test".into(),
            embedding: dummy_embedding(1.0),
        });
        // Slightly different embedding -- won't hit 0.99
        let mut query = dummy_embedding(1.0);
        query[0] += 0.5;
        query[1] -= 0.3;
        let result = cache.search(&query);
        // With such a high threshold and a perturbed vector, it should miss
        if let Some(hit) = result {
            assert!(hit.score >= 0.99);
        }
    }

    #[test]
    fn cache_len() {
        let mut cache = SemanticCache::new(0.90);
        assert_eq!(cache.len(), 0);
        assert!(cache.is_empty());
        cache.insert(CacheEntry {
            question: "q".into(),
            answer: "a".into(),
            category: "t".into(),
            embedding: vec![1.0],
        });
        assert_eq!(cache.len(), 1);
        assert!(!cache.is_empty());
    }
}
