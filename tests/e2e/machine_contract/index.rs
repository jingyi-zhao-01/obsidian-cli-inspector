use crate::e2e_tests::helpers::{
    get_test_config_path, normalize_for_snapshot, run_command_json, validate_schema,
};

// #[test]
// #[ignore]
// fn contract_index_group_help() {
//     let (success, stdout, _stderr) = run_command(&["index", "--help"]);
//     assert!(success, "index --help should succeed");
//     insta::assert_snapshot!("contract_index_help", stdout);
// }

// #[test]
// #[ignore]
// fn contract_index_command_help() {
//     let (success, stdout, _stderr) = run_command(&["index", "index", "--help"]);
//     assert!(success, "index index --help should succeed");
//     insta::assert_snapshot!("contract_index_index_help", stdout);
// }

#[test]
#[ignore]
fn machine_contract_index() {
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

    let output = run_command_json(&args).expect("Failed to run index command");
    validate_schema(&output, "index.index");

    insta::assert_json_snapshot!("machine_contract_index", normalize_for_snapshot(output));
}
