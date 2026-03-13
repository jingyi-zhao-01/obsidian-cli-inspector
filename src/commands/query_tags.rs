use crate::config::Config;
use crate::db::Database;
use crate::logger::Logger;
use crate::query;
use anyhow::{Context, Result};

pub fn list_notes_by_tag(
    config: &Config,
    tag: &Option<String>,
    all: bool,
    logger: Option<&Logger>,
) -> Result<()> {
    let db_path = config.database_path();

    if !db_path.exists() {
        anyhow::bail!(
            "Database not found at: {}\nRun 'obsidian-cli init' first",
            db_path.display()
        );
    }

    let db = Database::open(&db_path)
        .with_context(|| format!("Failed to open database: {}", db_path.display()))?;

    // Check if database has been indexed
    let stats = db.get_stats().context("Failed to get database stats")?;
    if stats.note_count == 0 {
        anyhow::bail!(
            "Database is empty. Run 'obsidian-cli-inspector index' to index your vault first"
        );
    }

    if all || tag.is_none() {
        let all_tags = db
            .conn()
            .execute_query(query::list_tags)
            .context("Failed to list tags")?;

        if all_tags.is_empty() {
            let msg = "No tags found in vault";
            if let Some(log) = logger {
                let _ = log.print_and_log("tags", msg);
            } else {
                println!("{msg}");
            }
            return Ok(());
        }

        let msg = format!("All Tags ({} total):", all_tags.len());
        if let Some(log) = logger {
            let _ = log.print_and_log("tags", &msg);
        } else {
            println!("{msg}");
        }

        for (idx, tag_name) in all_tags.iter().enumerate() {
            let msg = format!("{}. {}", idx + 1, tag_name);
            if let Some(log) = logger {
                let _ = log.print_and_log("tags", &msg);
            } else {
                println!("{msg}");
            }
        }
    } else if let Some(tag_name) = tag {
        let notes = db
            .conn()
            .execute_query(|conn| query::get_notes_by_tag(conn, tag_name))
            .context("Failed to get notes by tag")?;

        if notes.is_empty() {
            let msg = format!("No notes found with tag: {tag_name}");
            if let Some(log) = logger {
                let _ = log.print_and_log("tags", &msg);
            } else {
                println!("{msg}");
            }
            return Ok(());
        }

        let msg = format!("Notes with tag '{}' ({} found):", tag_name, notes.len());
        if let Some(log) = logger {
            let _ = log.print_and_log("tags", &msg);
        } else {
            println!("{msg}");
        }

        for (idx, note) in notes.iter().enumerate() {
            let msg = format!(
                "{}. {} ({})\n   Tags: {}",
                idx + 1,
                note.note_title,
                note.note_path,
                note.tags.join(", ")
            );
            if let Some(log) = logger {
                let _ = log.print_and_log("tags", &msg);
            } else {
                println!("{msg}");
            }
        }
    }

    Ok(())
}
