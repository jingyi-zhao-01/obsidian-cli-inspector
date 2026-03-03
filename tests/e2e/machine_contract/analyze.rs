use crate::e2e_tests::helpers::{
    get_test_config_path, normalize_for_snapshot, run_command_json, validate_schema,
};

// #[test]
// #[ignore]
// fn contract_analyze_group_help() {
//     let (success, stdout, _stderr) = run_command(&["analyze", "--help"]);
//     assert!(success, "analyze --help should succeed");
//     insta::assert_snapshot!("contract_analyze_help", stdout);
// }

// #[test]
// #[ignore]
// fn contract_analyze_bloat_help() {
//     let (success, stdout, _stderr) = run_command(&["analyze", "bloat", "--help"]);
//     assert!(success, "analyze bloat --help should succeed");
//     insta::assert_snapshot!("contract_analyze_bloat_help", stdout);
// }

// #[test]
// #[ignore]
// fn contract_analyze_related_help() {
//     let (success, stdout, _stderr) = run_command(&["analyze", "related", "--help"]);
//     assert!(success, "analyze related --help should succeed");
//     insta::assert_snapshot!("contract_analyze_related_help", stdout);
// }

#[test]
#[ignore]
fn machine_contract_bloat() {
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

    let output = run_command_json(&args).expect("Failed to run bloat command");
    validate_schema(&output, "analyze.bloat");

    assert_eq!(output["params"]["threshold"], 50000);
    insta::assert_json_snapshot!("machine_contract_bloat", normalize_for_snapshot(output));
}

#[test]
#[ignore]
fn machine_contract_related() {
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

    let output = run_command_json(&args).expect("Failed to run related command");
    validate_schema(&output, "analyze.related");

    assert_eq!(output["params"]["note"], "Home");
    assert_eq!(output["params"]["limit"], 5);
    insta::assert_json_snapshot!("machine_contract_related", normalize_for_snapshot(output));
}
