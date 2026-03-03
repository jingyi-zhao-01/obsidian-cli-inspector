use crate::config::Config;
use crate::db::Database;
use crate::logger::Logger;
use crate::query;
use anyhow::{Context, Result};

pub fn get_note_describe(config: &Config, filename: &str, logger: Option<&Logger>) -> Result<()> {
    let db_path = config.database_path();

    if !db_path.exists() {
        anyhow::bail!(
            "Database not found at: {}\nRun 'obsidian-cli init' first",
            db_path.display()
        );
    }

    let db = Database::open(&db_path)
        .with_context(|| format!("Failed to open database: {}", db_path.display()))?;

    let note = db
        .conn()
        .execute_query(|conn| query::get_note_by_filename(conn, filename))
        .context("Failed to get note metadata")?;

    match note {
        Some(note) => {
            let mtime = chrono::DateTime::from_timestamp(note.mtime, 0)
                .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                .unwrap_or_else(|| note.mtime.to_string());

            println!("File Metadata:");
            println!("=============");
            println!("ID:          {}", note.id);
            println!("Title:       {}", note.title);
            println!("Path:        {}", note.path);
            println!("Modified:    {mtime}");
            println!("Hash:        {}", note.hash);
            println!("Created:     {}", note.created_at);
            println!("Updated:     {}", note.updated_at);

            if let Some(frontmatter) = note.frontmatter {
                if !frontmatter.is_empty() {
                    println!("Frontmatter: {frontmatter}");
                }
            }

            Ok(())
        }
        None => {
            let msg = format!("Note not found: {filename}");
            if let Some(log) = logger {
                let _ = log.print_and_log("describe", &msg);
            } else {
                println!("{msg}");
            }
            Ok(())
        }
    }
}
