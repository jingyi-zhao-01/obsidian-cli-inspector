use crate::config::Config;
use crate::db::Database;
use crate::logger::Logger;
use crate::query;
use anyhow::{Context, Result};

pub fn get_backlinks(config: &Config, note: &str, logger: Option<&Logger>) -> Result<()> {
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

    let backlinks = db
        .conn()
        .execute_query(|conn| query::get_backlinks(conn, note))
        .context("Failed to get backlinks")?;

    if backlinks.is_empty() {
        let msg = format!("No backlinks found for: {note}");
        if let Some(log) = logger {
            let _ = log.print_and_log("backlinks", &msg);
        } else {
            println!("{msg}");
        }
        return Ok(());
    }

    let msg = format!("Backlinks to '{}' ({} found):", note, backlinks.len());
    if let Some(log) = logger {
        let _ = log.print_and_log("backlinks", &msg);
    } else {
        println!("{msg}");
    }

    for (idx, link) in backlinks.iter().enumerate() {
        let link_type = if link.is_embed { "embed" } else { "link" };
        let msg = format!(
            "{}. {} ({})\n   Type: {} {}",
            idx + 1,
            link.note_title,
            link.note_path,
            link_type,
            link.alias
                .as_ref()
                .map(|a| format!("(alias: {a})"))
                .unwrap_or_default()
        );
        if let Some(log) = logger {
            let _ = log.print_and_log("backlinks", &msg);
        } else {
            println!("{msg}");
        }
    }

    Ok(())
}
