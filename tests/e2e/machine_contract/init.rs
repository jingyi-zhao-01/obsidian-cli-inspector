use crate::e2e_tests::helpers::{
    get_test_config_path, normalize_for_snapshot, run_command_json, validate_schema,
};

// #[test]
// #[ignore]
// fn contract_init_group_help() {
//     let (success, stdout, _stderr) = run_command(&["init", "--help"]);
//     assert!(success, "init --help should succeed");
//     insta::assert_snapshot!("contract_init_help", stdout);
// }

// #[test]
// #[ignore]
// fn contract_init_command_help() {
//     let (success, stdout, _stderr) = run_command(&["init", "init", "--help"]);
//     assert!(success, "init init --help should succeed");
//     insta::assert_snapshot!("contract_init_init_help", stdout);
// }

#[test]
#[ignore]
fn machine_contract_init() {
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

    let output = run_command_json(&args).expect("Failed to run init command");
    validate_schema(&output, "init.init");

    insta::assert_json_snapshot!("machine_contract_init", normalize_for_snapshot(output));
}
