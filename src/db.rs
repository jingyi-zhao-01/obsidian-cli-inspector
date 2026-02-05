use rusqlite::{Connection, Result, OptionalExtension};
use std::path::Path;

pub const SCHEMA_VERSION: i32 = 1;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn = Connection::open(path)?;
        Ok(Database { conn })
    }

    pub fn initialize(&self, force: bool) -> Result<()> {
        if force {
            self.drop_tables()?;
        }

        // Create schema version table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS schema_version (
                version INTEGER PRIMARY KEY,
                applied_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        // Check current version
        let current_version: Option<i32> = self
            .conn
            .query_row(
                "SELECT version FROM schema_version ORDER BY version DESC LIMIT 1",
                [],
                |row| row.get(0),
            )
            .ok();

        if current_version.is_none() || force {
            self.create_schema()?;
            self.conn.execute(
                "INSERT OR REPLACE INTO schema_version (version) VALUES (?1)",
                [SCHEMA_VERSION],
            )?;
        }

        Ok(())
    }

    fn create_schema(&self) -> Result<()> {
        // Notes table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS notes (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                path TEXT NOT NULL UNIQUE,
                title TEXT NOT NULL,
                mtime INTEGER NOT NULL,
                hash TEXT NOT NULL,
                frontmatter_json TEXT,
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        // Links table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS links (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                src_note_id INTEGER NOT NULL,
                dst_text TEXT NOT NULL,
                dst_note_id INTEGER,
                kind TEXT NOT NULL,
                is_embed INTEGER NOT NULL DEFAULT 0,
                alias TEXT,
                heading_ref TEXT,
                block_ref TEXT,
                FOREIGN KEY (src_note_id) REFERENCES notes(id) ON DELETE CASCADE,
                FOREIGN KEY (dst_note_id) REFERENCES notes(id) ON DELETE SET NULL
            )",
            [],
        )?;

        // Tags table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS tags (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                note_id INTEGER NOT NULL,
                tag TEXT NOT NULL,
                FOREIGN KEY (note_id) REFERENCES notes(id) ON DELETE CASCADE,
                UNIQUE(note_id, tag)
            )",
            [],
        )?;

        // Chunks table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS chunks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                note_id INTEGER NOT NULL,
                heading_path TEXT,
                text TEXT NOT NULL,
                byte_offset INTEGER NOT NULL,
                byte_length INTEGER NOT NULL,
                FOREIGN KEY (note_id) REFERENCES notes(id) ON DELETE CASCADE
            )",
            [],
        )?;

        // FTS5 virtual table for full-text search
        self.conn.execute(
            "CREATE VIRTUAL TABLE IF NOT EXISTS fts_chunks USING fts5(
                note_id UNINDEXED,
                heading_path,
                text,
                content=chunks,
                content_rowid=id
            )",
            [],
        )?;

        // Triggers to keep FTS5 in sync
        self.conn.execute(
            "CREATE TRIGGER IF NOT EXISTS chunks_ai AFTER INSERT ON chunks BEGIN
                INSERT INTO fts_chunks(rowid, note_id, heading_path, text)
                VALUES (new.id, new.note_id, new.heading_path, new.text);
            END",
            [],
        )?;

        self.conn.execute(
            "CREATE TRIGGER IF NOT EXISTS chunks_ad AFTER DELETE ON chunks BEGIN
                DELETE FROM fts_chunks WHERE rowid = old.id;
            END",
            [],
        )?;

        self.conn.execute(
            "CREATE TRIGGER IF NOT EXISTS chunks_au AFTER UPDATE ON chunks BEGIN
                DELETE FROM fts_chunks WHERE rowid = old.id;
                INSERT INTO fts_chunks(rowid, note_id, heading_path, text)
                VALUES (new.id, new.note_id, new.heading_path, new.text);
            END",
            [],
        )?;

        // Create indexes
        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_notes_path ON notes(path)",
            [],
        )?;

        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_notes_mtime ON notes(mtime)",
            [],
        )?;

        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_links_src ON links(src_note_id)",
            [],
        )?;

        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_links_dst ON links(dst_note_id)",
            [],
        )?;

        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_links_dst_text ON links(dst_text)",
            [],
        )?;

        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_tags_note ON tags(note_id)",
            [],
        )?;

        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_tags_tag ON tags(tag)",
            [],
        )?;

        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_chunks_note ON chunks(note_id)",
            [],
        )?;

        Ok(())
    }

    fn drop_tables(&self) -> Result<()> {
        self.conn.execute("DROP TABLE IF EXISTS fts_chunks", [])?;
        self.conn.execute("DROP TABLE IF EXISTS chunks", [])?;
        self.conn.execute("DROP TABLE IF EXISTS tags", [])?;
        self.conn.execute("DROP TABLE IF EXISTS links", [])?;
        self.conn.execute("DROP TABLE IF EXISTS notes", [])?;
        self.conn.execute("DROP TABLE IF EXISTS schema_version", [])?;
        Ok(())
    }

    pub fn get_version(&self) -> Result<Option<i32>> {
        self.conn
            .query_row(
                "SELECT version FROM schema_version ORDER BY version DESC LIMIT 1",
                [],
                |row| row.get(0),
            )
            .optional()
    }

    pub fn get_stats(&self) -> Result<DatabaseStats> {
        let note_count: i32 = self
            .conn
            .query_row("SELECT COUNT(*) FROM notes", [], |row| row.get(0))?;

        let link_count: i32 = self
            .conn
            .query_row("SELECT COUNT(*) FROM links", [], |row| row.get(0))?;

        let tag_count: i32 = self.conn.query_row(
            "SELECT COUNT(DISTINCT tag) FROM tags",
            [],
            |row| row.get(0),
        )?;

        let chunk_count: i32 = self
            .conn
            .query_row("SELECT COUNT(*) FROM chunks", [], |row| row.get(0))?;

        let unresolved_links: i32 = self.conn.query_row(
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

    pub fn insert_note(
        &self,
        path: &str,
        title: &str,
        mtime: u64,
        hash: &str,
        frontmatter_json: Option<&str>,
    ) -> Result<i64> {
        self.conn.execute(
            "INSERT OR REPLACE INTO notes (path, title, mtime, hash, frontmatter_json)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![path, title, mtime as i64, hash, frontmatter_json],
        )?;

        let note_id: i64 = self.conn.query_row(
            "SELECT id FROM notes WHERE path = ?1",
            [path],
            |row| row.get(0),
        )?;

        Ok(note_id)
    }

    pub fn get_note_by_path(&self, path: &str) -> Result<Option<i64>> {
        self.conn
            .query_row("SELECT id FROM notes WHERE path = ?1", [path], |row| {
                row.get(0)
            })
            .optional()
    }

    pub fn insert_tag(&self, note_id: i64, tag: &str) -> Result<()> {
        self.conn.execute(
            "INSERT OR IGNORE INTO tags (note_id, tag) VALUES (?1, ?2)",
            rusqlite::params![note_id, tag],
        )?;
        Ok(())
    }

    pub fn insert_link(
        &self,
        src_note_id: i64,
        dst_text: &str,
        kind: &str,
        is_embed: bool,
        alias: Option<&str>,
        heading_ref: Option<&str>,
        block_ref: Option<&str>,
    ) -> Result<()> {
        self.conn.execute(
            "INSERT INTO links (src_note_id, dst_text, kind, is_embed, alias, heading_ref, block_ref)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            rusqlite::params![
                src_note_id,
                dst_text,
                kind,
                if is_embed { 1 } else { 0 },
                alias,
                heading_ref,
                block_ref
            ],
        )?;
        Ok(())
    }

    pub fn insert_chunk(&self, note_id: i64, heading_path: Option<&str>, text: &str) -> Result<()> {
        let text_len = text.len() as i32;
        self.conn.execute(
            "INSERT INTO chunks (note_id, heading_path, text, byte_offset, byte_length)
             VALUES (?1, ?2, ?3, 0, ?4)",
            rusqlite::params![note_id, heading_path, text, text_len],
        )?;
        Ok(())
    }

    pub fn clear_note_data(&self, note_id: i64) -> Result<()> {
        self.conn.execute(
            "DELETE FROM links WHERE src_note_id = ?1",
            [note_id],
        )?;
        self.conn.execute(
            "DELETE FROM tags WHERE note_id = ?1",
            [note_id],
        )?;
        self.conn.execute(
            "DELETE FROM chunks WHERE note_id = ?1",
            [note_id],
        )?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct DatabaseStats {
    pub note_count: usize,
    pub link_count: usize,
    pub tag_count: usize,
    pub chunk_count: usize,
    pub unresolved_links: usize,
}
