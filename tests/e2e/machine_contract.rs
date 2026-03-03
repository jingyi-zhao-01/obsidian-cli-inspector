use serde_json::Value;
use std::path::PathBuf;
use std::process::Command;

/// Helper function to get test config path
fn get_test_config_path() -> PathBuf {
    PathBuf::from("test-config.toml")
}

/// Run a single command and capture JSON output
fn run_command(args: &[&str]) -> Result<Value, String> {
    let output = Command::new("./target/debug/obsidian-cli-inspector")
        .args(args)
        .current_dir(".")
        .output()
        .map_err(|e| format!("Failed to execute command: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Command failed: {stderr}"));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Find the last line which should be the JSON
    let lines: Vec<&str> = stdout.lines().collect();
    if let Some(last_line) = lines.last() {
        // Check if it looks like JSON (starts with {)
        if last_line.trim_start().starts_with('{') {
            return serde_json::from_str(last_line)
                .map_err(|e| format!("Failed to parse JSON: {e:?}"));
        }

        // Otherwise search for the last { and } pair
        if let Some(json_end) = last_line.rfind('}') {
            if let Some(json_start) = last_line[..=json_end].rfind('{') {
                let json_str = &last_line[json_start..=json_end];
                return serde_json::from_str(json_str)
                    .map_err(|e| format!("Failed to parse JSON: {e:?}"));
            }
        }
    }

    Err("No valid JSON response found".to_string())
}

/// Validate that JSON response has required schema fields
fn validate_schema(json: &Value, expected_command: &str) {
    assert!(json.is_object(), "Response must be a JSON object");

    // Validate required fields
    assert_eq!(json["version"], "1.0", "Schema version must be 1.0");
    assert_eq!(json["command"], expected_command, "Command name mismatch");
    assert!(
        json["timestamp"].is_string(),
        "Timestamp must be present and string"
    );
    assert!(json["params"].is_object(), "Params must be an object");
    assert!(json["result"].is_object(), "Result must be an object");
    assert!(json["meta"].is_object(), "Meta must be an object");

    // Validate meta fields
    let meta = &json["meta"];
    assert!(
        meta["query_time_ms"].is_number(),
        "query_time_ms must be a number"
    );
    assert!(
        meta["vault_path"].is_string(),
        "vault_path must be a string"
    );
}

/// Remove dynamic fields for snapshot comparison
fn normalize_for_snapshot(mut json: Value) -> Value {
    // Remove dynamic timestamp
    json["timestamp"] = Value::String("TIMESTAMP".to_string());

    // Remove dynamic query_time_ms
    if let Some(meta) = json.get_mut("meta") {
        if let Some(meta_obj) = meta.as_object_mut() {
            meta_obj.insert("query_time_ms".to_string(), Value::Number(0.into()));
        }
    }

    json
}

// These tests are considered end-to-end/contract checks and can be
// expensive (invoke the CLI binary and exercise the full database). They
// should not run during the normal `cargo test`/`make test` invocation. The
// `#[ignore]` attribute means they are skipped by default; the e2e Makefile
// runs them explicitly with `--ignored`.
#[test]
#[ignore]
fn test_machine_contract_init() {
    let config_path = get_test_config_path().to_string_lossy().to_string();
    let args = vec![
        "--output",
        "json",
        "--config",
        &config_path,
        "init",
        "init",
        "--force",
    ];

    let output = run_command(&args).expect("Failed to run init command");
    validate_schema(&output, "init.init");

    let normalized = normalize_for_snapshot(output);
    insta::assert_json_snapshot!("machine_contract_init", normalized);
}

#[test]
#[ignore]
fn test_machine_contract_index() {
    let config_path = get_test_config_path().to_string_lossy().to_string();
    let args = vec![
        "--output",
        "json",
        "--config",
        &config_path,
        "index",
        "index",
        "--force",
    ];

    let output = run_command(&args).expect("Failed to run index command");
    validate_schema(&output, "index.index");

    let normalized = normalize_for_snapshot(output);
    insta::assert_json_snapshot!("machine_contract_index", normalized);
}

#[test]
#[ignore]
fn test_machine_contract_search() {
    let config_path = get_test_config_path().to_string_lossy().to_string();
    let args = vec![
        "--output",
        "json",
        "--config",
        &config_path,
        "query",
        "search",
        "productivity",
    ];

    let output = run_command(&args).expect("Failed to run search command");
    validate_schema(&output, "query.search");

    // Verify params
    assert_eq!(output["params"]["query"], "productivity");
    assert_eq!(output["params"]["limit"], 20);

    // Verify result structure
    assert!(output["result"]["total"].is_number());
    assert!(output["result"]["items"].is_array());
}

#[test]
#[ignore]
fn test_machine_contract_backlinks() {
    let config_path = get_test_config_path().to_string_lossy().to_string();
    let args = vec![
        "--output",
        "json",
        "--config",
        &config_path,
        "query",
        "backlinks",
        "Home",
    ];

    let output = run_command(&args).expect("Failed to run backlinks command");
    validate_schema(&output, "query.backlinks");

    // Verify params
    assert_eq!(output["params"]["note"], "Home");
    insta::assert_json_snapshot!("machine_contract_backlinks", normalize_for_snapshot(output));
}

#[test]
#[ignore]
fn test_machine_contract_links() {
    let config_path = get_test_config_path().to_string_lossy().to_string();
    let args = vec![
        "--output",
        "json",
        "--config",
        &config_path,
        "query",
        "links",
        "Home",
    ];

    let output = run_command(&args).expect("Failed to run links command");
    validate_schema(&output, "query.links");

    // Verify params
    assert_eq!(output["params"]["note"], "Home");
    insta::assert_json_snapshot!("machine_contract_links", normalize_for_snapshot(output));
}

#[test]
#[ignore]
fn test_machine_contract_unresolved() {
    let config_path = get_test_config_path().to_string_lossy().to_string();
    let args = vec![
        "--output",
        "json",
        "--config",
        &config_path,
        "query",
        "unresolved",
    ];

    let output = run_command(&args).expect("Failed to run unresolved command");
    validate_schema(&output, "query.unresolved");

    // Unresolved should have empty params
    assert!(output["params"].as_object().unwrap().is_empty());
    insta::assert_json_snapshot!(
        "machine_contract_unresolved",
        normalize_for_snapshot(output)
    );
}

#[test]
#[ignore]
fn test_machine_contract_tags() {
    let config_path = get_test_config_path().to_string_lossy().to_string();
    let args = vec![
        "--output",
        "json",
        "--config",
        &config_path,
        "query",
        "tags",
    ];

    let output = run_command(&args).expect("Failed to run tags command");
    validate_schema(&output, "query.tags");

    // Verify params structure
    assert!(output["params"]["tag"].is_null() || output["params"]["tag"].is_string());
    insta::assert_json_snapshot!("machine_contract_tags", normalize_for_snapshot(output));
}

#[test]
#[ignore]
fn test_machine_contract_tags_list() {
    let config_path = get_test_config_path().to_string_lossy().to_string();
    let args = vec![
        "--output",
        "json",
        "--config",
        &config_path,
        "query",
        "tags",
        "--list",
    ];

    let output = run_command(&args).expect("Failed to run tags --list command");
    validate_schema(&output, "query.tags");

    // Verify list flag is captured in params
    assert_eq!(output["params"]["list"], true);
    insta::assert_json_snapshot!("machine_contract_tags_list", normalize_for_snapshot(output));
}

#[test]
#[ignore]
fn test_machine_contract_bloat() {
    let config_path = get_test_config_path().to_string_lossy().to_string();
    let args = vec![
        "--output",
        "json",
        "--config",
        &config_path,
        "analyze",
        "bloat",
        "--threshold",
        "50000",
    ];

    let output = run_command(&args).expect("Failed to run bloat command");
    validate_schema(&output, "analyze.bloat");

    // Verify params
    assert_eq!(output["params"]["threshold"], 50000);
    insta::assert_json_snapshot!("machine_contract_bloat", normalize_for_snapshot(output));
}

#[test]
#[ignore]
fn test_machine_contract_related() {
    let config_path = get_test_config_path().to_string_lossy().to_string();
    let args = vec![
        "--output",
        "json",
        "--config",
        &config_path,
        "analyze",
        "related",
        "Home",
        "--limit",
        "5",
    ];

    let output = run_command(&args).expect("Failed to run related command");
    validate_schema(&output, "analyze.related");

    // Verify params
    assert_eq!(output["params"]["note"], "Home");
    assert_eq!(output["params"]["limit"], 5);
    insta::assert_json_snapshot!("machine_contract_related", normalize_for_snapshot(output));
}

#[test]
#[ignore]
fn test_machine_contract_orphans() {
    let config_path = get_test_config_path().to_string_lossy().to_string();
    let args = vec![
        "--output",
        "json",
        "--config",
        &config_path,
        "diagnose",
        "orphans",
    ];

    let output = run_command(&args).expect("Failed to run orphans command");
    validate_schema(&output, "diagnose.orphans");

    insta::assert_json_snapshot!("machine_contract_orphans", normalize_for_snapshot(output));
}

#[test]
#[ignore]
fn test_machine_contract_broken_links() {
    let config_path = get_test_config_path().to_string_lossy().to_string();
    let args = vec![
        "--output",
        "json",
        "--config",
        &config_path,
        "diagnose",
        "broken-links",
    ];

    let output = run_command(&args).expect("Failed to run broken-links command");
    validate_schema(&output, "diagnose.broken-links");

    insta::assert_json_snapshot!(
        "machine_contract_broken_links",
        normalize_for_snapshot(output)
    );
}

#[test]
#[ignore]
fn test_machine_contract_stats() {
    let config_path = get_test_config_path().to_string_lossy().to_string();
    let args = vec![
        "--output",
        "json",
        "--config",
        &config_path,
        "view",
        "stats",
    ];

    let output = run_command(&args).expect("Failed to run stats command");
    validate_schema(&output, "view.stats");

    // Verify result structure for stats
    let result = &output["result"];
    assert!(result["notes"].is_number());
    assert!(result["links"].is_number());
    assert!(result["tags"].is_number());
    assert!(result["chunks"].is_number());
    assert!(result["unresolved_links"].is_number());

    insta::assert_json_snapshot!("machine_contract_stats", normalize_for_snapshot(output));
}

#[test]
#[ignore]
fn test_machine_contract_describe() {
    let config_path = get_test_config_path().to_string_lossy().to_string();
    let args = vec![
        "--output",
        "json",
        "--config",
        &config_path,
        "view",
        "describe",
        "Home",
    ];

    let output = run_command(&args).expect("Failed to run describe command");
    validate_schema(&output, "view.describe");

    // Verify params
    assert_eq!(output["params"]["filename"], "Home");
    insta::assert_json_snapshot!("machine_contract_describe", normalize_for_snapshot(output));
}
