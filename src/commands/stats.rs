use anyhow::{Context, Result};

use crate::config::Config;
use crate::db::Database;
use crate::logger::Logger;

pub fn show_stats(config: &Config, logger: Option<&Logger>) -> Result<()> {
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

    let messages = vec![
        "Vault Statistics".to_string(),
        "================".to_string(),
        format!("Notes:            {}", stats.note_count),
        format!("Links:            {}", stats.link_count),
        format!("Tags:             {}", stats.tag_count),
        format!("Chunks:           {}", stats.chunk_count),
        format!("Unresolved links: {}", stats.unresolved_links),
    ];

    for msg in messages {
        if let Some(log) = logger {
            let _ = log.print_and_log("stats", &msg);
        } else {
            println!("{msg}");
        }
    }

    Ok(())
}
