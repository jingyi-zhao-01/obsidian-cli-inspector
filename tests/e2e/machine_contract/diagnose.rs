use crate::e2e_tests::helpers::{
    get_test_config_path, normalize_for_snapshot, run_command_json, validate_schema,
};

// #[test]
// #[ignore]
// fn contract_diagnose_group_help() {
//     let (success, stdout, _stderr) = run_command(&["diagnose", "--help"]);
//     assert!(success, "diagnose --help should succeed");
//     insta::assert_snapshot!("contract_diagnose_help", stdout);
// }

// #[test]
// #[ignore]
// fn contract_diagnose_orphans_help() {
//     let (success, stdout, _stderr) = run_command(&["diagnose", "orphans", "--help"]);
//     assert!(success, "diagnose orphans --help should succeed");
//     insta::assert_snapshot!("contract_diagnose_orphans_help", stdout);
// }

// #[test]
// #[ignore]
// fn contract_diagnose_broken_links_help() {
//     let (success, stdout, _stderr) = run_command(&["diagnose", "broken-links", "--help"]);
//     assert!(success, "diagnose broken-links --help should succeed");
//     insta::assert_snapshot!("contract_diagnose_broken_links_help", stdout);
// }

#[test]
#[ignore]
fn machine_contract_orphans() {
    let config_path = get_test_config_path().to_string_lossy().to_string();
    let args = vec![
        "--output",
        "json",
        "--config",
        &config_path,
        "diagnose",
        "orphans",
    ];

    let output = run_command_json(&args).expect("Failed to run orphans command");
    validate_schema(&output, "diagnose.orphans");

    insta::assert_json_snapshot!("machine_contract_orphans", normalize_for_snapshot(output));
}

#[test]
#[ignore]
fn machine_contract_broken_links() {
    let config_path = get_test_config_path().to_string_lossy().to_string();
    let args = vec![
        "--output",
        "json",
        "--config",
        &config_path,
        "diagnose",
        "broken-links",
    ];

    let output = run_command_json(&args).expect("Failed to run broken-links command");
    validate_schema(&output, "diagnose.broken-links");

    insta::assert_json_snapshot!(
        "machine_contract_broken_links",
        normalize_for_snapshot(output)
    );
}
