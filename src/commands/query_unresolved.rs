use crate::config::Config;
use crate::db::Database;
use crate::logger::Logger;
use crate::query;
use anyhow::{Context, Result};

pub fn list_unresolved_links(config: &Config, logger: Option<&Logger>) -> Result<()> {
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

    let unresolved = db
        .conn()
        .execute_query(query::get_unresolved_links)
        .context("Failed to get unresolved links")?;

    if unresolved.is_empty() {
        let msg = "No unresolved links found!";
        if let Some(log) = logger {
            let _ = log.print_and_log("unresolved-links", msg);
        } else {
            println!("{msg}");
        }
        return Ok(());
    }

    let msg = format!("Unresolved Links ({} found):", unresolved.len());
    if let Some(log) = logger {
        let _ = log.print_and_log("unresolved-links", &msg);
    } else {
        println!("{msg}");
    }

    for (idx, link) in unresolved.iter().enumerate() {
        let msg = format!(
            "{}. {} → {}\n   In: {}",
            idx + 1,
            link.note_title,
            link.note_path,
            link.note_path
        );
        if let Some(log) = logger {
            let _ = log.print_and_log("unresolved-links", &msg);
        } else {
            println!("{msg}");
        }
    }

    Ok(())
}
