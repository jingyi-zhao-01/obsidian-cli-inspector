use crate::config::Config;
use crate::db::Database;
use crate::logger::Logger;
use crate::query;
use anyhow::{Context, Result};

pub fn semantic_search_vault(
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

    if query_str.trim().is_empty() {
        if let Some(log) = logger {
            let _ = log.print_and_log("semantic", "Semantic search query cannot be empty");
        } else {
            println!("Semantic search query cannot be empty");
        }
        return Ok(());
    }

    let results = db
        .conn()
        .execute_query(|conn| query::semantic_search_chunks(conn, query_str, limit))
        .context("Failed to execute semantic search")?;

    if results.is_empty() {
        let msg = format!("No semantic results found for: {query_str}");
        if let Some(log) = logger {
            let _ = log.print_and_log("semantic", &msg);
        } else {
            println!("{msg}");
        }
        return Ok(());
    }

    let msg = format!(
        "Semantic Search Results for '{query_str}' ({} results):",
        results.len()
    );
    if let Some(log) = logger {
        let _ = log.print_and_log("semantic", &msg);
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
            let _ = log.print_and_log("semantic", &msg);
        } else {
            println!("{msg}");
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::index_vault;
    use crate::commands::initialize_database;
    use tempfile::TempDir;

    fn setup_config() -> Config {
        let vault = TempDir::new().unwrap();
        let db = TempDir::new().unwrap();
        let vault_path = vault.keep();
        let db_path = db.keep();

        std::fs::write(
            vault.path().join("a.md"),
            "# Deep Work\nProductivity improves with focused sessions.",
        )
        .unwrap();
        std::fs::write(
            vault.path().join("b.md"),
            "# Gardening\nPlant care and soil health notes.",
        )
        .unwrap();

        Config {
            vault_path,
            database_path: Some(db_path.join("vault.db")),
            log_path: None,
            exclude: Default::default(),
            search: Default::default(),
            graph: Default::default(),
            llm: None,
        }
    }

    #[test]
    fn test_semantic_search_vault_works() {
        let config = setup_config();
        initialize_database(&config, false, None).unwrap();
        index_vault(&config, false, false, false, None).unwrap();

        let result = semantic_search_vault(&config, "focused productivity", 5, None);
        assert!(result.is_ok());
    }
}
