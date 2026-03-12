use anyhow::{Context, Result};
use embed_anything::config::TextEmbedConfig;
use embed_anything::embed_query;
use embed_anything::embeddings::embed::EmbedderBuilder;
use std::sync::{Arc, OnceLock};
use tokio::runtime::Runtime;

const DEFAULT_MODEL_ID: &str = "jinaai/jina-embeddings-v2-small-en";

pub struct SemanticEmbedder {
    embedder: Arc<embed_anything::embeddings::embed::Embedder>,
    runtime: Runtime,
    config: TextEmbedConfig,
    model_id: String,
}

impl SemanticEmbedder {
    pub fn global() -> Result<&'static Self> {
        static INSTANCE: OnceLock<SemanticEmbedder> = OnceLock::new();
        INSTANCE.get_or_try_init(Self::new)
    }

    fn new() -> Result<Self> {
        let embedder = EmbedderBuilder::new()
            .model_id(Some(DEFAULT_MODEL_ID))
            .from_pretrained_hf()
            .with_context(|| format!("Failed to load semantic model: {DEFAULT_MODEL_ID}"))?;

        Ok(Self {
            embedder: Arc::new(embedder),
            runtime: Runtime::new().context("Failed to create embedding runtime")?,
            config: TextEmbedConfig::default(),
            model_id: DEFAULT_MODEL_ID.to_string(),
        })
    }

    pub fn model_id(&self) -> &str {
        &self.model_id
    }

    pub fn embed_text(&self, text: &str) -> Result<Vec<f32>> {
        let mut embeddings = self.embed_texts(&[text])?;
        embeddings
            .pop()
            .context("No embedding returned for semantic query")
    }

    pub fn embed_texts(&self, texts: &[&str]) -> Result<Vec<Vec<f32>>> {
        if texts.is_empty() {
            return Ok(Vec::new());
        }

        let results = self
            .runtime
            .block_on(embed_query(texts, &self.embedder, Some(&self.config)))
            .context("Failed to embed semantic text")?;

        results
            .into_iter()
            .map(|item| {
                let mut vector = item
                    .embedding
                    .to_dense()
                    .context("Semantic embedding did not return dense vector")?;
                l2_normalize(&mut vector);
                Ok(vector)
            })
            .collect()
    }
}

fn l2_normalize(vec: &mut [f32]) {
    let norm = vec.iter().map(|v| v * v).sum::<f32>().sqrt();
    if norm > 0.0 {
        for value in vec.iter_mut() {
            *value /= norm;
        }
    }
}

pub fn cosine_similarity(lhs: &[f32], rhs: &[f32]) -> f32 {
    lhs.iter().zip(rhs.iter()).map(|(a, b)| a * b).sum()
}

pub fn parse_embedding(embedding_json: &str) -> Option<Vec<f32>> {
    serde_json::from_str(embedding_json).ok()
}

pub fn serialize_embedding(embedding: &[f32]) -> String {
    serde_json::to_string(embedding).unwrap_or_else(|_| "[]".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_and_serialize_embedding() {
        let embedding = vec![0.1_f32, -0.2, 0.3];
        let serialized = serialize_embedding(&embedding);
        let parsed = parse_embedding(&serialized).expect("Failed to parse embedding");
        assert_eq!(parsed, embedding);
    }
}
