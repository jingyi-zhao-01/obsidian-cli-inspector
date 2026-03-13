use crate::config::Config;
use crate::db::Database;
use crate::logger::Logger;
use crate::query;
use anyhow::{Context, Result};

pub fn search_vault(
    config: &Config,
    query_str: &str,
    limit: usize,
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

    if query_str.is_empty() {
        if let Some(log) = logger {
            let _ = log.print_and_log("search", "Search query cannot be empty");
        } else {
            println!("Search query cannot be empty");
        }
        return Ok(());
    }

    let results = db
        .conn()
        .execute_query(|conn| query::search_chunks(conn, query_str, limit))
        .context("Failed to execute search")?;

    if results.is_empty() {
        let msg = format!("No results found for: {query_str}");
        if let Some(log) = logger {
            let _ = log.print_and_log("search", &msg);
        } else {
            println!("{msg}");
        }
        return Ok(());
    }

    let msg = format!(
        "Search Results for '{query_str}' ({} results):",
        results.len()
    );
    if let Some(log) = logger {
        let _ = log.print_and_log("search", &msg);
    } else {
        println!("{msg}");
    }

    for (idx, result) in results.iter().enumerate() {
        let heading_info = result
            .heading_path
            .as_ref()
            .map(|h| format!(" [{h}]"))
            .unwrap_or_default();
        let msg = format!(
            "{}. {} ({}){}\n   {}",
            idx + 1,
            result.note_title,
            result.note_path,
            heading_info,
            result
                .chunk_text
                .lines()
                .next()
                .unwrap_or("")
                .chars()
                .take(80)
                .collect::<String>()
        );
        if let Some(log) = logger {
            let _ = log.print_and_log("search", &msg);
        } else {
            println!("{msg}");
        }
    }

    Ok(())
}
