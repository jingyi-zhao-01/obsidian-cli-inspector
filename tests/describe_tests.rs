
mod common;

use anyhow::Result;
use obsidian_cli_inspector::commands::*;

// CLI equivalent: cargo run -- describe "Home.md"
#[test]
fn test_describe_note() -> Result<()> {
    let (_vault_dir, _db_dir, config) = common::setup_test_config()?;

    // Setup
    initialize_database(&config, false, None)?;
    index_vault(&config, false, false, false, None)?;

    // Test describe command
    get_note_describe(&config, "Home.md", None)?;

    Ok(())
}

// CLI equivalent: cargo run -- describe "NonExistent.md"
#[test]
fn test_describe_nonexistent_note() -> Result<()> {
    let (_vault_dir, _db_dir, config) = common::setup_test_config()?;

    // Setup
    initialize_database(&config, false, None)?;
    index_vault(&config, false, false, false, None)?;

    // Test describe for non-existent note
    get_note_describe(&config, "NonExistent.md", None)?;

    Ok(())
}

// Test partial matching
#[test]
fn test_describe_partial_match() -> Result<()> {
    let (_vault_dir, _db_dir, config) = common::setup_test_config()?;

    // Setup
    initialize_database(&config, false, None)?;
    index_vault(&config, false, false, false, None)?;

    // Test describe with partial match
    get_note_describe(&config, "Home", None)?;

    Ok(())
}

// Test describe note with frontmatter
#[test]
fn test_describe_note_with_frontmatter() -> Result<()> {
    let (_vault_dir, _db_dir, config) = common::setup_test_config()?;

    // Setup
    initialize_database(&config, false, None)?;
    index_vault(&config, false, false, false, None)?;

    // Test describe command - notes with frontmatter
    get_note_describe(&config, "Projects.md", None)?;

    Ok(())
}
