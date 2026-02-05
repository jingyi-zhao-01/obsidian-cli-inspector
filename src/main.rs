mod cli;
mod config;
mod db;
mod parser;
mod scanner;

use anyhow::{Context, Result};
use clap::Parser;
use cli::{Cli, Commands};
use config::Config;
use db::Database;
use parser::MarkdownParser;
use scanner::VaultScanner;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { force } => {
            let config = load_config(cli.config)?;
            initialize_database(&config, force)?;
        }
        Commands::Stats => {
            let config = load_config(cli.config)?;
            show_stats(&config)?;
        }
        Commands::Index {
            dry_run,
            force,
            verbose,
        } => {
            let config = load_config(cli.config)?;
            index_vault(&config, dry_run, force, verbose)?;
            println!("  force: {}", force);
            println!("  verbose: {}", verbose);
        }
        Commands::Search { query, limit } => {
            println!("Search command not yet implemented");
            println!("  query: {}", query);
            println!("  limit: {}", limit);
        }
        Commands::Backlinks { note } => {
            println!("Backlinks command not yet implemented");
            println!("  note: {}", note);
        }
        Commands::Links { note } => {
            println!("Links command not yet implemented");
            println!("  note: {}", note);
        }
        Commands::UnresolvedLinks => {
            println!("UnresolvedLinks command not yet implemented");
        }
        Commands::Tags { tag, all } => {
            println!("Tags command not yet implemented");
            println!("  tag: {:?}", tag);
            println!("  all: {}", all);
        }
        Commands::Suggest { note, limit } => {
            println!("Suggest command not yet implemented");
            println!("  note: {}", note);
            println!("  limit: {}", limit);
        }
        Commands::Bloat { threshold, limit } => {
            println!("Bloat command not yet implemented");
            println!("  threshold: {}", threshold);
            println!("  limit: {}", limit);
        }
        Commands::Tui => {
            println!("TUI not yet implemented");
        }
        Commands::Graph { note, depth } => {
            println!("Graph command not yet implemented");
            println!("  note: {:?}", note);
            println!("  depth: {}", depth);
        }
    }

    Ok(())
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

fn initialize_database(config: &Config, force: bool) -> Result<()> {
    let db_path = config.database_path();

    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create database directory: {}", parent.display()))?;
    }

    println!("Initializing database at: {}", db_path.display());

    let db = Database::open(&db_path)
        .with_context(|| format!("Failed to open database: {}", db_path.display()))?;

    db.initialize(force)
        .context("Failed to initialize database schema")?;

    let version = db.get_version()?.unwrap_or(0);
    println!("Database initialized successfully (schema version: {})", version);

    Ok(())
}

fn show_stats(config: &Config) -> Result<()> {
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

    println!("Vault Statistics");
    println!("================");
    println!("Notes:            {}", stats.note_count);
    println!("Links:            {}", stats.link_count);
    println!("Tags:             {}", stats.tag_count);
    println!("Chunks:           {}", stats.chunk_count);
    println!("Unresolved links: {}", stats.unresolved_links);

    Ok(())
}

fn index_vault(config: &Config, dry_run: bool, force: bool, verbose: bool) -> Result<()> {
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
        println!("Starting vault indexing...");
        println!("Vault path: {}", config.vault_path.display());
    }

    // Scan the vault
    let scanner = VaultScanner::new(config.vault_path.clone(), config.exclude.patterns.clone());
    let files = scanner.scan().context("Failed to scan vault")?;

    if verbose {
        println!("Found {} markdown files", files.len());
    }

    if dry_run {
        println!("[DRY RUN] Would index {} files", files.len());
        return Ok(());
    }

    // Index each file
    let mut indexed_count = 0;
    for file in files {
        if verbose {
            println!("Indexing: {}", file.relative_path);
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
            println!("  → Note: {} (id: {})", parsed.title, note_id);
        }

        // Insert tags
        for tag in &parsed.tags {
            db.insert_tag(note_id, tag)
                .context("Failed to insert tag")?;
            if verbose {
                println!("    • Tag: {}", tag);
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
                println!("    • {}: [[{}]]", link_type, link.text);
            }
        }

        // Insert chunks (simple: one chunk per note for now)
        db.insert_chunk(note_id, None, &parsed.text)
            .context("Failed to insert chunk")?;
        if verbose {
            println!("    • Chunk: {} chars", parsed.text.len());
        }
        indexed_count += 1;
    }

    println!("Indexed {} notes successfully", indexed_count);

    Ok(())
}
