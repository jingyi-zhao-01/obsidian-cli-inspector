# Phase 5 Quick Spec — Query Layer (Basic Retrieval)

## Goal
Implement fast, correct retrieval for search, links, and tags over the existing SQLite schema and chunk index.

## Scope
- Full‑text search using FTS5 (BM25) over `fts_chunks`
- Backlinks lookup
- Forward links lookup
- Unresolved links listing
- Tag listing and tag set operations (AND/OR)
- Ranked chunk retrieval (search results ranked by BM25)

## Out of Scope
- Relevance suggestions (Phase 7)
- Incremental indexing (Phase 6)
- Any new schema changes beyond what’s already defined

## Functional Requirements
1) **Search**
- Query `fts_chunks` with BM25 ranking
- Return top N chunks with note metadata
- Support `--limit`
- Display snippet (optional) and heading path if available

2) **Links**
- Backlinks: list notes linking to a target note
- Forward links: list notes the target note links to
- Unresolved links: list entries with no resolved `dst_note_id`

3) **Tags**
- List notes by a single tag
- Support AND/OR tag queries via CLI flags
- Return unique note list with stable ordering

## Non‑Functional Requirements
- Search latency under 100ms on test vault for `--limit 10`
- Deterministic output for identical input and vault state
- No new DB migrations

## CLI Expectations
- `search "query" --limit N`
- `backlinks "Note Name"`
- `links "Note Name"`
- `unresolved-links`
- `tags tag1 [--all]` (AND) and `--any` (OR) variant if available

## Implementation Tasks
- Add query helpers in `src/query/` for search, links, tags
- Wire command handlers in `src/commands/`
- Ensure note name normalization matches existing parser rules
- Add basic tests for search, tags, links, unresolved links

## Acceptance Criteria
- All Phase 5 verification steps in docs pass
- Search returns ranked results
- Backlinks/links/unresolved lists are correct
- Tag queries support AND/OR and return correct note sets
- Performance meets the target on test vault

## Risks / Notes
- Note name normalization must match link parsing; mismatches will break link resolution.
- FTS queries need careful escaping to avoid syntax errors.
