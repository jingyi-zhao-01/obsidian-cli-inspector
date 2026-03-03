use crate::e2e_tests::helpers::{
    get_test_config_path, normalize_for_snapshot, run_command_json, validate_schema,
};

// #[test]
// #[ignore]
// fn contract_tui_help() {
//     let (success, stdout, _stderr) = run_command(&["tui", "--help"]);
//     assert!(success, "tui --help should succeed");
//     insta::assert_snapshot!("contract_tui_help", stdout);
// }

#[test]
#[ignore]
fn machine_contract_tui() {
    let config_path = get_test_config_path().to_string_lossy().to_string();
    let args = vec!["--output", "json", "--config", &config_path, "tui"];

    let output = run_command_json(&args).expect("Failed to run tui command");
    validate_schema(&output, "tui");

    insta::assert_json_snapshot!("machine_contract_tui", normalize_for_snapshot(output));
}
