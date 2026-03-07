use crate::e2e_tests::helpers::{
    get_test_config_path, run_command, run_command_json, validate_schema,
};

// #[test]
// #[ignore]
// fn contract_root_help() {
//     let (success, stdout, _stderr) = run_command(&["--help"]);
//     assert!(success, "--help should succeed");
//     insta::assert_snapshot!("contract_root_help", stdout);
// }

#[test]
#[ignore]
fn contract_invalid_command() {
    let (success, _stdout, stderr) = run_command(&["invalid-command"]);
    assert!(!success, "Invalid command should fail");
    assert!(!stderr.is_empty(), "Error output should be present");
}

#[test]
#[ignore]
fn contract_missing_required_arg() {
    let (success, _stdout, _stderr) = run_command(&["search", "notes"]);
    assert!(!success, "search without query should fail");
}

#[test]
#[ignore]
fn contract_text_output_format() {
    let config_path = get_test_config_path().to_string_lossy().to_string();
    let (success, stdout, _stderr) = run_command(&["--config", &config_path, "view", "stats"]);

    assert!(success, "view stats should succeed");
    let trimmed = stdout.trim_start();
    assert!(
        !trimmed.starts_with('{'),
        "Default output should be text format, not JSON"
    );
    assert!(!stdout.is_empty(), "Output should not be empty");
}

#[test]
#[ignore]
fn contract_json_output_mode() {
    let config_path = get_test_config_path().to_string_lossy().to_string();
    let args = vec![
        "--output",
        "json",
        "--config",
        &config_path,
        "search",
        "notes",
        "test",
    ];

    let output = run_command_json(&args).expect("search notes with JSON output should succeed");
    validate_schema(&output, "search.notes");
}

#[test]
#[ignore]
fn contract_config_flag() {
    let config_path = get_test_config_path().to_string_lossy().to_string();
    let (success, stdout, _stderr) = run_command(&["--config", &config_path, "view", "stats"]);

    assert!(success, "Command with --config flag should succeed");
    assert!(!stdout.is_empty(), "Output should be present");
}
