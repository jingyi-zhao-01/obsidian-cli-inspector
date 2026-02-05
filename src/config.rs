use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub vault_path: PathBuf,
    #[serde(default)]
    pub database_path: Option<PathBuf>,
    #[serde(default)]
    pub log_path: Option<PathBuf>,
    #[serde(default)]
    pub exclude: ExcludeConfig,
    #[serde(default)]
    pub search: SearchConfig,
    #[serde(default)]
    pub graph: GraphConfig,
    #[serde(default)]
    pub llm: Option<LlmConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExcludeConfig {
    #[serde(default = "default_exclude_patterns")]
    pub patterns: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchConfig {
    #[serde(default = "default_search_limit")]
    pub default_limit: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphConfig {
    #[serde(default = "default_max_depth")]
    pub max_depth: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    pub api_url: String,
    pub model: String,
    #[serde(default = "default_timeout")]
    pub timeout_seconds: u64,
}

fn default_exclude_patterns() -> Vec<String> {
    vec![
        ".obsidian/".to_string(),
        ".git/".to_string(),
        ".trash/".to_string(),
    ]
}

fn default_search_limit() -> usize {
    20
}

fn default_max_depth() -> usize {
    3
}

fn default_timeout() -> u64 {
    30
}

impl Default for SearchConfig {
    fn default() -> Self {
        Self {
            default_limit: default_search_limit(),
        }
    }
}

impl Default for GraphConfig {
    fn default() -> Self {
        Self {
            max_depth: default_max_depth(),
        }
    }
}

impl Config {
    pub fn from_file(path: &PathBuf) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn database_path(&self) -> PathBuf {
        self.database_path.clone().unwrap_or_else(|| {
            let mut path = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
            path.push("obsidian-cli");
            path.push("index.db");
            path
        })
    }

    pub fn config_dir(&self) -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("obsidian-cli")
    }

    pub fn log_dir(&self) -> PathBuf {
        self.log_path.clone().unwrap_or_else(|| {
            self.config_dir().join("logs")
        })
    }
}
