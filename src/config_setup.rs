use anyhow::{Context, Result};
use std::path::PathBuf;

use crate::config::Config;

/// Default config template that will be seeded on first run
pub const DEFAULT_CONFIG: &str = include_str!("../template-config.toml");

pub fn ensure_config_exists(path: &PathBuf) -> Result<PathBuf> {
    // Skip creation if config already exists
    if !path.exists() {
        // Create parent directories if they don't exist
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).context("Failed to create config directory")?;
        }

        // Write default config
        std::fs::write(path, DEFAULT_CONFIG).context("Failed to write default config file")?;

        println!(
            "Created default config at: {}\n\
             Please edit this file and set your vault_path.",
            path.display()
        );
    }

    Ok(path.clone())
}

/// Ensure the config file exists and return its path (does not parse the file)
pub fn config_file_path(config_path: Option<PathBuf>) -> Result<PathBuf> {
    let path = config_path.unwrap_or_else(|| {
        let mut p = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        p.push("obsidian-cli-inspector");
        p.push("config.toml");
        p
    });

    ensure_config_exists(&path)
}

/// Interactively prompt the user to confirm or override key config values and
/// persist them. Prompts: vault_path, database_path, log_path — each shows a
/// sensible default and accepts ENTER to keep it.
pub fn interactive_config_setup(config_path: Option<PathBuf>) -> Result<Config> {
    use std::io::{self, Write};

    let config_file = config_file_path(config_path)?;

    // Load current config (template will already contain a placeholder)
    let mut cfg = Config::from_file(&config_file).context("Failed to parse config file")?;

    // 1) Vault path (required)
    let current_vault = cfg.vault_path.to_string_lossy();
    print!("Vault path [{}]: ", current_vault);
    io::stdout().flush()?;
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input)?;
    let val = input.trim();
    if !val.is_empty() {
        cfg.vault_path = PathBuf::from(val);
    }

    // 2) Database path (optional) — show explicit default
    let db_default = cfg
        .database_path
        .clone()
        .unwrap_or_else(|| cfg.database_path());
    print!("Database path [{}]: ", db_default.display());
    io::stdout().flush()?;
    input.clear();
    let _ = io::stdin().read_line(&mut input)?;
    let val = input.trim();
    if !val.is_empty() {
        cfg.database_path = Some(PathBuf::from(val));
    } else {
        // store the explicit default so config.toml contains the concrete path
        cfg.database_path = Some(db_default);
    }

    // 3) Log path (optional) — show explicit default
    let log_default = cfg.log_path.clone().unwrap_or_else(|| cfg.log_dir());
    print!("Log path [{}]: ", log_default.display());
    io::stdout().flush()?;
    input.clear();
    let _ = io::stdin().read_line(&mut input)?;
    let val = input.trim();
    if !val.is_empty() {
        cfg.log_path = Some(PathBuf::from(val));
    } else {
        // store the explicit default so config.toml contains the concrete path
        cfg.log_path = Some(log_default);
    }

    // Persist updated config back to disk (overwrite)
    let toml = toml::to_string_pretty(&cfg).context("Failed to serialize config to TOML")?;
    std::fs::write(&config_file, toml).context("Failed to write updated config file")?;

    println!("Updated config at: {}", config_file.display());

    Ok(cfg)
}

pub fn load_config(config_path: Option<PathBuf>) -> Result<Config> {
    let path = config_path.unwrap_or_else(|| {
        let mut p = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        p.push("obsidian-cli-inspector");
        p.push("config.toml");
        p
    });

    // Ensure config file exists (create default if needed)
    let config_file_path = ensure_config_exists(&path)?;

    Config::from_file(&config_file_path).context("Failed to load config file")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_default_config_includes_vault_path_placeholder() {
        // Verify the template contains the vault_path placeholder
        assert!(DEFAULT_CONFIG.contains("vault_path"));
        assert!(DEFAULT_CONFIG.contains("\"/path/to/your/obsidian/vault\""));
    }

    #[test]
    fn test_default_config_includes_all_sections() {
        // Verify all expected config sections are present
        assert!(DEFAULT_CONFIG.contains("[exclude]"));
        assert!(DEFAULT_CONFIG.contains("[search]"));
        assert!(DEFAULT_CONFIG.contains("[graph]"));
    }

    #[test]
    fn test_ensure_config_returns_existing_path() {
        // Create a temp directory with an existing config file
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.toml");
        fs::write(&config_path, "vault_path = \"/test/vault\"").unwrap();

        let result = ensure_config_exists(&config_path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), config_path);
    }

    #[test]
    fn test_ensure_config_creates_default_config() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.toml");

        let result = ensure_config_exists(&config_path);
        assert!(result.is_ok());
        assert!(config_path.exists());

        let content = fs::read_to_string(&config_path).unwrap();
        assert!(content.contains("vault_path"));
    }

    #[test]
    fn test_ensure_config_creates_parent_directories() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("subdir").join("config.toml");

        let result = ensure_config_exists(&config_path);
        assert!(result.is_ok());
        assert!(config_path.exists());
    }

    #[test]
    fn test_ensure_config_does_not_overwrite_existing() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.toml");
        let original_content = "vault_path = \"/my/custom/vault\"";
        fs::write(&config_path, original_content).unwrap();

        let result = ensure_config_exists(&config_path);
        assert!(result.is_ok());

        let content = fs::read_to_string(&config_path).unwrap();
        assert_eq!(content, original_content);
    }

    #[test]
    fn test_load_config_with_existing_file() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.toml");
        fs::write(
            &config_path,
            r#"vault_path = "/test/vault"
[search]
default_limit = 50
"#,
        )
        .unwrap();

        let config = load_config(Some(config_path)).unwrap();
        assert_eq!(config.vault_path.to_string_lossy(), "/test/vault");
        assert_eq!(config.search.default_limit, 50);
    }

    #[test]
    fn test_load_config_creates_default_when_missing() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.toml");

        // Should create default config
        let result = load_config(Some(config_path.clone()));
        assert!(result.is_ok());
        assert!(config_path.exists());
    }

    #[test]
    fn test_default_config_has_search_section() {
        // Verify search config section is present
        assert!(DEFAULT_CONFIG.contains("[search]"));
        assert!(DEFAULT_CONFIG.contains("default_limit"));
    }

    #[test]
    fn test_default_config_has_exclude_section() {
        // Verify exclude config section is present
        assert!(DEFAULT_CONFIG.contains("[exclude]"));
        assert!(DEFAULT_CONFIG.contains("patterns"));
    }
}
