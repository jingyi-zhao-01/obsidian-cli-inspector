use rusqlite::Connection;

#[derive(Debug)]
pub struct DatabaseStats {
    pub note_count: usize,
    pub link_count: usize,
    pub tag_count: usize,
    pub chunk_count: usize,
    pub unresolved_links: usize,
}

pub fn get_stats(conn: &Connection) -> rusqlite::Result<DatabaseStats> {
    let note_count: i32 = conn.query_row("SELECT COUNT(*) FROM notes", [], |row| row.get(0))?;

    let link_count: i32 = conn.query_row("SELECT COUNT(*) FROM links", [], |row| row.get(0))?;

    let tag_count: i32 =
        conn.query_row("SELECT COUNT(DISTINCT tag) FROM tags", [], |row| row.get(0))?;

    let chunk_count: i32 = conn.query_row("SELECT COUNT(*) FROM chunks", [], |row| row.get(0))?;

    let unresolved_links: i32 = conn.query_row(
        "SELECT COUNT(*) FROM links WHERE dst_note_id IS NULL",
        [],
        |row| row.get(0),
    )?;

    Ok(DatabaseStats {
        note_count: note_count as usize,
        link_count: link_count as usize,
        tag_count: tag_count as usize,
        chunk_count: chunk_count as usize,
        unresolved_links: unresolved_links as usize,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute(
            "CREATE TABLE notes (id INTEGER PRIMARY KEY, path TEXT, title TEXT, mtime INTEGER, hash TEXT, created_at TEXT, updated_at TEXT, frontmatter_json TEXT)",
            [],
        )
        .unwrap();
        conn.execute(
            "CREATE TABLE links (id INTEGER PRIMARY KEY, src_note_id INTEGER, dst_note_id INTEGER, dst_text TEXT, kind TEXT, is_embed INTEGER, alias TEXT, heading_ref TEXT, block_ref TEXT)",
            [],
        )
        .unwrap();
        conn.execute(
            "CREATE TABLE tags (id INTEGER PRIMARY KEY, note_id INTEGER, tag TEXT)",
            [],
        )
        .unwrap();
        conn.execute(
            "CREATE TABLE chunks (id INTEGER PRIMARY KEY, note_id INTEGER, heading_path TEXT, text TEXT, byte_offset INTEGER, byte_length INTEGER)",
            [],
        )
        .unwrap();
        conn
    }

    #[test]
    fn test_get_stats_empty_database() {
        let conn = create_test_db();
        let stats = get_stats(&conn).unwrap();
        assert_eq!(stats.note_count, 0);
        assert_eq!(stats.link_count, 0);
        assert_eq!(stats.tag_count, 0);
        assert_eq!(stats.chunk_count, 0);
        assert_eq!(stats.unresolved_links, 0);
    }

    #[test]
    fn test_database_stats_creation() {
        let stats = DatabaseStats {
            note_count: 10,
            link_count: 20,
            tag_count: 5,
            chunk_count: 15,
            unresolved_links: 2,
        };
        assert_eq!(stats.note_count, 10);
        assert_eq!(stats.link_count, 20);
    }
}
