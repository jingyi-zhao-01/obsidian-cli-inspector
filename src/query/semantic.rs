use rusqlite::{params, Connection, Result};
use std::collections::HashMap;

const EMBEDDING_DIM: usize = 256;
const EMBEDDING_MODEL: &str = "hash-emb-v2";

fn stable_hash_token(token: &str) -> u64 {
    const FNV_OFFSET: u64 = 0xcbf29ce484222325;
    const FNV_PRIME: u64 = 0x100000001b3;

    let mut hash = FNV_OFFSET;
    for byte in token.as_bytes() {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}

#[derive(Debug, Clone)]
pub struct SemanticSearchResult {
    pub chunk_id: i64,
    pub note_id: i64,
    pub note_path: String,
    pub note_title: String,
    pub heading_path: Option<String>,
    pub chunk_text: String,
    pub score: f32,
}

fn token_counts(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();

    for token in text
        .to_lowercase()
        .split(|ch: char| !ch.is_alphanumeric())
        .filter(|token| !token.is_empty())
    {
        *counts.entry(token.to_string()).or_insert(0) += 1;
    }

    counts
}

fn l2_normalize(vec: &mut [f32]) {
    let norm = vec.iter().map(|v| v * v).sum::<f32>().sqrt();
    if norm > 0.0 {
        for value in vec.iter_mut() {
            *value /= norm;
        }
    }
}

fn embed_text(text: &str) -> Vec<f32> {
    let counts = token_counts(text);
    let mut embedding = vec![0.0_f32; EMBEDDING_DIM];

    for (token, count) in counts {
        let hash = stable_hash_token(&token);

        let index = (hash as usize) % EMBEDDING_DIM;
        let sign = if ((hash >> 8) & 1) == 0 { 1.0 } else { -1.0 };
        let weight = 1.0 + (count as f32).ln();

        embedding[index] += sign * weight;
    }

    l2_normalize(&mut embedding);
    embedding
}

fn cosine_similarity(lhs: &[f32], rhs: &[f32]) -> f32 {
    lhs.iter().zip(rhs.iter()).map(|(a, b)| a * b).sum()
}

fn parse_embedding(embedding_json: &str) -> Option<Vec<f32>> {
    let vector: Vec<f32> = serde_json::from_str(embedding_json).ok()?;
    if vector.len() == EMBEDDING_DIM {
        Some(vector)
    } else {
        None
    }
}

fn serialize_embedding(embedding: &[f32]) -> String {
    serde_json::to_string(embedding).unwrap_or_else(|_| "[]".to_string())
}

pub fn ensure_semantic_embedding_cache(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS chunk_embeddings (
            chunk_id INTEGER PRIMARY KEY,
            model TEXT NOT NULL,
            embedding_json TEXT NOT NULL,
            updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (chunk_id) REFERENCES chunks(id) ON DELETE CASCADE
        )",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_chunk_embeddings_model ON chunk_embeddings(model)",
        [],
    )?;

    Ok(())
}

fn get_or_create_chunk_embedding(conn: &Connection, chunk_id: i64, text: &str) -> Result<Vec<f32>> {
    let existing = conn.query_row(
        "SELECT embedding_json FROM chunk_embeddings WHERE chunk_id = ?1 AND model = ?2",
        params![chunk_id, EMBEDDING_MODEL],
        |row| row.get::<_, String>(0),
    );

    if let Ok(embedding_json) = existing {
        if let Some(vector) = parse_embedding(&embedding_json) {
            return Ok(vector);
        }
    }

    let vector = embed_text(text);
    let embedding_json = serialize_embedding(&vector);

    conn.execute(
        "INSERT INTO chunk_embeddings (chunk_id, model, embedding_json)
         VALUES (?1, ?2, ?3)
         ON CONFLICT(chunk_id) DO UPDATE SET
            model = excluded.model,
            embedding_json = excluded.embedding_json,
            updated_at = CURRENT_TIMESTAMP",
        params![chunk_id, EMBEDDING_MODEL, embedding_json],
    )?;

    Ok(vector)
}

pub fn semantic_search_chunks(
    conn: &Connection,
    query: &str,
    limit: usize,
) -> Result<Vec<SemanticSearchResult>> {
    ensure_semantic_embedding_cache(conn)?;

    let query_embedding = embed_text(query);
    if query_embedding
        .iter()
        .all(|value| value.abs() < f32::EPSILON)
    {
        return Ok(Vec::new());
    }

    let mut stmt = conn.prepare(
        "SELECT
            c.id,
            n.id,
            n.path,
            n.title,
            c.heading_path,
            c.text
         FROM chunks c
         JOIN notes n ON c.note_id = n.id
         ORDER BY n.path COLLATE NOCASE, c.byte_offset, c.id",
    )?;

    let chunk_rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, i64>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, String>(3)?,
            row.get::<_, Option<String>>(4)?,
            row.get::<_, String>(5)?,
        ))
    })?;

    let mut scored_results = Vec::new();
    for row in chunk_rows {
        let (chunk_id, note_id, note_path, note_title, heading_path, chunk_text) = row?;
        let chunk_embedding = get_or_create_chunk_embedding(conn, chunk_id, &chunk_text)?;
        let score = cosine_similarity(&query_embedding, &chunk_embedding);

        if score > 0.0 {
            scored_results.push(SemanticSearchResult {
                chunk_id,
                note_id,
                note_path,
                note_title,
                heading_path,
                chunk_text,
                score,
            });
        }
    }

    scored_results.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.note_path.cmp(&b.note_path))
            .then_with(|| a.chunk_id.cmp(&b.chunk_id))
    });

    scored_results.truncate(limit);
    Ok(scored_results)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embed_text_dimension() {
        let vector = embed_text("deep work and focus");
        assert_eq!(vector.len(), EMBEDDING_DIM);
    }

    #[test]
    fn test_embed_text_is_normalized() {
        let vector = embed_text("deep work and focus");
        let norm = vector.iter().map(|v| v * v).sum::<f32>().sqrt();
        assert!((norm - 1.0).abs() < 1e-4);
    }

    #[test]
    fn test_cosine_similarity_higher_for_related_text() {
        let query = embed_text("deep work productivity");
        let related = embed_text("productivity through deep focus");
        let unrelated = embed_text("database schema migration");

        assert!(cosine_similarity(&query, &related) > cosine_similarity(&query, &unrelated));
    }
}
