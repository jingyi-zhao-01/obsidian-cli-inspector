.PHONY: test-all init-sanity search-sanity links-sanity tags-sanity machine-contract machine-contract-update

# =============================================================================
# E2E Tests - End-to-end CLI tests
# =============================================================================
# These tests exercise the full CLI workflow using test-config.toml
# =============================================================================

# Full E2E: all CLI commands exercised (uses debug build for faster iteration)
test-all: build
	@echo "=== Testing init commands ==="
	@cd .. && cargo run -- --config test-config.toml init init --force
	@echo ""
	@echo "=== Testing index commands ==="
	@cd .. && cargo run -- --config test-config.toml index index --force
	@echo ""
	@echo "=== Testing query commands ==="
	@cd .. && cargo run -- --config test-config.toml query search "productivity"
	@cd .. && cargo run -- --config test-config.toml query backlinks "Home"
	@cd .. && cargo run -- --config test-config.toml query links "Home"
	@cd .. && cargo run -- --config test-config.toml query unresolved
	@cd .. && cargo run -- --config test-config.toml query tags
	@cd .. && cargo run -- --config test-config.toml query tags --list
	@echo ""
	@echo "=== Testing analyze commands ==="
	@cd .. && cargo run -- --config test-config.toml analyze bloat --threshold 50000
	@cd .. && cargo run -- --config test-config.toml analyze related "Home" --limit 5
	@echo ""
	@echo "=== Testing diagnose commands ==="
	@cd .. && cargo run -- --config test-config.toml diagnose orphans
	@cd .. && cargo run -- --config test-config.toml diagnose broken-links
	@echo ""
	@echo "=== Testing view commands ==="
	@cd .. && cargo run -- --config test-config.toml view stats
	@cd .. && cargo run -- --config test-config.toml view describe "Home"
	@echo ""
	@echo "=== All CLI commands tested successfully ==="

# Sanity Tests - Quick smoke tests using release build

# Full sanity: init, index, search, stats
init-sanity: build-release
	@cd .. && ./target/release/obsidian-cli-inspector --config test-config.toml init init
	@cd .. && ./target/release/obsidian-cli-inspector --config test-config.toml index index
	@cd .. && ./target/release/obsidian-cli-inspector --config test-config.toml query search "productivity"
	@cd .. && ./target/release/obsidian-cli-inspector --config test-config.toml view stats

# Search sanity: init, index, search
search-sanity: build-release
	@cd .. && ./target/release/obsidian-cli-inspector --config test-config.toml init init
	@cd .. && ./target/release/obsidian-cli-inspector --config test-config.toml index index
	@cd .. && ./target/release/obsidian-cli-inspector --config test-config.toml query search "productivity"

# Links sanity: init, index, links, backlinks
links-sanity: build-release
	@cd .. && ./target/release/obsidian-cli-inspector --config test-config.toml init init
	@cd .. && ./target/release/obsidian-cli-inspector --config test-config.toml index index
	@cd .. && ./target/release/obsidian-cli-inspector --config test-config.toml query links "Home.md"
	@cd .. && ./target/release/obsidian-cli-inspector --config test-config.toml query backlinks "Home.md"

# Tags sanity: init, index, tags
tags-sanity: build-release
	@cd .. && ./target/release/obsidian-cli-inspector --config test-config.toml init init
	@cd .. && ./target/release/obsidian-cli-inspector --config test-config.toml index index
	@cd .. && ./target/release/obsidian-cli-inspector --config test-config.toml query tags learning

# =============================================================================
# Machine Contract Tests - JSON schema validation and snapshot testing
# =============================================================================
# Tests JSON output mode and validates machine contract schema
# Located in machine_contract.rs with snapshot files in snapshots/
# =============================================================================

# Run machine contract tests (snapshot-based validation)
machine-contract: build
	@echo "=== Initializing database for machine contract tests ==="
	@cd .. && cargo run -- --config test-config.toml init init --force
	@echo ""
	@echo "=== Indexing vault for machine contract tests ==="
	@cd .. && cargo run -- --config test-config.toml index index --force
	@echo ""
	@echo "=== Running machine contract tests ==="
	@cd .. && cargo test --test machine_contract --all-features -- --test-threads=1

# Update/generate machine contract snapshots
# Use when machine contract JSON schema changes
machine-contract-update: build
	@echo "=== Initializing database for machine contract snapshot generation ==="
	@cd .. && cargo run -- --config test-config.toml init init --force
	@echo ""
	@echo "=== Indexing vault for machine contract snapshot generation ==="
	@cd .. && cargo run -- --config test-config.toml index index --force
	@echo ""
	@echo "=== Updating machine contract snapshots ==="
	@cd .. && INSTA_UPDATE=inline cargo test --test machine_contract --all-features -- --test-threads=1

# Helper targets
build:
	@cd .. && cargo build

build-release:
	@cd .. && cargo build --release
