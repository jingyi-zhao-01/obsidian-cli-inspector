# E2E Contract Tests

Unified contract validation for JSON output (`--output json`) and conventional CLI output/help text. Tests are marked `#[ignore]` to keep the default unit test suite fast.

## Tests

- `machine_contract/` - Contract tests for all command groups
- `machine_contract/snapshots/` - Snapshot files for JSON and help output

## Run

```bash
make -C tests/e2e contracts           # Unified contract suite
make -C tests/e2e machine-contract    # Alias to contracts

# Or from root
cargo test --test e2e machine_contract -- --ignored
```

## Update Snapshots

When you intentionally change CLI help text or JSON output structure:

```bash
INSTA_UPDATE=always cargo test --test e2e machine_contract -- --ignored --test-threads=1

# Or from tests/e2e/
make contracts-update
```

Then commit the updated `machine_contract/snapshots/` files.
