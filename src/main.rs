mod cli;
mod config;
mod db;
mod logger;
mod parser;
mod scanner;

use anyhow::{Context, Result};
use clap::Parser;
use cli::{Cli, Commands};
use config::Config;
use db::Database;
use logger::Logger;
use parser::MarkdownParser;
use scanner::VaultScanner;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let config = load_config(cli.config.clone()).ok();
    let logger = if let Some(ref cfg) = config {
        match Logger::new(cfg.log_dir()) {
            Ok(log) => Some(log),
            Err(_) => None,
        }
    } else {
        None
    };

    let result = match cli.command {
        Commands::Init { force } => {
            let config = load_config(cli.config)?;
            if let Some(ref log) = logger {
                let _ = log.log_section("init", "Starting Init Command");
            }
            initialize_database(&config, force, logger.as_ref())
        }
        Commands::Stats => {
            let config = load_config(cli.config)?;
            if let Some(ref log) = logger {
                let _ = log.log_section("stats", "Starting Stats Command");
            }
            show_stats(&config, logger.as_ref())
        }
        Commands::Index {
            dry_run,
            force,
            verbose,
        } => {
            let config = load_config(cli.config)?;
            if let Some(ref log) = logger {
                let _ = log.log_section("index", "Starting Index Command");
            }
            index_vault(&config, dry_run, force, verbose, logger.as_ref())
        }
        Commands::Search { query, limit } => {
            if let Some(ref log) = logger {
                let _ = log.print_and_log("search", "Search command not yet implemented");
                let _ = log.print_and_log("search", &format!("  query: {}", query));
                let _ = log.print_and_log("search", &format!("  limit: {}", limit));
            } else {
                println!("Search command not yet implemented");
                println!("  query: {}", query);
                println!("  limit: {}", limit);
            }
            Ok(())
        }
        Commands::Backlinks { note } => {
            if let Some(ref log) = logger {
                let _ = log.print_and_log("backlinks", "Backlinks command not yet implemented");
                let _ = log.print_and_log("backlinks", &format!("  note: {}", note));
            } else {
                println!("Backlinks command not yet implemented");
                println!("  note: {}", note);
            }
            Ok(())
        }
        Commands::Links { note } => {
            if let Some(ref log) = logger {
                let _ = log.print_and_log("links", "Links command not yet implemented");
                let _ = log.print_and_log("links", &format!("  note: {}", note));
            } else {
                println!("Links command not yet implemented");
                println!("  note: {}", note);
            }
            Ok(())
        }
        Commands::UnresolvedLinks => {
            if let Some(ref log) = logger {
                let _ = log.print_and_log("unresolved-links", "UnresolvedLinks command not yet implemented");
            } else {
                println!("UnresolvedLinks command not yet implemented");
            }
            Ok(())
        }
        Commands::Tags { tag, all } => {
            if let Some(ref log) = logger {
                let _ = log.print_and_log("tags", "Tags command not yet implemented");
                let _ = log.print_and_log("tags", &format!("  tag: {:?}", tag));
                let _ = log.print_and_log("tags", &format!("  all: {}", all));
            } else {
                println!("Tags command not yet implemented");
                println!("  tag: {:?}", tag);
                println!("  all: {}", all);
            }
            Ok(())
        }
        Commands::Suggest { note, limit } => {
            if let Some(ref log) = logger {
                let _ = log.print_and_log("suggest", "Suggest command not yet implemented");
                let _ = log.print_and_log("suggest", &format!("  note: {}", note));
                let _ = log.print_and_log("suggest", &format!("  limit: {}", limit));
            } else {
                println!("Suggest command not yet implemented");
                println!("  note: {}", note);
                println!("  limit: {}", limit);
            }
            Ok(())
        }
        Commands::Bloat { threshold, limit } => {
            if let Some(ref log) = logger {
                let _ = log.print_and_log("bloat", "Bloat command not yet implemented");
                let _ = log.print_and_log("bloat", &format!("  threshold: {}", threshold));
                let _ = log.print_and_log("bloat", &format!("  limit: {}", limit));
            } else {
                println!("Bloat command not yet implemented");
                println!("  threshold: {}", threshold);
                println!("  limit: {}", limit);
            }
            Ok(())
        }
        Commands::Tui => {
            if let Some(ref log) = logger {
                let _ = log.print_and_log("tui", "TUI not yet implemented");
            } else {
                println!("TUI not yet implemented");
            }
            Ok(())
        }
        Commands::Graph { note, depth } => {
            if let Some(ref log) = logger {
                let _ = log.print_and_log("graph", "Graph command not yet implemented");
                let _ = log.print_and_log("graph", &format!("  note: {:?}", note));
                let _ = log.print_and_log("graph", &format!("  depth: {}", depth));
            } else {
                println!("Graph command not yet implemented");
                println!("  note: {:?}", note);
                println!("  depth: {}", depth);
            }
            Ok(())
        }
    };

    result
}

fn load_config(config_path: Option<PathBuf>) -> Result<Config> {
    let path = config_path.unwrap_or_else(|| {
        let mut p = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        p.push("obsidian-cli");
        p.push("config.toml");
        p
    });

    if !path.exists() {
        anyhow::bail!(
            "Config file not found at: {}\nCreate one using config.toml.example as template",
            path.display()
        );
    }

    Config::from_file(&path).context("Failed to load config file")
}

fn initialize_database(config: &Config, force: bool, logger: Option<&Logger>) -> Result<()> {
    let db_path = config.database_path();

    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create database directory: {}", parent.display()))?;
    }

    let msg = format!("Initializing database at: {}", db_path.display());
    if let Some(log) = logger {
        let _ = log.print_and_log("init", &msg);
    } else {
        println!("{}", msg);
    }

    let db = Database::open(&db_path)
        .with_context(|| format!("Failed to open database: {}", db_path.display()))?;

    db.initialize(force)
        .context("Failed to initialize database schema")?;

    let version = db.get_version()?.unwrap_or(0);
    let msg = format!("Database initialized successfully (schema version: {})", version);
    if let Some(log) = logger {
        let _ = log.print_and_log("init", &msg);
    } else {
        println!("{}", msg);
    }

    Ok(())
}

fn show_stats(config: &Config, logger: Option<&Logger>) -> Result<()> {
    let db_path = config.database_path();

    if !db_path.exists() {
        anyhow::bail!(
            "Database not found at: {}\nRun 'obsidian-cli init' first",
            db_path.display()
        );
    }

    let db = Database::open(&db_path)
        .with_context(|| format!("Failed to open database: {}", db_path.display()))?;

    let stats = db.get_stats().context("Failed to get database stats")?;

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
            println!("{}", msg);
        }
    }

    Ok(())
}

fn index_vault(config: &Config, dry_run: bool, _force: bool, verbose: bool, logger: Option<&Logger>) -> Result<()> {
    let db_path = config.database_path();

    if !db_path.exists() {
        anyhow::bail!(
            "Database not found at: {}\nRun 'obsidian-cli init' first",
            db_path.display()
        );
    }

    let db = Database::open(&db_path)
        .with_context(|| format!("Failed to open database: {}", db_path.display()))?;

    if verbose {
        let msg = "Starting vault indexing...";
        if let Some(log) = logger {
            let _ = log.print_and_log("index", msg);
        } else {
            println!("{}", msg);
        }
        let msg = format!("Vault path: {}", config.vault_path.display());
        if let Some(log) = logger {
            let _ = log.print_and_log("index", &msg);
        } else {
            println!("{}", msg);
        }
    }

    // Scan the vault
    let scanner = VaultScanner::new(config.vault_path.clone(), config.exclude.patterns.clone());
    let files = scanner.scan().context("Failed to scan vault")?;

    if verbose {
        let msg = format!("Found {} markdown files", files.len());
        if let Some(log) = logger {
            let _ = log.print_and_log("index", &msg);
        } else {
            println!("{}", msg);
        }
    }

    if dry_run {
        let msg = format!("[DRY RUN] Would index {} files", files.len());
        if let Some(log) = logger {
            let _ = log.print_and_log("index", &msg);
        } else {
            println!("{}", msg);
        }
        return Ok(());
    }

    // Index each file
    let mut indexed_count = 0;
    for file in files {
        if verbose {
            let msg = format!("Indexing: {}", file.relative_path);
            if let Some(log) = logger {
                let _ = log.print_and_log("index", &msg);
            } else {
                println!("{}", msg);
            }
        }

        // Read file content
        let content = fs::read_to_string(&file.path)
            .with_context(|| format!("Failed to read file: {}", file.path.display()))?;

        // Parse markdown
        let parsed = MarkdownParser::parse(&content);

        // Compute hash (simple for now - just file size + mtime)
        let hash = format!("{:x}:{}", file.size, file.mtime);

        // Insert note
        let note_id = db
            .insert_note(&file.relative_path, &parsed.title, file.mtime, &hash, None)
            .context("Failed to insert note")?;

        if verbose {
            let msg = format!("  → Note: {} (id: {})", parsed.title, note_id);
            if let Some(log) = logger {
                let _ = log.print_and_log("index", &msg);
            } else {
                println!("{}", msg);
            }
        }

        // Insert tags
        for tag in &parsed.tags {
            db.insert_tag(note_id, tag)
                .context("Failed to insert tag")?;
            if verbose {
                let msg = format!("    • Tag: {}", tag);
                if let Some(log) = logger {
                    let _ = log.print_and_log("index", &msg);
                } else {
                    println!("{}", msg);
                }
            }
        }

        // Insert links
        for link in &parsed.links {
            db.insert_link(
                note_id,
                &link.text,
                "wikilink",
                link.is_embed,
                link.alias.as_deref(),
                link.heading_ref.as_deref(),
                link.block_ref.as_deref(),
            )
            .context("Failed to insert link")?;
            if verbose {
                let link_type = if link.is_embed { "Embed" } else { "Link" };
                let msg = format!("    • {}: [[{}]]", link_type, link.text);
                if let Some(log) = logger {
                    let _ = log.print_and_log("index", &msg);
                } else {
                    println!("{}", msg);
                }
            }
        }

        // Insert chunks (simple: one chunk per note for now)
        db.insert_chunk(note_id, None, &parsed.text)
            .context("Failed to insert chunk")?;
        if verbose {
            let msg = format!("    • Chunk: {} chars", parsed.text.len());
            if let Some(log) = logger {
                let _ = log.print_and_log("index", &msg);
            } else {
                println!("{}", msg);
            }
        }
        indexed_count += 1;
    }

    let msg = format!("Indexed {} notes successfully", indexed_count);
    if let Some(log) = logger {
        let _ = log.print_and_log("index", &msg);
    } else {
        println!("{}", msg);
    }

    Ok(())
}
