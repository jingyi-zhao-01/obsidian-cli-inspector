.PHONY: test-all init-sanity search-sanity links-sanity tags-sanity bootstrap-db contracts contracts-update machine-contract machine-contract-update

# =============================================================================
# E2E Tests - End-to-end CLI tests
# =============================================================================
# These tests exercise the full CLI workflow using test-config.toml
# =============================================================================


# =============================================================================
# Contract Tests - Unified validation of JSON and conventional output
# =============================================================================
# Combined Contract: JSON schema + CLI help/output behavior
#   Location: tests/e2e/machine_contract/
#   Snapshots: tests/e2e/machine_contract/snapshots/
# =============================================================================

# Bootstrap test database before any e2e tests
bootstrap-db: build
	@echo "=== Bootstrapping e2e test database ==="
	@cd ../.. && cargo run -- --config test-config.toml init init --force
	@echo ""
	@cd ../.. && cargo run -- --config test-config.toml index index --force
	@echo ""

# Run unified contract tests (JSON + conventional output)
contracts: bootstrap-db
	@echo "=== Running unified contract tests ==="
	@cd ../.. && cargo test --test e2e machine_contract --all-features -- --ignored --test-threads=1

# Update/generate unified contract snapshots
contracts-update: bootstrap-db
	@echo "=== Updating contract snapshots ==="
	@cd ../.. && INSTA_UPDATE=always cargo test --test e2e machine_contract --all-features -- --ignored --test-threads=1

# Backward-compatible aliases
machine-contract: contracts

machine-contract-update: contracts-update

test-all: contracts


build:
	@cd ../.. && cargo build

build-release:
	@cd ../.. && cargo build --release
