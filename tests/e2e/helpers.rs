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
    parse_json_output(&stdout)
}

fn parse_json_output(stdout: &str) -> Result<Value, String> {
    // Find the first line that starts with '{' and parse from there
    // This handles pretty-printed JSON that may be preceded by non-JSON output
    let lines: Vec<&str> = stdout.lines().collect();
    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim_start();
        if trimmed.starts_with('{') {
            // Join all lines from this point to reconstruct the JSON
            let json_str = lines[i..].join("\n");
            return serde_json::from_str(&json_str)
                .map_err(|e| format!("Failed to parse JSON: {e:?}"));
        }
    }

    // Fallback: try to find JSON within a line (for compact JSON embedded in other text)
    for line in &lines {
        if let Some(json_start) = line.find('{') {
            if let Some(json_end) = line.rfind('}') {
                if json_start < json_end {
                    let json_str = &line[json_start..=json_end];
                    return serde_json::from_str(json_str)
                        .map_err(|e| format!("Failed to parse JSON: {e:?}"));
                }
            }
        }
    }

    Err("No valid JSON response found".to_string())
}

#[cfg(test)]
mod tests {
    use super::{parse_json_output, run_command_json};

    #[test]
    fn parse_json_output_pretty_with_prefix() {
        let stdout = "Starting...\n  {\n    \"command\": \"query.search\",\n    \"result\": {\"status\": \"ok\"}\n  }\n";
        let parsed = parse_json_output(stdout).expect("should parse pretty JSON");
        assert_eq!(parsed["command"], "query.search");
        assert_eq!(parsed["result"]["status"], "ok");
    }

    #[test]
    fn parse_json_output_inline_json() {
        let stdout = "log: {\"command\":\"view.stats\",\"result\":{\"status\":\"ok\"}}";
        let parsed = parse_json_output(stdout).expect("should parse inline JSON");
        assert_eq!(parsed["command"], "view.stats");
    }

    #[test]
    fn parse_json_output_returns_error_when_missing() {
        let error = parse_json_output("no json here").expect_err("should fail without JSON");
        assert!(error.contains("No valid JSON"));
    }

    #[cfg(unix)]
    #[test]
    fn run_command_json_parses_stub_binary_output() {
        use std::fs;
        use std::os::unix::fs::PermissionsExt;

        let bin_dir = std::path::Path::new("target/debug");
        fs::create_dir_all(bin_dir).expect("failed to create binary directory");

        let bin_path = bin_dir.join("obsidian-cli-inspector");
        let script = "#!/bin/sh\necho '{\"command\":\"stub\",\"result\":{\"status\":\"ok\"}}'\n";
        fs::write(&bin_path, script).expect("failed to write stub binary");

        let mut perms = fs::metadata(&bin_path)
            .expect("failed to read stub metadata")
            .permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&bin_path, perms).expect("failed to set stub permissions");

        let parsed = run_command_json(&[]).expect("should parse stub output");
        assert_eq!(parsed["command"], "stub");

        let _ = fs::remove_file(&bin_path);
    }

}


pub fn validate_schema(json: &Value, expected_command: &str) {
    assert!(json.is_object(), "Response must be a JSON object");

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
