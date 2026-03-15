# Design: Heading-Aware CLI Operations for Obsidian CLI Inspector

Status: Draft
Date: 2026-03-15
Owner: OpenHands

## Summary

Agent feedback shows repetitive manual steps when editing Obsidian notes:

- Navigating to headings manually to extract or rewrite sections.
- Re-checking link targets (especially heading anchors) for correctness.
- Lack of section-scoped diffs and context extraction tools.

This design proposes a phased plan to add heading-aware read/edit workflows to `obsidian-cli-inspector` while reusing existing indexing, chunking, and link parsing. The plan explicitly distinguishes what can be adapted from current functionality versus foundational new work.

## Goals

- Provide CLI commands to list, read, diff, and replace sections by heading name.
- Resolve Obsidian heading anchors deterministically (matching Obsidian’s normalization rules).
- Validate heading/block links against vault content.
- Offer a task-scoped context extractor for agent workflows.
- Preserve JSON output contracts for machine integration.

## Non-Goals

- Full Obsidian plugin parity or UI automation.
- Real-time sync with the Obsidian app.
- WYSIWYG editing or rich-text transformations.

## Existing Functionality Audit (Adaptable Building Blocks)

The current implementation already contains several primitives that can be reused:

1. **Heading-aware chunking** (`src/chunker.rs`)
   - Chunker splits notes by headings and captures `heading_path` plus `byte_offset`/`byte_length`.
   - Provides a lightweight heading parser (`src/chunker/heading.rs`).

2. **Persistent index with offsets** (`src/db/schema.rs`, `src/commands/index.rs`)
   - `chunks` table stores `heading_path`, `byte_offset`, and `byte_length`.
   - Offsets enable deterministic section extraction from raw files.

3. **Link parsing with heading/block references** (`src/parser/*.rs`)
   - Wikilinks and markdown links extract `heading_ref` and `block_ref`.
   - Existing normalization for note identifiers (`normalize_note_identifier`).

4. **Query and JSON output** (`src/query/*`, `src/machine_contract.rs`)
   - JSON envelope already used for deterministic machine output.
   - Search results include `heading_path` fields, useful for section context.

5. **Read-only operational posture**
   - The CLI is currently read-only, so adding safe, opt-in write paths can be layered without destabilizing existing read flows.

## Foundational New Work Required

The following capabilities do not exist today and require new foundational work:

- **Obsidian heading anchor normalization** (case folding, punctuation stripping, whitespace normalization, de-duplication).
- **Heading registry per note** (explicit table for headings and anchors, beyond chunk-level paths).
- **Block reference indexing** (capture `^block-id` markers in notes).
- **Frontmatter alias extraction** (Obsidian aliases are not stored in current index).
- **Safe write pipeline** (file locking, atomic write, backups, and dry-run support).
- **Section diff tooling** (section-scoped comparison across notes/files).
- **Context extractor** (task-scoped section extraction across multiple notes).

## Proposed Command Surface (Draft)

These commands are additive and can coexist with existing CLI groups.

### Read-only section tools

- `obsidian-cli-inspector note headings <note> [--format json]`
- `obsidian-cli-inspector note get-section <note> --heading "<text>" [--include-children] [--format json]`
- `obsidian-cli-inspector note resolve-anchor <note> --heading "<text>"`
- `obsidian-cli-inspector diagnose link-targets [--include-blocks]`

### Edit tools (opt-in)

- `obsidian-cli-inspector note replace-section <note> --heading "<text>" --content-file <path> [--dry-run] [--backup]`
- `obsidian-cli-inspector note append-section <note> --heading "<text>" --content-file <path> [--dry-run]`

### Diff + context

- `obsidian-cli-inspector note diff-section <note> --heading "<text>" --against <note|file>`
- `obsidian-cli-inspector task extract-context --note <note> --headings "A|B|C" [--format json]`

## Data Model Extensions

### New tables

- `headings`
  - `id`, `note_id`, `level`, `text`, `anchor`, `byte_offset`, `byte_length`, `parent_id`
- `blocks`
  - `id`, `note_id`, `block_id`, `byte_offset`, `byte_length`
- `aliases`
  - `id`, `note_id`, `alias`

### Why this is needed

- `chunks.heading_path` is helpful for search context but insufficient for exact section boundaries.
- Explicit heading rows enable precise heading resolution, de-duplication, and hierarchical relationships.
- Block/alias tables support robust link validation and anchor resolution.

## Phased Implementation Plan

### Phase 0 — Specification & Validation (1–2 days)

**Outcomes**
- Document Obsidian heading anchor normalization rules with test vectors.
- Decide how to handle duplicate headings (suffix numbering strategy).
- Confirm expected JSON output schema for new commands.

**Why now**
- Anchor resolution is foundational for all later phases.

### Phase 1 — Indexing Extensions (3–5 days)

**Adaptations**
- Extend the indexer (`commands/index.rs`) to capture headings and block IDs using existing heading parser and new block-id parser.
- Store explicit heading metadata in the new `headings` table.

**New work**
- Add Obsidian anchor normalization library.
- Add schema version bump and migration logic.

**Deliverables**
- `headings`, `blocks`, and `aliases` tables populated during indexing.
- Regression tests for heading detection and anchor generation.

### Phase 2 — Read-only Section Commands (3–4 days)

**Adaptations**
- Use `headings` table with stored offsets to extract section text directly from files.
- Reuse existing JSON envelope from `machine_contract` for new commands.

**New work**
- `note headings` and `note get-section` CLI group.
- `resolve-anchor` and `diagnose link-targets` commands.

**Deliverables**
- Deterministic JSON outputs for section retrieval and link validation.
- Clear error codes for missing headings or ambiguous matches.

### Phase 3 — Safe Write Pipeline + Section Editing (4–6 days)

**Adaptations**
- Use same section extraction routines to locate replacement ranges.
- Leverage config paths for backup storage.

**New work**
- File locking strategy (OS-specific) + atomic writes.
- Dry-run diffs and backup naming scheme.
- `replace-section` and `append-section` commands.

**Deliverables**
- Opt-in write mode with backups and dry runs.
- Tests on temp vault fixtures to ensure no corruption.

### Phase 4 — Diff + Context Extraction (3–4 days)

**Adaptations**
- Reuse section extraction from Phase 2 and diff crate for comparisons.

**New work**
- `diff-section` command for human output and JSON.
- `task extract-context` aggregation command (multiple headings, multi-note).

**Deliverables**
- Section-scoped diffs and a context bundle response suitable for agents.

## Risks & Mitigations

- **Anchor normalization mismatches**: Mitigate with extensive test vectors and real-world fixtures.
- **Write-safety concerns**: Default to read-only, add `--write`/`--dry-run` gates and backups.
- **Schema migrations**: Versioned migrations with explicit fallback instructions in docs.

## Open Questions

- Should section editing be allowed in JSON-mode only, or both JSON and text?
- Where to store backups by default (same directory vs config path)?
- Should ambiguous heading matches prefer shortest path or require explicit disambiguation?

## Success Metrics

- Agent can retrieve a section by heading in a single command without manual scanning.
- Heading link validation reliably flags missing anchors and block refs.
- Replacing a section produces deterministic diffs and preserves file integrity.
- JSON outputs remain stable for automation.
