use crate::semantic::{cosine_similarity, parse_embedding, serialize_embedding, SemanticEmbedder};
use anyhow::Error as AnyhowError;
use rusqlite::{params, Connection, Error as RusqliteError, Result};

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

fn embed_error(err: AnyhowError) -> RusqliteError {
    RusqliteError::UserFunctionError(Box::new(err))
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

fn get_or_create_chunk_embedding(
    conn: &Connection,
    embedder: &SemanticEmbedder,
    chunk_id: i64,
    text: &str,
    expected_dim: usize,
) -> Result<Vec<f32>> {
    let model = embedder.model_id();
    let existing = conn.query_row(
        "SELECT embedding_json FROM chunk_embeddings WHERE chunk_id = ?1 AND model = ?2",
        params![chunk_id, model],
        |row| row.get::<_, String>(0),
    );

    if let Ok(embedding_json) = existing {
        if let Some(vector) = parse_embedding(&embedding_json) {
            if expected_dim == 0 || vector.len() == expected_dim {
                return Ok(vector);
            }
        }
    }

    let vector = embedder.embed_text(text).map_err(embed_error)?;
    let embedding_json = serialize_embedding(&vector);

    conn.execute(
        "INSERT INTO chunk_embeddings (chunk_id, model, embedding_json)
         VALUES (?1, ?2, ?3)
         ON CONFLICT(chunk_id) DO UPDATE SET
            model = excluded.model,
            embedding_json = excluded.embedding_json,
            updated_at = CURRENT_TIMESTAMP",
        params![chunk_id, model, embedding_json],
    )?;

    Ok(vector)
}

pub fn semantic_search_chunks(
    conn: &Connection,
    query: &str,
    limit: usize,
) -> Result<Vec<SemanticSearchResult>> {
    ensure_semantic_embedding_cache(conn)?;

    let embedder = SemanticEmbedder::global().map_err(embed_error)?;
    let query_embedding = embedder.embed_text(query).map_err(embed_error)?;
    if query_embedding
        .iter()
        .all(|value| value.abs() < f32::EPSILON)
    {
        return Ok(Vec::new());
    }

    let expected_dim = query_embedding.len();
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
        let chunk_embedding =
            get_or_create_chunk_embedding(conn, embedder, chunk_id, &chunk_text, expected_dim)?;

        if chunk_embedding.len() != expected_dim {
            continue;
        }

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
