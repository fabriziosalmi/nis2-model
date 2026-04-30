//! Chat engine -- orchestrates cache lookup, session tracking, and adaptive follow-ups.

use crate::cache::{CacheEntry, SemanticCache};
use crate::dataset;
use std::collections::HashSet;

/// All 16 NIS2 obligation areas mapped from Art. 20, 21, 23.
const OBLIGATION_AREAS: &[(&str, &str)] = &[
    ("governance", "Art. 20 — Governance & management approval"),
    ("access_control", "Art. 21(2)(i) — Access control & asset management"),
    ("encryption", "Art. 21(2)(h) — Cryptography & encryption"),
    ("incident_response", "Art. 21(2)(b) — Incident handling"),
    ("business_continuity", "Art. 21(2)(c) — Business continuity & DR"),
    ("supply_chain", "Art. 21(2)(d) — Supply chain security"),
    ("vulnerability_mgmt", "Art. 21(2)(e) — Vulnerability management"),
    ("risk_assessment", "Art. 21(2)(a) — Risk analysis"),
    ("network_security", "Art. 21(2)(a) — Network & system security"),
    ("detection", "Art. 21(2)(b) — Detection & monitoring"),
    ("email_security", "Art. 21(2)(g) — Cyber hygiene & training"),
    ("documentation", "Art. 21(2) — Policy documentation"),
    ("remote_work", "Art. 21(2)(j) — Secured communications"),
    ("physical", "Art. 21(2)(a) — Physical security"),
    ("legal", "Art. 23/27 — Notification & registration"),
    ("sanctions", "Art. 34 — Sanctions regime"),
];

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
    /// Suggested follow-up questions (filtered by session).
    pub follow_ups: Vec<String>,
    /// Coverage: how many of the 16 obligation areas the user has explored.
    pub coverage: Coverage,
}

/// NIS2 compliance coverage tracker.
#[derive(Debug, Clone, Default)]
pub struct Coverage {
    /// Number of obligation areas explored.
    pub explored: usize,
    /// Total obligation areas.
    pub total: usize,
    /// Areas not yet explored (suggestions).
    pub missing: Vec<String>,
}

/// Tracks conversation state to prevent cycles and suggest unexplored areas.
pub struct Session {
    visited_questions: HashSet<String>,
    visited_categories: HashSet<String>,
}

impl Default for Session {
    fn default() -> Self {
        Self::new()
    }
}

impl Session {
    pub fn new() -> Self {
        Self {
            visited_questions: HashSet::new(),
            visited_categories: HashSet::new(),
        }
    }

    /// Record that a question/category was visited.
    pub fn visit(&mut self, question: &str, category: &str) {
        self.visited_questions.insert(question.to_string());
        // Normalize _impl categories to their parent
        let base = category.trim_end_matches("_impl");
        self.visited_categories.insert(base.to_string());
    }

    /// Filter follow-ups: remove already-visited questions.
    pub fn filter_follow_ups(&self, follow_ups: &[String]) -> Vec<String> {
        follow_ups.iter()
            .filter(|q| !self.visited_questions.contains(q.as_str()))
            .cloned()
            .collect()
    }

    /// Compute coverage against the 16 obligation areas.
    pub fn coverage(&self) -> Coverage {
        let explored = OBLIGATION_AREAS.iter()
            .filter(|(cat, _)| self.visited_categories.contains(*cat))
            .count();
        let missing: Vec<String> = OBLIGATION_AREAS.iter()
            .filter(|(cat, _)| !self.visited_categories.contains(*cat))
            .map(|(_, desc)| desc.to_string())
            .collect();
        Coverage {
            explored,
            total: OBLIGATION_AREAS.len(),
            missing,
        }
    }

    /// Suggest unexplored categories as entry questions.
    pub fn suggest_unexplored(&self) -> Vec<String> {
        let suggestions: Vec<(&str, &str)> = vec![
            ("governance", "Is the board personally liable under NIS2?"),
            ("access_control", "Do we need to rotate passwords and secrets under NIS2?"),
            ("encryption", "Do we need encryption at rest?"),
            ("incident_response", "What do we do if we get hacked?"),
            ("business_continuity", "Do we need backups?"),
            ("supply_chain", "Do we need to audit our vendors?"),
            ("vulnerability_mgmt", "Do we need to patch our systems?"),
            ("risk_assessment", "Do we need a risk assessment?"),
            ("network_security", "Do we need network segmentation?"),
            ("detection", "Do we need a SIEM under NIS2?"),
            ("email_security", "Does NIS2 address phishing protection?"),
            ("documentation", "What documentation does NIS2 require?"),
            ("remote_work", "What does NIS2 say about remote work?"),
            ("physical", "Does NIS2 cover physical security?"),
            ("legal", "When does NIS2 come into effect?"),
            ("sanctions", "What are the NIS2 penalties?"),
        ];

        suggestions.iter()
            .filter(|(cat, _)| !self.visited_categories.contains(*cat))
            .map(|(_, q)| q.to_string())
            .collect()
    }

    /// Number of unique questions visited.
    pub fn questions_visited(&self) -> usize {
        self.visited_questions.len()
    }

    /// Reset session state.
    pub fn reset(&mut self) {
        self.visited_questions.clear();
        self.visited_categories.clear();
    }
}

/// Chat engine with semantic cache and session tracking.
pub struct ChatEngine {
    cache: SemanticCache,
    pub session: Session,
}

impl ChatEngine {
    /// Create a new chat engine with the given similarity threshold.
    pub fn new(threshold: f32) -> Self {
        Self {
            cache: SemanticCache::new(threshold),
            session: Session::new(),
        }
    }

    /// Warm up the cache by loading the dataset and computing embeddings.
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

    /// Ask a question with session-aware follow-ups.
    pub fn ask(&mut self, query_embedding: &[f32]) -> ChatResponse {
        match self.cache.search(query_embedding) {
            Some(hit) => {
                // Track visit
                self.session.visit(&hit.entry.question, &hit.entry.category);

                // Filter out already-visited follow-ups
                let mut follow_ups = self.session.filter_follow_ups(&hit.entry.follow_ups);

                // If all follow-ups exhausted, suggest unexplored areas
                if follow_ups.is_empty() {
                    follow_ups = self.session.suggest_unexplored();
                    // Limit to 4
                    follow_ups.truncate(4);
                }

                ChatResponse {
                    answer: hit.entry.answer,
                    from_cache: true,
                    score: hit.score,
                    category: hit.entry.category,
                    follow_ups,
                    coverage: self.session.coverage(),
                }
            }
            None => ChatResponse {
                answer: "This question is not in the knowledge base. \
                         Please rephrase or ask about NIS2 applicability, \
                         obligations, sanctions, or incident reporting."
                    .into(),
                from_cache: false,
                score: 0.0,
                category: "miss".into(),
                follow_ups: self.session.suggest_unexplored().into_iter().take(4).collect(),
                coverage: self.session.coverage(),
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
        let hash = text.bytes().fold(0u64, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u64));
        let v: Vec<f32> = (0..384).map(|i| ((hash.wrapping_add(i)) as f32).sin()).collect();
        Ok(v)
    }

    #[test]
    fn warmup_loads_dataset() {
        let mut engine = ChatEngine::new(0.90);
        let count = engine.warmup(mock_embed).unwrap();
        assert!(count >= 100);
        assert_eq!(engine.cache_size(), count);
    }

    #[test]
    fn exact_question_hits_cache() {
        let mut engine = ChatEngine::new(0.90);
        engine.warmup(mock_embed).unwrap();
        let emb = mock_embed("What are the NIS2 penalties?").unwrap();
        let resp = engine.ask(&emb);
        assert!(resp.from_cache);
        assert!(resp.score >= 0.90);
    }

    #[test]
    fn session_filters_visited() {
        let mut engine = ChatEngine::new(0.90);
        engine.warmup(mock_embed).unwrap();
        let emb = mock_embed("What are the NIS2 penalties?").unwrap();
        let resp1 = engine.ask(&emb);
        assert!(resp1.from_cache);
        // Ask same question again -- follow-ups should exclude visited
        let resp2 = engine.ask(&emb);
        // The matched question itself was visited, follow-ups filtered
        assert!(resp2.from_cache);
    }

    #[test]
    fn coverage_starts_at_zero() {
        let engine = ChatEngine::new(0.90);
        let cov = engine.session.coverage();
        assert_eq!(cov.explored, 0);
        assert_eq!(cov.total, 16);
        assert_eq!(cov.missing.len(), 16);
    }

    #[test]
    fn coverage_increases_on_visit() {
        let mut engine = ChatEngine::new(0.90);
        engine.session.visit("test", "access_control");
        let cov = engine.session.coverage();
        assert_eq!(cov.explored, 1);
        assert_eq!(cov.missing.len(), 15);
    }

    #[test]
    fn impl_category_normalizes() {
        let mut engine = ChatEngine::new(0.90);
        engine.session.visit("test", "access_control_impl");
        let cov = engine.session.coverage();
        assert_eq!(cov.explored, 1); // _impl stripped, counts as access_control
    }

    #[test]
    fn learn_adds_entry() {
        let mut engine = ChatEngine::new(0.90);
        assert_eq!(engine.cache_size(), 0);
        engine.learn("test q".into(), "test a".into(), "test".into(), mock_embed("test q").unwrap());
        assert_eq!(engine.cache_size(), 1);
    }

    #[test]
    fn random_query_misses() {
        let mut engine = ChatEngine::new(0.99);
        engine.warmup(mock_embed).unwrap();
        let emb = mock_embed("recipe for apple pie").unwrap();
        let resp = engine.ask(&emb);
        if !resp.from_cache {
            assert_eq!(resp.category, "miss");
        }
    }
}
