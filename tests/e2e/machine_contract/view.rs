use crate::e2e_tests::helpers::{
    get_test_config_path, normalize_for_snapshot, run_command_json, validate_schema,
};

// #[test]
// #[ignore]
// fn contract_view_group_help() {
//     let (success, stdout, _stderr) = run_command(&["view", "--help"]);
//     assert!(success, "view --help should succeed");
//     insta::assert_snapshot!("contract_view_help", stdout);
// }

// #[test]
// #[ignore]
// fn contract_view_stats_help() {
//     let (success, stdout, _stderr) = run_command(&["view", "stats", "--help"]);
//     assert!(success, "view stats --help should succeed");
//     insta::assert_snapshot!("contract_view_stats_help", stdout);
// }

// #[test]
// #[ignore]
// fn contract_view_describe_help() {
//     let (success, stdout, _stderr) = run_command(&["view", "describe", "--help"]);
//     assert!(success, "view describe --help should succeed");
//     insta::assert_snapshot!("contract_view_describe_help", stdout);
// }

#[test]
#[ignore]
fn machine_contract_stats() {
    let config_path = get_test_config_path().to_string_lossy().to_string();
    let args = vec![
        "--output",
        "json",
        "--config",
        &config_path,
        "view",
        "stats",
    ];

    let output = run_command_json(&args).expect("Failed to run stats command");
    validate_schema(&output, "view.stats");

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
fn machine_contract_describe() {
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

    let output = run_command_json(&args).expect("Failed to run describe command");
    validate_schema(&output, "view.describe");

    assert_eq!(output["params"]["filename"], "Home");
    insta::assert_json_snapshot!("machine_contract_describe", normalize_for_snapshot(output));
}
