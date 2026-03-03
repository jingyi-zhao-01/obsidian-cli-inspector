# E2E Contract Tests

Contract-based validation for JSON output (machine contract) and CLI interface (help text). Tests marked `#[ignore]` to keep unit test suite fast.

## Tests

- `machine_contract.rs` - JSON output validation (14 tests)
- `cli_contract.rs` - CLI help text validation (24 tests)
- `snapshots/` - Snapshot files for both suites

## Run

```bash
make -C tests/e2e all-contracts       # All (38 tests)
make -C tests/e2e machine-contract    # Machine contract only
make -C tests/e2e cli-contract        # CLI contract only

# Or from root
cargo test --test e2e -- --ignored
```

## Update Snapshots

When you intentionally change CLI help text or JSON output structure:

```bash
INSTA_UPDATE=inline cargo test --test e2e -- --ignored --test-threads=1

# Or from tests/e2e/
make machine-contract-update
make cli-contract-update
```

Then commit the updated `snapshots/` files.
