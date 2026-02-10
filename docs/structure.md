## Project Structure (High-Level)

```
obsidian-cli-inspector/
├── src/                          # Runtime code
│   ├── main.rs                   # CLI entrypoint and dispatch
│   ├── cli.rs                    # Command definitions and args
│   ├── config.rs                 # Config loading and defaults
│   ├── logger.rs                 # Logging/output helpers
│   ├── scanner.rs                # Vault scan + metadata
│   ├── parser/                   # Markdown + Obsidian parsing
│   ├── chunker/                  # Heading/paragraph chunking
│   ├── db/                       # SQLite schema + operations
│   │   ├── schema.rs             # Tables/indexes
│   │   ├── operations.rs         # Inserts/updates
│   │   └── stats.rs              # Stats queries
│   ├── query/                    # Retrieval layer (Phase 5)
│   │   ├── search.rs             # FTS5 search
│   │   ├── links.rs              # Backlinks/forward/unresolved
│   │   └── tags.rs               # Tag list + AND/OR
│   └── commands/                 # CLI command handlers
│       ├── index.rs              # Index pipeline
│       ├── search.rs             # Search handler
│       ├── other.rs              # Links/tags handlers
│       └── stats.rs              # Stats handler
├── tests/                        # Integration tests
│   └── test-vault/               # Sample Obsidian vault
├── docs/                         # Documentation
│   ├── PHASE2.md, PHASE3.md      # Implementation logs
│   ├── ADR-5/, ADR-6/            # Phase design + PRDs
│   └── structure.md              # This file
├── cicd/                         # Release scripts
│   ├── generate_tag.sh
│   └── generate_release_notes.sh
├── .github/workflows/build.yml   # CI build/test/release
└── test-config.toml              # Test config (test-vault)
```
