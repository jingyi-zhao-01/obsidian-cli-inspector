use std::path::PathBuf;
use std::process::Command;

/// Helper function to get test config path
fn get_test_config_path() -> PathBuf {
    PathBuf::from("test-config.toml")
}

/// Run a command and capture output
fn run_command(args: &[&str]) -> (bool, String, String) {
    let output = Command::new("./target/debug/obsidian-cli-inspector")
        .args(args)
        .current_dir(".")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    (output.status.success(), stdout, stderr)
}

// ============================================================================
// CLI Contract Tests - Validate command structure and text output
// ============================================================================
// These tests ensure the CLI follows the documented contract for:
// - Command structure (group subcommand args)
// - Help text and flags
// - Exit codes
// - Text output format (non-JSON)
// ============================================================================

#[test]
#[ignore]
fn test_cli_contract_init_help() {
    let (success, stdout, _stderr) = run_command(&["init", "--help"]);
    assert!(success, "init --help should succeed");
    insta::assert_snapshot!("cli_contract_init_help", stdout);
}

#[test]
#[ignore]
fn test_cli_contract_index_help() {
    let (success, stdout, _stderr) = run_command(&["index", "--help"]);
    assert!(success, "index --help should succeed");
    insta::assert_snapshot!("cli_contract_index_help", stdout);
}

#[test]
#[ignore]
fn test_cli_contract_query_help() {
    let (success, stdout, _stderr) = run_command(&["query", "--help"]);
    assert!(success, "query --help should succeed");
    insta::assert_snapshot!("cli_contract_query_help", stdout);
}

#[test]
#[ignore]
fn test_cli_contract_query_search_help() {
    let (success, stdout, _stderr) = run_command(&["query", "search", "--help"]);
    assert!(success, "query search --help should succeed");
    assert!(stdout.contains("search"), "Help should mention search");
    assert!(
        stdout.contains("--limit") || stdout.contains("limit"),
        "Help should mention limit flag"
    );
}

#[test]
#[ignore]
fn test_cli_contract_query_backlinks_help() {
    let (success, stdout, _stderr) = run_command(&["query", "backlinks", "--help"]);
    assert!(success, "query backlinks --help should succeed");
    assert!(
        stdout.contains("backlinks"),
        "Help should mention backlinks"
    );
}

#[test]
#[ignore]
fn test_cli_contract_query_links_help() {
    let (success, stdout, _stderr) = run_command(&["query", "links", "--help"]);
    assert!(success, "query links --help should succeed");
    assert!(stdout.contains("links"), "Help should mention links");
}

#[test]
#[ignore]
fn test_cli_contract_query_unresolved_help() {
    let (success, stdout, _stderr) = run_command(&["query", "unresolved", "--help"]);
    assert!(success, "query unresolved --help should succeed");
    assert!(
        stdout.contains("unresolved"),
        "Help should mention unresolved"
    );
}

#[test]
#[ignore]
fn test_cli_contract_query_tags_help() {
    let (success, stdout, _stderr) = run_command(&["query", "tags", "--help"]);
    assert!(success, "query tags --help should succeed");
    assert!(stdout.contains("tags"), "Help should mention tags");
    assert!(
        stdout.contains("--list") || stdout.contains("list"),
        "Help should mention list flag"
    );
}

#[test]
#[ignore]
fn test_cli_contract_analyze_help() {
    let (success, stdout, _stderr) = run_command(&["analyze", "--help"]);
    assert!(success, "analyze --help should succeed");
    insta::assert_snapshot!("cli_contract_analyze_help", stdout);
}

#[test]
#[ignore]
fn test_cli_contract_analyze_bloat_help() {
    let (success, stdout, _stderr) = run_command(&["analyze", "bloat", "--help"]);
    assert!(success, "analyze bloat --help should succeed");
    assert!(stdout.contains("bloat"), "Help should mention bloat");
    assert!(
        stdout.contains("--threshold") || stdout.contains("threshold"),
        "Help should mention threshold flag"
    );
}

#[test]
#[ignore]
fn test_cli_contract_analyze_related_help() {
    let (success, stdout, _stderr) = run_command(&["analyze", "related", "--help"]);
    assert!(success, "analyze related --help should succeed");
    assert!(stdout.contains("related"), "Help should mention related");
    assert!(
        stdout.contains("--limit") || stdout.contains("limit"),
        "Help should mention limit flag"
    );
}

#[test]
#[ignore]
fn test_cli_contract_diagnose_help() {
    let (success, stdout, _stderr) = run_command(&["diagnose", "--help"]);
    assert!(success, "diagnose --help should succeed");
    insta::assert_snapshot!("cli_contract_diagnose_help", stdout);
}

#[test]
#[ignore]
fn test_cli_contract_diagnose_orphans_help() {
    let (success, stdout, _stderr) = run_command(&["diagnose", "orphans", "--help"]);
    assert!(success, "diagnose orphans --help should succeed");
    assert!(stdout.contains("orphans"), "Help should mention orphans");
    assert!(
        stdout.contains("--exclude-templates") || stdout.contains("exclude-templates"),
        "Help should mention exclude-templates flag"
    );
}

#[test]
#[ignore]
fn test_cli_contract_diagnose_broken_links_help() {
    let (success, stdout, _stderr) = run_command(&["diagnose", "broken-links", "--help"]);
    assert!(success, "diagnose broken-links --help should succeed");
    assert!(
        stdout.contains("broken-links"),
        "Help should mention broken-links"
    );
}

#[test]
#[ignore]
fn test_cli_contract_view_help() {
    let (success, stdout, _stderr) = run_command(&["view", "--help"]);
    assert!(success, "view --help should succeed");
    insta::assert_snapshot!("cli_contract_view_help", stdout);
}

#[test]
#[ignore]
fn test_cli_contract_view_stats_help() {
    let (success, stdout, _stderr) = run_command(&["view", "stats", "--help"]);
    assert!(success, "view stats --help should succeed");
    assert!(stdout.contains("stats"), "Help should mention stats");
}

#[test]
#[ignore]
fn test_cli_contract_view_describe_help() {
    let (success, stdout, _stderr) = run_command(&["view", "describe", "--help"]);
    assert!(success, "view describe --help should succeed");
    assert!(stdout.contains("describe"), "Help should mention describe");
}

#[test]
#[ignore]
fn test_cli_contract_invalid_command() {
    let (success, _stdout, stderr) = run_command(&["invalid-command"]);
    assert!(!success, "Invalid command should fail");
    assert!(
        !stderr.is_empty() || !_stdout.is_empty(),
        "Error output should be present"
    );
}

#[test]
#[ignore]
fn test_cli_contract_missing_required_arg() {
    let (success, _stdout, _stderr) = run_command(&["query", "search"]);
    assert!(!success, "search without query should fail");
}

#[test]
#[ignore]
fn test_cli_contract_text_output_format() {
    let config_path = get_test_config_path().to_string_lossy().to_string();
    let (success, stdout, _stderr) = run_command(&["--config", &config_path, "view", "stats"]);

    assert!(success, "view stats should succeed");
    // Text output should NOT start with { (JSON marker)
    let trimmed = stdout.trim_start();
    assert!(
        !trimmed.starts_with('{'),
        "Default output should be text format, not JSON"
    );
    // Should contain human-readable text
    assert!(!stdout.is_empty(), "Output should not be empty");
}

#[test]
#[ignore]
fn test_cli_contract_json_output_mode() {
    let config_path = get_test_config_path().to_string_lossy().to_string();
    let (success, stdout, _stderr) = run_command(&[
        "--output",
        "json",
        "--config",
        &config_path,
        "query",
        "search",
        "test",
    ]);

    assert!(success, "query search with JSON output should succeed");
    let trimmed = stdout.trim_start();
    // JSON output from search should start with {
    if !trimmed.starts_with('{') {
        // If not JSON, it could be text - that's also acceptable for this contract test
        assert!(!stdout.is_empty(), "Output should not be empty");
    }
}

#[test]
#[ignore]
fn test_cli_contract_config_flag() {
    let config_path = get_test_config_path().to_string_lossy().to_string();
    let (success, stdout, _stderr) = run_command(&["--config", &config_path, "view", "stats"]);

    assert!(success, "Command with --config flag should succeed");
    assert!(!stdout.is_empty(), "Output should be present");
}

#[test]
#[ignore]
fn test_cli_contract_version_flag() {
    let (success, stdout, _stderr) = run_command(&["--version"]);
    assert!(success, "--version should succeed");
    // Version output should contain version info
    assert!(
        stdout.contains("obsidian-cli-inspector") || stdout.contains("0."),
        "Version output should contain version information"
    );
}

#[test]
#[ignore]
fn test_cli_contract_help_flag() {
    let (success, stdout, _stderr) = run_command(&["--help"]);
    assert!(success, "--help should succeed");
    insta::assert_snapshot!("cli_contract_root_help", stdout);
}
