.PHONY: test-all init-sanity search-sanity links-sanity tags-sanity

# =============================================================================
# E2E Tests - End-to-end CLI tests
# =============================================================================
# These tests exercise the full CLI workflow using test-config.toml
# =============================================================================

# Full E2E: all CLI commands exercised (uses debug build for faster iteration)
test-all:
	@cd .. && cargo build
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
init-sanity:
	@cd .. && cargo build --release
	@./target/release/obsidian-cli-inspector --config test-config.toml init init
	@./target/release/obsidian-cli-inspector --config test-config.toml index index
	@./target/release/obsidian-cli-inspector --config test-config.toml query search "productivity"
	@./target/release/obsidian-cli-inspector --config test-config.toml view stats

# Search sanity: init, index, search
search-sanity:
	@cd .. && cargo build --release
	@./target/release/obsidian-cli-inspector --config test-config.toml init init
	@./target/release/obsidian-cli-inspector --config test-config.toml index index
	@./target/release/obsidian-cli-inspector --config test-config.toml query search "productivity"

# Links sanity: init, index, links, backlinks
links-sanity:
	@cd .. && cargo build --release
	@./target/release/obsidian-cli-inspector --config test-config.toml init init
	@./target/release/obsidian-cli-inspector --config test-config.toml index index
	@./target/release/obsidian-cli-inspector --config test-config.toml query links "Home.md"
	@./target/release/obsidian-cli-inspector --config test-config.toml query backlinks "Home.md"

# Tags sanity: init, index, tags
tags-sanity:
	@cd .. && cargo build --release
	@./target/release/obsidian-cli-inspector --config test-config.toml init init
	@./target/release/obsidian-cli-inspector --config test-config.toml index index
	@./target/release/obsidian-cli-inspector --config test-config.toml query tags learning
