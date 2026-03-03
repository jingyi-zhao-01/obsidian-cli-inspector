use crate::config::Config;
use crate::db::Database;
use crate::logger::Logger;
use crate::query;
use anyhow::{Context, Result};

pub fn get_forward_links(config: &Config, note: &str, logger: Option<&Logger>) -> Result<()> {
    let db_path = config.database_path();

    if !db_path.exists() {
        anyhow::bail!(
            "Database not found at: {}\nRun 'obsidian-cli init' first",
            db_path.display()
        );
    }

    let db = Database::open(&db_path)
        .with_context(|| format!("Failed to open database: {}", db_path.display()))?;

    let forward_links = db
        .conn()
        .execute_query(|conn| query::get_forward_links(conn, note))
        .context("Failed to get forward links")?;

    if forward_links.is_empty() {
        let msg = format!("No forward links found for: {note}");
        if let Some(log) = logger {
            let _ = log.print_and_log("links", &msg);
        } else {
            println!("{msg}");
        }
        return Ok(());
    }

    let msg = format!(
        "Forward links from '{}' ({} found):",
        note,
        forward_links.len()
    );
    if let Some(log) = logger {
        let _ = log.print_and_log("links", &msg);
    } else {
        println!("{msg}");
    }

    for (idx, link) in forward_links.iter().enumerate() {
        let status = if link.note_id < 0 {
            "[unresolved]"
        } else {
            "[resolved]"
        };
        let msg = format!(
            "{}. {} ({})\n   {}",
            idx + 1,
            link.note_title,
            link.note_path,
            status
        );
        if let Some(log) = logger {
            let _ = log.print_and_log("links", &msg);
        } else {
            println!("{msg}");
        }
    }

    Ok(())
}
