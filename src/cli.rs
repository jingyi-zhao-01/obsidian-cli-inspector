use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "obsidian-cli-inspector")]
#[command(author, version, about = "Local-first CLI/TUI for indexing and querying Obsidian vaults", long_about = None)]
pub struct Cli {
    /// Path to config file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize or reinitialize the database
    Init {
        /// Force reinitialization (drops existing data)
        #[arg(short, long)]
        force: bool,
    },

    /// Index the vault (scan and parse all files)
    Index {
        /// Perform a dry run without writing to database
        #[arg(short = 'n', long)]
        dry_run: bool,

        /// Force full re-index (ignores change detection)
        #[arg(short, long)]
        force: bool,

        /// Show verbose output
        #[arg(short, long)]
        verbose: bool,
    },

    /// Search notes using full-text search
    Search {
        /// Search query
        query: String,

        /// Maximum number of results
        #[arg(short, long, default_value = "20")]
        limit: usize,
    },

    /// List backlinks to a note
    Backlinks {
        /// Note path or title
        note: String,
    },

    /// List forward links from a note
    Links {
        /// Note path or title
        note: String,
    },

    /// List all unresolved links in the vault
    UnresolvedLinks,

    /// List notes by tag
    Tags {
        /// Tag name (without #)
        tag: Option<String>,

        /// List all tags if no tag specified
        #[arg(short, long)]
        all: bool,
    },

    /// Suggest related notes not directly linked
    Suggest {
        /// Note path or title
        note: String,

        /// Maximum number of suggestions
        #[arg(short, long, default_value = "10")]
        limit: usize,
    },

    /// Detect bloated notes and suggest refactoring
    Bloat {
        /// Minimum size threshold in bytes
        #[arg(short, long, default_value = "50000")]
        threshold: usize,

        /// Maximum number of notes to analyze
        #[arg(short, long, default_value = "10")]
        limit: usize,
    },

    /// Show statistics about the vault
    Stats,

    /// Launch interactive TUI
    Tui,

    /// Display vault graph information
    Graph {
        /// Note path or title (if not specified, shows overall graph)
        note: Option<String>,

        /// Maximum traversal depth
        #[arg(short, long, default_value = "2")]
        depth: usize,
    },
}
