.PHONY: test-all init-sanity search-sanity links-sanity tags-sanity machine-contract machine-contract-update cli-contract cli-contract-update all-contracts

# =============================================================================
# E2E Tests - End-to-end CLI tests
# =============================================================================
# These tests exercise the full CLI workflow using test-config.toml
# =============================================================================


# =============================================================================
# Contract Tests - Validation of machine and CLI contracts
# =============================================================================
# Machine Contract: JSON output validation and snapshot testing
#   Location: tests/e2e/machine_contract.rs
#   Snapshots: tests/e2e/snapshots/
#
# CLI Contract: CLI interface and command validation
#   Location: tests/e2e/cli_contract.rs
# =============================================================================

# Run machine contract tests (snapshot-based JSON validation)
machine-contract: build
	@echo "=== Initializing database for machine contract tests ==="
	@cd ../.. && cargo run -- --config test-config.toml init init --force
	@echo ""
	@echo "=== Indexing vault for machine contract tests ==="
	@cd ../.. && cargo run -- --config test-config.toml index index --force
	@echo ""
	@echo "=== Running machine contract tests ==="
	@cd ../.. && cargo test --test e2e machine_contract --all-features -- --ignored --test-threads=1

# Update/generate machine contract snapshots (when JSON schema changes)
machine-contract-update: build
	@echo "=== Initializing database for machine contract snapshot generation ==="
	@cd ../.. && cargo run -- --config test-config.toml init init --force
	@echo ""
	@echo "=== Indexing vault for machine contract snapshot generation ==="
	@cd ../.. && cargo run -- --config test-config.toml index index --force
	@echo ""
	@echo "=== Updating machine contract snapshots ==="
	@cd ../.. && INSTA_UPDATE=inline cargo test --test e2e machine_contract --all-features -- --ignored --test-threads=1

# Run CLI contract tests (command structure and help text validation)
cli-contract: build
	@echo "=== Running CLI contract tests ==="
	@cd ../.. && cargo test --test e2e cli_contract --all-features -- --ignored --test-threads=1

# Update CLI contract tests (if adding new tests)
cli-contract-update:
	@echo "=== Running CLI contract tests with update mode ==="
	@cd ../.. && cargo test --test e2e cli_contract --all-features -- --ignored --test-threads=1

# # Run all contract tests (both machine and CLI)
# all-contracts: build
# 	@echo "=== Running all contract tests ==="
# 	@cd ../.. && cargo test --test e2e --all-features -- --ignored --test-threads=1


build:
	@cd ../.. && cargo build

build-release:
	@cd ../.. && cargo build --release
