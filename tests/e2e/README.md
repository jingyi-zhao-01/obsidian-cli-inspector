# E2E Contract Tests

This folder contains end-to-end checks for customer-facing CLI output (text help + JSON output).

## Tests

- `machine_contract/` - Test cases
- `machine_contract/snapshots/` - Expected output snapshots

## Run

```bash
make -C tests/e2e contracts           # Run full e2e contract suite
make -C tests/e2e machine-contract    # Alias

# Or from root
cargo test --test e2e machine_contract --all-features -- --ignored --test-threads=1
```

`contracts` automatically initializes and indexes the test database before running tests.

## Update Snapshots

When output intentionally changes, update snapshots:

```bash
INSTA_UPDATE=always cargo test --test e2e machine_contract -- --ignored --test-threads=1

# Or from tests/e2e/
make contracts-update
```

Then commit updated files in `machine_contract/snapshots/`.

Contract format reference: [docs/machine-contract.md](../../docs/machine-contract.md)
