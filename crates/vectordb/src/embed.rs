//! Local embedding pipeline using fastembed (ONNX Runtime).
//!
//! Wraps the BGE-Small-EN-v1.5 model for generating embeddings from legal text.
//! Multilingual support available via BGE-M3 (select via [`EmbedModel`]).

use anyhow::Result;
use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};
use std::sync::Mutex;

/// Supported embedding model variants.
#[derive(Debug, Clone, Copy)]
pub enum EmbedModel {
    /// BGE-Small-EN-v1.5 — fast, 384-dim, good for prototyping.
    BgeSmall,
    /// BGE-M3 — multilingual, 1024-dim, production-grade.
    BgeM3,
}

impl EmbedModel {
    fn to_fastembed(self) -> EmbeddingModel {
        match self {
            EmbedModel::BgeSmall => EmbeddingModel::BGESmallENV15,
            EmbedModel::BgeM3 => EmbeddingModel::BGEM3,
        }
    }

    /// Returns the embedding dimensionality for this model.
    pub fn dimensions(self) -> usize {
        match self {
            EmbedModel::BgeSmall => 384,
            EmbedModel::BgeM3 => 1024,
        }
    }
}

/// Local embedding engine backed by fastembed / ONNX Runtime.
///
/// Uses interior mutability via [`Mutex`] since fastembed's `embed()` requires `&mut self`.
pub struct Embedder {
    model: Mutex<TextEmbedding>,
    dims: usize,
}

impl Embedder {
    /// Initialize the embedder with the specified model.
    /// On first run, the model weights are downloaded to a local cache.
    pub fn new(variant: EmbedModel) -> Result<Self> {
        let opts = InitOptions::new(variant.to_fastembed());
        let model = TextEmbedding::try_new(opts)?;
        Ok(Self {
            model: Mutex::new(model),
            dims: variant.dimensions(),
        })
    }

    /// Returns the embedding dimensionality.
    pub fn dimensions(&self) -> usize {
        self.dims
    }

    /// Embed a single text string.
    pub fn embed_one(&self, text: &str) -> Result<Vec<f32>> {
        let mut model = self.model.lock().map_err(|e| anyhow::anyhow!("Lock poisoned: {e}"))?;
        let results = model.embed(vec![text], None)?;
        results
            .into_iter()
            .next()
            .ok_or_else(|| anyhow::anyhow!("Embedding returned empty result"))
    }

    /// Embed a batch of text strings.
    pub fn embed_batch(&self, texts: &[&str]) -> Result<Vec<Vec<f32>>> {
        let mut model = self.model.lock().map_err(|e| anyhow::anyhow!("Lock poisoned: {e}"))?;
        let input: Vec<String> = texts.iter().map(|t| t.to_string()).collect();
        let results = model.embed(input, None)?;
        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Single test to avoid file-lock race conditions on model download cache.
    #[test]
    fn embedding_dimensions_and_semantic_similarity() {
        let embedder = Embedder::new(EmbedModel::BgeSmall).unwrap();

        // 1. Dimension check
        let vec = embedder.embed_one("test di prova").unwrap();
        assert_eq!(vec.len(), 384, "BGE-Small should produce 384-dim vectors");

        // 2. Semantic similarity check
        let v1 = embedder
            .embed_one("misure di gestione dei rischi di cibersicurezza")
            .unwrap();
        let v2 = embedder
            .embed_one("gestione dei rischi informatici e di sicurezza")
            .unwrap();
        let v3 = embedder.embed_one("ricetta per la torta di mele").unwrap();

        let sim_related = cosine_similarity(&v1, &v2);
        let sim_unrelated = cosine_similarity(&v1, &v3);

        assert!(
            sim_related > sim_unrelated,
            "Related texts ({sim_related:.4}) should be more similar than unrelated ({sim_unrelated:.4})"
        );
    }

    fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
        let dot: f32 = a.iter().zip(b).map(|(x, y)| x * y).sum();
        let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
        dot / (norm_a * norm_b)
    }
}
