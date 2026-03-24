use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_db_path")]
    pub db_path: PathBuf,

    #[serde(default)]
    pub llm: LlmConfig,

    #[serde(default)]
    pub web: WebConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    #[serde(default = "default_provider")]
    pub provider: String,

    #[serde(default)]
    pub api_key: Option<String>,

    #[serde(default = "default_model")]
    pub model: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebConfig {
    #[serde(default = "default_host")]
    pub host: String,

    #[serde(default = "default_port")]
    pub port: u16,
}

fn default_db_path() -> PathBuf {
    orchid_home().join("orchid.db")
}

fn default_provider() -> String {
    "anthropic".to_string()
}

fn default_model() -> String {
    "claude-sonnet-4-20250514".to_string()
}

fn default_host() -> String {
    "127.0.0.1".to_string()
}

fn default_port() -> u16 {
    3100
}

impl Default for LlmConfig {
    fn default() -> Self {
        Self {
            provider: default_provider(),
            api_key: None,
            model: default_model(),
        }
    }
}

impl Default for WebConfig {
    fn default() -> Self {
        Self {
            host: default_host(),
            port: default_port(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            db_path: default_db_path(),
            llm: LlmConfig::default(),
            web: WebConfig::default(),
        }
    }
}

/// Returns ~/.orchid/, creating it if necessary.
pub fn orchid_home() -> PathBuf {
    let home = dirs::home_dir().expect("could not determine home directory");
    home.join(".orchid")
}

impl Config {
    /// Load config from ~/.orchid/config.toml, falling back to defaults.
    pub fn load() -> Result<Self> {
        let path = orchid_home().join("config.toml");
        if path.exists() {
            let contents =
                std::fs::read_to_string(&path).context("failed to read config.toml")?;
            let config: Config =
                toml::from_str(&contents).context("failed to parse config.toml")?;
            Ok(config)
        } else {
            Ok(Config::default())
        }
    }
}
