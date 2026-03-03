//! Stubbed responses for E2E contract tests
//!
//! This module provides predefined JSON responses for testing without
//! requiring actual database operations or vault indexing.

use serde_json::{json, Value};

/// Generate a stubbed JSON response for any command
pub fn get_stub_response(command: &str, params: &Value) -> Value {
    let stub_result = match command {
        // Query commands
        "query.search" => get_stub_search_response(params),
        "query.backlinks" => get_stub_backlinks_response(params),
        "query.links" => get_stub_links_response(params),
        "query.unresolved" => get_stub_unresolved_response(params),
        "query.tags" => get_stub_tags_response(params),
        // Index command
        "index.index" => json!({ "status": "indexed", "notes_processed": 10 }),
        // Init command
        "init.init" => json!({ "status": "initialized" }),
        // Analyze commands
        "analyze.bloat" => get_stub_bloat_response(params),
        "analyze.related" => get_stub_related_response(params),
        // Diagnose commands
        "diagnose.orphans" => get_stub_orphans_response(params),
        "diagnose.broken-links" => get_stub_broken_links_response(),
        // View commands
        "view.stats" => get_stub_stats_response(),
        "view.describe" => get_stub_describe_response(params),
        // TUI command
        "tui" => json!({ "status": "launching tui" }),
        // Default empty response
        _ => json!({ "total": 0, "items": [] }),
    };

    json!({
        "version": "1.0",
        "command": command,
        "timestamp": "2026-01-01T00:00:00Z",
        "params": params,
        "result": stub_result,
        "meta": {
            "query_time_ms": 0,
            "vault_path": "./tests/test-vault"
        }
    })
}

fn get_stub_search_response(params: &Value) -> Value {
    let query = params.get("query").and_then(|v| v.as_str()).unwrap_or("");
    let limit = params.get("limit").and_then(|v| v.as_u64()).unwrap_or(20) as usize;

    // Return deterministic stub data based on query
    let items: Vec<Value> = (0..limit.min(3))
        .map(|i| {
            json!({
                "chunk_id": i + 1,
                "note_id": i + 1,
                "note_path": format!("tests/test-vault/Note{}.md", i + 1),
                "note_title": format!("Test Note {}", i + 1),
                "heading_path": "",
                "chunk_text": format!("Content matching '{}' in test note {}", query, i + 1),
                "rank": (10 - i) as f64
            })
        })
        .collect();

    json!({ "total": items.len(), "items": items })
}

fn get_stub_backlinks_response(params: &Value) -> Value {
    let _note = params.get("note").and_then(|v| v.as_str()).unwrap_or("");

    let items: Vec<Value> = vec![
        json!({
            "note_id": 1,
            "note_path": "tests/test-vault/Backlink1.md",
            "note_title": "Backlink Note 1",
            "is_embed": false,
            "alias": null,
            "heading_ref": null,
            "block_ref": null
        }),
        json!({
            "note_id": 2,
            "note_path": "tests/test-vault/Backlink2.md",
            "note_title": "Backlink Note 2",
            "is_embed": false,
            "alias": "Custom Alias",
            "heading_ref": "Section",
            "block_ref": null
        }),
    ];

    json!({ "total": items.len(), "items": items })
}

fn get_stub_links_response(params: &Value) -> Value {
    let _note = params.get("note").and_then(|v| v.as_str()).unwrap_or("");

    let items: Vec<Value> = vec![
        json!({
            "note_id": 3,
            "note_path": "tests/test-vault/Link1.md",
            "note_title": "Linked Note 1",
            "is_embed": false,
            "alias": null,
            "heading_ref": null,
            "block_ref": null
        }),
        json!({
            "note_id": 4,
            "note_path": "tests/test-vault/Link2.md",
            "note_title": "Linked Note 2",
            "is_embed": true,
            "alias": null,
            "heading_ref": "Important Section",
            "block_ref": "block-1"
        }),
    ];

    json!({ "total": items.len(), "items": items })
}

fn get_stub_unresolved_response(_params: &Value) -> Value {
    let items: Vec<Value> = vec![json!({
        "note_id": 5,
        "note_path": "tests/test-vault/Unresolved1.md",
        "note_title": "Unresolved Note 1",
        "is_embed": false,
        "alias": null,
        "heading_ref": null,
        "block_ref": null
    })];

    json!({ "total": items.len(), "items": items })
}

fn get_stub_tags_response(params: &Value) -> Value {
    let list_all = params
        .get("list")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let tag = params.get("tag").and_then(|v| v.as_str());

    if list_all || tag.is_none() {
        // Return all tags
        let items: Vec<Value> = vec![
            json!({ "tag": "rust" }),
            json!({ "tag": "programming" }),
            json!({ "tag": "notes" }),
            json!({ "tag": "productivity" }),
            json!({ "tag": "learning" }),
        ];
        json!({ "total": items.len(), "items": items })
    } else {
        // Return notes for specific tag
        let items: Vec<Value> = vec![json!({
            "note_id": 1,
            "note_path": "tests/test-vault/TagNote1.md",
            "note_title": "Tagged Note 1",
            "tags": [tag.unwrap()]
        })];
        json!({ "total": items.len(), "items": items })
    }
}

fn get_stub_bloat_response(params: &Value) -> Value {
    let threshold = params
        .get("threshold")
        .and_then(|v| v.as_u64())
        .unwrap_or(50000) as usize;
    let limit = params.get("limit").and_then(|v| v.as_u64()).unwrap_or(10) as usize;

    let items: Vec<Value> = (0..limit)
        .map(|i| {
            json!({
                "note_id": i + 1,
                "note_path": format!("tests/test-vault/LargeNote{}.md", i + 1),
                "note_title": format!("Large Note {}", i + 1),
                "size_bytes": threshold + (i * 10000),
                "chunk_count": i + 5
            })
        })
        .collect();

    json!({ "total": items.len(), "items": items })
}

fn get_stub_related_response(params: &Value) -> Value {
    let note = params.get("note").and_then(|v| v.as_str()).unwrap_or("");
    let limit = params.get("limit").and_then(|v| v.as_u64()).unwrap_or(10) as usize;

    let items: Vec<Value> = (0..limit)
        .map(|i| {
            json!({
                "note_id": i + 100,
                "note_path": format!("tests/test-vault/Related{}.md", i + 1),
                "note_title": format!("Related to {} - Note {}", note, i + 1),
                "relevance_score": 0.95 - (i as f64 * 0.1)
            })
        })
        .collect();

    json!({ "total": items.len(), "items": items })
}

fn get_stub_orphans_response(params: &Value) -> Value {
    let exclude_templates = params
        .get("exclude_templates")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let exclude_daily = params
        .get("exclude_daily")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let mut items: Vec<Value> = vec![
        json!({
            "note_id": 10,
            "note_path": "tests/test-vault/Orphan1.md",
            "note_title": "Orphan Note 1"
        }),
        json!({
            "note_id": 11,
            "note_path": "tests/test-vault/Orphan2.md",
            "note_title": "Orphan Note 2"
        }),
    ];

    if !exclude_templates && !exclude_daily {
        items.push(json!({
            "note_id": 12,
            "note_path": "tests/test-vault/Template.md",
            "note_title": "Template Note"
        }));
    }

    json!({ "total": items.len(), "items": items })
}

fn get_stub_broken_links_response() -> Value {
    let items: Vec<Value> = vec![json!({
        "source_note_id": 1,
        "source_note_path": "tests/test-vault/Source1.md",
        "source_note_title": "Source Note 1",
        "target": "NonExistentNote",
        "link_type": "wikilink"
    })];

    json!({ "total": items.len(), "items": items })
}

fn get_stub_stats_response() -> Value {
    json!({
        "notes": 42,
        "links": 156,
        "tags": 23,
        "chunks": 128,
        "unresolved_links": 3
    })
}

fn get_stub_describe_response(params: &Value) -> Value {
    let filename = params
        .get("filename")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    json!({
        "note_id": 1,
        "note_path": format!("tests/test-vault/{}.md", filename),
        "note_title": filename,
        "size_bytes": 12345,
        "created": "2026-01-01T00:00:00Z",
        "modified": "2026-01-15T00:00:00Z",
        "links_in": 5,
        "links_out": 3,
        "tags": ["tag1", "tag2"]
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_stub_response_query_search() {
        let params = json!({ "query": "rust", "limit": 10 });
        let response = get_stub_response("query.search", &params);

        assert_eq!(response["version"], "1.0");
        assert_eq!(response["command"], "query.search");
        assert!(response["result"].is_object());
    }

    #[test]
    fn test_get_stub_response_unknown_command() {
        let params = json!({});
        let response = get_stub_response("unknown.command", &params);

        assert_eq!(response["version"], "1.0");
        assert!(response["result"]["total"] == 0);
    }

    #[test]
    fn test_get_stub_response_view_stats() {
        let params = json!({});
        let response = get_stub_response("view.stats", &params);

        assert_eq!(response["result"]["notes"], 42);
        assert_eq!(response["result"]["links"], 156);
    }
}
