use serde_json::Value;
use std::path::PathBuf;
use std::process::Command;

pub fn get_test_config_path() -> PathBuf {
    PathBuf::from("test-config.toml")
}

pub fn run_command(args: &[&str]) -> (bool, String, String) {
    let output = Command::new("./target/debug/obsidian-cli-inspector")
        .args(args)
        .current_dir(".")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    (output.status.success(), stdout, stderr)
}

pub fn run_command_json(args: &[&str]) -> Result<Value, String> {
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
    let lines: Vec<&str> = stdout.lines().collect();
    if let Some(last_line) = lines.last() {
        if last_line.trim_start().starts_with('{') {
            return serde_json::from_str(last_line)
                .map_err(|e| format!("Failed to parse JSON: {e:?}"));
        }

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

pub fn validate_schema(json: &Value, expected_command: &str) {
    assert!(json.is_object(), "Response must be a JSON object");

    assert_eq!(json["version"], "1.0", "Schema version must be 1.0");
    assert_eq!(json["command"], expected_command, "Command name mismatch");
    assert!(
        json["timestamp"].is_string(),
        "Timestamp must be present and string"
    );
    assert!(json["params"].is_object(), "Params must be an object");
    assert!(json["result"].is_object(), "Result must be an object");
    assert!(json["meta"].is_object(), "Meta must be an object");

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

pub fn normalize_for_snapshot(mut json: Value) -> Value {
    json["timestamp"] = Value::String("TIMESTAMP".to_string());

    if let Some(meta) = json.get_mut("meta") {
        if let Some(meta_obj) = meta.as_object_mut() {
            meta_obj.insert("query_time_ms".to_string(), Value::Number(0.into()));
        }
    }

    json
}

pub fn bootstrap_test_db() {
    let config_path = get_test_config_path().to_string_lossy().to_string();

    let init_args = vec![
        "--output",
        "json",
        "--config",
        &config_path,
        "init",
        "init",
        "--force",
    ];
    run_command_json(&init_args).expect("Failed to initialize test database");

    let index_args = vec![
        "--output",
        "json",
        "--config",
        &config_path,
        "index",
        "index",
        "--force",
    ];
    run_command_json(&index_args).expect("Failed to index test database");
}
