//! User configuration, loaded from `~/.config/tdo/config.toml`.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub local: LocalConfig,
    pub server: ServerConfig,
    pub daemon: DaemonConfig,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct LocalConfig {
    /// Name of the default list for quick-add (created if it doesn't exist).
    pub default_list: String,
    /// Minimum title length enforced for the positional quick-add shortcut.
    pub min_task_length: usize,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ServerConfig {
    pub url: Option<String>,
    pub api_key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct DaemonConfig {
    pub tick_seconds: u64,
    pub auto_sync_minutes: u64,
    pub notify_due: bool,
    pub notify_sync_errors: bool,
}

impl Default for LocalConfig {
    fn default() -> Self {
        Self {
            default_list: "inbox".to_string(),
            min_task_length: 5,
        }
    }
}

impl Default for DaemonConfig {
    fn default() -> Self {
        Self {
            tick_seconds: 60,
            auto_sync_minutes: 15,
            notify_due: true,
            notify_sync_errors: true,
        }
    }
}

/// Returns the path to the config file, creating parent directories if needed.
pub fn config_path() -> Result<PathBuf> {
    let dir = dirs::config_dir()
        .context("could not determine config directory")?
        .join("tdo");
    std::fs::create_dir_all(&dir)
        .with_context(|| format!("could not create config directory {}", dir.display()))?;
    Ok(dir.join("config.toml"))
}

/// Load configuration from disk, returning defaults if the file doesn't exist yet.
pub fn load() -> Result<Config> {
    let path = config_path()?;
    if !path.exists() {
        return Ok(Config::default());
    }
    let contents = std::fs::read_to_string(&path)
        .with_context(|| format!("could not read config file {}", path.display()))?;
    toml::from_str(&contents).context("could not parse config file")
}
