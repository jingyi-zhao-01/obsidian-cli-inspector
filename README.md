# Obsidian CLI

A **local-first CLI/TUI tool** for indexing and querying Obsidian vaults. Built with Rust for speed and reliability.

## Features

- 🔍 **Fast full-text search** using SQLite FTS5
- 🔗 **Link analysis** - Find backlinks, forward links, and unresolved references
- 🏷️ **Tag management** - Query notes by tags with AND/OR operations
- 📊 **Graph navigation** - Explore connections between notes
- 💡 **Smart suggestions** - Find related notes without explicit links
- 🧹 **Vault hygiene** - Detect bloated notes and suggest refactoring
- ⚡ **Incremental indexing** - Fast re-indexing by tracking changes
- 🖥️ **CLI + TUI** - Scriptable commands and interactive interface

## Status

**Phase 0 (Foundations) - ✅ Complete**
- ✅ Project builds as a single Rust binary
- ✅ SQLite database with FTS5 support
- ✅ Configuration via `config.toml`
- ✅ CLI command structure defined

**Upcoming Phases:**
- 📝 Phase 1: Vault scanning and change detection
- 📄 Phase 2: Obsidian markdown parsing (wikilinks, frontmatter, tags)
- ✂️ Phase 3: Document chunking for retrieval
- 🗄️ Phase 4: Full database schema implementation
- 🔄 Phase 5: Incremental indexing
- 🔎 Phase 6: Query layer (search, links, tags)
- 🧠 Phase 7: Relevance suggestions
- 🎯 Phase 8: Bloat detection and refactoring

## Installation

### Prerequisites

- Rust 1.70+ (install via [rustup](https://rustup.rs/))

### Build from source

```bash
git clone https://github.com/yourusername/obsidian-cli.git
cd obsidian-cli
cargo build --release
```

The binary will be at `target/release/obsidian-cli`.

## Quick Start

### 1. Create configuration

Copy the example config and customize it:

```bash
mkdir -p ~/.config/obsidian-cli
cp config.toml.example ~/.config/obsidian-cli/config.toml
```

Edit `~/.config/obsidian-cli/config.toml` and set your vault path:

```toml
vault_path = "/path/to/your/obsidian/vault"
```

### 2. Initialize database

```bash
obsidian-cli init
```

### 3. Index your vault

```bash
obsidian-cli index
```

### 4. Start exploring

```bash
# Search notes
obsidian-cli search "your query"

# Find backlinks
obsidian-cli backlinks "Note Name"

# List notes by tag
obsidian-cli tags productivity

# Show vault statistics
obsidian-cli stats
```

## Usage

```bash
# Initialize or reinitialize database
obsidian-cli init [--force]

# Index vault (scan and parse all files)
obsidian-cli index [--dry-run] [--force] [--verbose]

# Full-text search
obsidian-cli search "query" [--limit 20]

# Find backlinks to a note
obsidian-cli backlinks "Note Name"

# Find forward links from a note
obsidian-cli links "Note Name"

# List unresolved links
obsidian-cli unresolved-links

# List notes by tag
obsidian-cli tags [tag-name] [--all]

# Suggest related notes
obsidian-cli suggest "Note Name" [--limit 10]

# Detect bloated notes
obsidian-cli bloat [--threshold 50000] [--limit 10]

# Show statistics
obsidian-cli stats

# Launch interactive TUI
obsidian-cli tui

# Explore graph
obsidian-cli graph ["Note Name"] [--depth 2]
```

## Configuration

The configuration file (`config.toml`) supports:

- **vault_path**: Path to your Obsidian vault (required)
- **database_path**: Where to store the index (default: `~/.local/share/obsidian-cli/index.db`)
- **exclude.patterns**: Directories/patterns to skip (default: `.obsidian/`, `.git/`, `.trash/`)
- **search.default_limit**: Default search result limit
- **graph.max_depth**: Maximum graph traversal depth
- **llm**: Optional LLM configuration for Q&A features

See [config.toml.example](config.toml.example) for details.

## Architecture

### Tech Stack

- **Rust** - Performance and reliability
- **SQLite** - Local storage with FTS5 for full-text search
- **rusqlite** - SQLite bindings
- **clap** - CLI argument parsing
- **toml** - Configuration

### Data Model

- **Notes** - File metadata, content hash, frontmatter
- **Links** - Wikilinks, embeds, markdown links with resolution status
- **Tags** - Inline and frontmatter tags
- **Chunks** - Text segments for retrieval and search
- **FTS Index** - Full-text search over chunks

## Development

### Build and test

```bash
# Build
cargo build

# Run tests
cargo test

# Check code
cargo clippy

# Format code
cargo fmt
```

### Project Structure

```
src/
├── main.rs       # Entry point
├── cli.rs        # Command definitions
├── config.rs     # Configuration loading
└── db.rs         # Database schema and operations
```

## Roadmap

See [TODOs.md](TODOs.md) for detailed roadmap and implementation checklist.

## License

MIT OR Apache-2.0

## Contributing

Contributions welcome! Please open an issue first to discuss major changes.
