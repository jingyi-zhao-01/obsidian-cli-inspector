# Machine Contract

For agent integration, use JSON mode via `-o json` or `--output json`.

## Enable JSON Mode

```bash
# Human output (default)
obsidian-cli-inspector search notes rust

# JSON mode
obsidian-cli-inspector -o json search notes rust
obsidian-cli-inspector --output json search notes rust
```

Notes:
- `--json` is not currently supported.
- Some commands may print human-readable lines before the JSON payload. The canonical machine payload is the final JSON object written for the command.

## Response Envelope (Current)

Successful JSON-mode commands produce this envelope:

```json
{
  "command": "search.notes",
  "timestamp": "2026-03-02T20:26:00Z",
  "params": {
    "query": "rust",
    "limit": 20
  },
  "result": {
    "total": 0,
    "items": []
  },
  "meta": {
    "query_time_ms": 1,
    "vault_path": "./tests/test-vault"
  }
}
```

Guaranteed envelope fields:
- `command` is the fully-qualified command name (for example, `search.notes`)
- `timestamp` is an RFC 3339 timestamp string
- `params` is an object
- `result` is an object
- `meta.query_time_ms` is numeric
- `meta.vault_path` is a string

## Result Shape

All `search.*` commands return:

```json
"result": {
  "total": 0,
  "items": []
}
```

`items` entry shape is command-specific (`search.notes`, `search.backlinks`, `search.links`, `search.unresolved`, `search.tags`).

Non-search commands may use command-specific result objects (for example, `view.stats`).

## Error Behavior (Current)

Three error paths exist:

1. Argument parsing errors (for example, missing required args)
   - Exit code: `2`
   - Output: clap-generated plain text on stderr (not JSON)

2. Runtime command errors after parsing succeeds
   - Exit code: `1`
   - JSON-mode output includes:

```json
{
  "error": {
    "code": 1,
    "message": "..."
  }
}
```

3. Database prerequisite errors (for query, view, diagnose, and analyze commands)
   - Exit code: `1`
   - Common error messages:
     - `"Database not found at: {path}\nRun 'obsidian-cli-inspector index' to create and index the database first"`
     - `"Database is empty. Run 'obsidian-cli-inspector index' to index your vault first"`
   - These errors appear in both text mode (stderr) and JSON mode (within error object)
```

## Non-Goals in Current Contract

The following are not currently guaranteed by implementation:
- Stable per-note `stable_id` in machine responses
- Deterministic sort tie-breaking by `stable_id`
- Distinct runtime exit codes beyond `1` and parse-time `2`
