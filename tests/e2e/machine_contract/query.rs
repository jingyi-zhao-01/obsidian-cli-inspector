use crate::e2e_tests::helpers::{
    get_test_config_path, normalize_for_snapshot, run_command, run_command_json, validate_schema,
};

#[test]
#[ignore]
fn contract_query_group_help() {
    let (success, stdout, _stderr) = run_command(&["query", "--help"]);
    assert!(success, "query --help should succeed");
    insta::assert_snapshot!("contract_query_help", stdout);
}

#[test]
#[ignore]
fn contract_query_search_help() {
    let (success, stdout, _stderr) = run_command(&["query", "search", "--help"]);
    assert!(success, "query search --help should succeed");
    insta::assert_snapshot!("contract_query_search_help", stdout);
}

#[test]
#[ignore]
fn contract_query_backlinks_help() {
    let (success, stdout, _stderr) = run_command(&["query", "backlinks", "--help"]);
    assert!(success, "query backlinks --help should succeed");
    insta::assert_snapshot!("contract_query_backlinks_help", stdout);
}

#[test]
#[ignore]
fn contract_query_links_help() {
    let (success, stdout, _stderr) = run_command(&["query", "links", "--help"]);
    assert!(success, "query links --help should succeed");
    insta::assert_snapshot!("contract_query_links_help", stdout);
}

#[test]
#[ignore]
fn contract_query_unresolved_help() {
    let (success, stdout, _stderr) = run_command(&["query", "unresolved", "--help"]);
    assert!(success, "query unresolved --help should succeed");
    insta::assert_snapshot!("contract_query_unresolved_help", stdout);
}

#[test]
#[ignore]
fn contract_query_tags_help() {
    let (success, stdout, _stderr) = run_command(&["query", "tags", "--help"]);
    assert!(success, "query tags --help should succeed");
    insta::assert_snapshot!("contract_query_tags_help", stdout);
}

#[test]
#[ignore]
fn machine_contract_search() {
    let config_path = get_test_config_path().to_string_lossy().to_string();
    let args = vec![
        "--output",
        "json",
        "--config",
        &config_path,
        "query",
        "search",
        "productivity",
    ];

    let output = run_command_json(&args).expect("Failed to run search command");
    validate_schema(&output, "query.search");

    assert_eq!(output["params"]["query"], "productivity");
    assert_eq!(output["params"]["limit"], 20);
    assert!(output["result"]["total"].is_number());
    assert!(output["result"]["items"].is_array());

    insta::assert_json_snapshot!("machine_contract_search", normalize_for_snapshot(output));
}

#[test]
#[ignore]
fn machine_contract_backlinks() {
    let config_path = get_test_config_path().to_string_lossy().to_string();
    let args = vec![
        "--output",
        "json",
        "--config",
        &config_path,
        "query",
        "backlinks",
        "Home",
    ];

    let output = run_command_json(&args).expect("Failed to run backlinks command");
    validate_schema(&output, "query.backlinks");

    assert_eq!(output["params"]["note"], "Home");
    insta::assert_json_snapshot!("machine_contract_backlinks", normalize_for_snapshot(output));
}

#[test]
#[ignore]
fn machine_contract_links() {
    let config_path = get_test_config_path().to_string_lossy().to_string();
    let args = vec![
        "--output",
        "json",
        "--config",
        &config_path,
        "query",
        "links",
        "Home",
    ];

    let output = run_command_json(&args).expect("Failed to run links command");
    validate_schema(&output, "query.links");

    assert_eq!(output["params"]["note"], "Home");
    insta::assert_json_snapshot!("machine_contract_links", normalize_for_snapshot(output));
}

#[test]
#[ignore]
fn machine_contract_unresolved() {
    let config_path = get_test_config_path().to_string_lossy().to_string();
    let args = vec![
        "--output",
        "json",
        "--config",
        &config_path,
        "query",
        "unresolved",
    ];

    let output = run_command_json(&args).expect("Failed to run unresolved command");
    validate_schema(&output, "query.unresolved");

    assert!(output["params"]
        .as_object()
        .expect("params should be object")
        .is_empty());
    insta::assert_json_snapshot!(
        "machine_contract_unresolved",
        normalize_for_snapshot(output)
    );
}

#[test]
#[ignore]
fn machine_contract_tags() {
    let config_path = get_test_config_path().to_string_lossy().to_string();
    let args = vec![
        "--output",
        "json",
        "--config",
        &config_path,
        "query",
        "tags",
    ];

    let output = run_command_json(&args).expect("Failed to run tags command");
    validate_schema(&output, "query.tags");

    assert!(output["params"]["tag"].is_null() || output["params"]["tag"].is_string());
    insta::assert_json_snapshot!("machine_contract_tags", normalize_for_snapshot(output));
}

#[test]
#[ignore]
fn machine_contract_tags_list() {
    let config_path = get_test_config_path().to_string_lossy().to_string();
    let args = vec![
        "--output",
        "json",
        "--config",
        &config_path,
        "query",
        "tags",
        "--list",
    ];

    let output = run_command_json(&args).expect("Failed to run tags --list command");
    validate_schema(&output, "query.tags");

    assert_eq!(output["params"]["list"], true);
    insta::assert_json_snapshot!("machine_contract_tags_list", normalize_for_snapshot(output));
}
