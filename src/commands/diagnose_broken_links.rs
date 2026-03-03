use crate::config::Config;
use crate::db::Database;
use crate::logger::Logger;
use crate::query;
use anyhow::{Context, Result};

pub fn diagnose_broken_links_cmd(config: &Config, logger: Option<&Logger>) -> Result<()> {
    let db_path = config.database_path();

    if !db_path.exists() {
        anyhow::bail!(
            "Database not found at: {}\nRun 'obsidian-cli init' first",
            db_path.display()
        );
    }

    let db = Database::open(&db_path)
        .with_context(|| format!("Failed to open database: {}", db_path.display()))?;

    let broken_links = db
        .conn()
        .execute_query(query::diagnose_broken_links)
        .context("Failed to diagnose broken links")?;

    if broken_links.is_empty() {
        println!("=== BROKEN LINKS ===");
        println!("No broken links found! All links are valid.");
    } else {
        let unresolved: Vec<_> = broken_links
            .iter()
            .filter(|l| l.status == "unresolved")
            .collect();
        let ambiguous: Vec<_> = broken_links
            .iter()
            .filter(|l| l.status == "ambiguous")
            .collect();

        println!("=== BROKEN LINKS ===");
        println!("Found {} broken link(s):\n", broken_links.len());

        if !unresolved.is_empty() {
            println!("--- UNRESOLVED ({}) ---", unresolved.len());
            for (idx, link) in unresolved.iter().enumerate() {
                println!(
                    "{}. [[{}]] → {} (in {})",
                    idx + 1,
                    link.target,
                    link.raw_link,
                    link.src_path
                );
            }
            println!();
        }

        if !ambiguous.is_empty() {
            println!("--- AMBIGUOUS ({}) ---", ambiguous.len());
            for (idx, link) in ambiguous.iter().enumerate() {
                println!("{}. [[{}]] (in {})", idx + 1, link.target, link.src_path);
                println!("   Candidates: {}", link.candidates.join(", "));
            }
        }
    }

    if let Some(log) = logger {
        let _ = log.print_and_log(
            "diagnose-broken-links",
            &format!("Found {} broken links", broken_links.len()),
        );
    }

    Ok(())
}
