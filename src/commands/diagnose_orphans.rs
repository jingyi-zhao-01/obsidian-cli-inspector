use crate::config::Config;
use crate::db::Database;
use crate::logger::Logger;
use crate::query;
use anyhow::{Context, Result};

pub fn diagnose_orphans(
    config: &Config,
    exclude_templates: bool,
    exclude_daily: bool,
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

    let orphans = db
        .conn()
        .execute_query(|conn| query::get_orphans(conn, exclude_templates, exclude_daily))
        .context("Failed to get orphans")?;

    let dead_ends = db
        .conn()
        .execute_query(|conn| query::get_dead_ends(conn, exclude_templates, exclude_daily))
        .context("Failed to get dead ends")?;

    println!("=== ORPHANS (no incoming + no outgoing links) ===");
    if orphans.is_empty() {
        println!("No orphan notes found.");
    } else {
        println!("Found {} orphan note(s):\n", orphans.len());
        for (idx, note) in orphans.iter().enumerate() {
            println!(
                "{}. {} ({})\n   In: {}",
                idx + 1,
                note.note_title,
                note.note_path,
                note.note_path
            );
        }
    }

    println!("\n=== DEAD ENDS (has incoming but no outgoing links) ===");
    if dead_ends.is_empty() {
        println!("No dead-end notes found.");
    } else {
        println!("Found {} dead-end note(s):\n", dead_ends.len());
        for (idx, note) in dead_ends.iter().enumerate() {
            println!(
                "{}. {} ({})\n   In: {}",
                idx + 1,
                note.note_title,
                note.note_path,
                note.note_path
            );
        }
    }

    if let Some(log) = logger {
        let _ = log.print_and_log(
            "diagnose-orphans",
            &format!(
                "Found {} orphans, {} dead-ends",
                orphans.len(),
                dead_ends.len()
            ),
        );
    }

    Ok(())
}
