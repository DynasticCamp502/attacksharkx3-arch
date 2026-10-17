//! Persistent application configuration and state storage.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::macros::MacroItem;
use crate::profiles::ProfileManager;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub profiles: ProfileManager,
    pub macros: Vec<MacroItem>,
    pub auto_connect: bool,
    pub dark_theme: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            profiles: ProfileManager::default(),
            macros: vec![MacroItem::default()],
            auto_connect: true,
            dark_theme: true,
        }
    }
}

impl AppConfig {
    pub fn config_path() -> PathBuf {
        let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("attack-shark-x3");
        path.push("config.toml");
        path
    }

    pub fn load() -> Self {
        let path = Self::config_path();
        if path.exists() {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(mut cfg) = toml::from_str::<AppConfig>(&content) {
                    if cfg.profiles.profiles.is_empty() {
                        cfg.profiles = ProfileManager::default();
                    }
                    return cfg;
                }
            }
        }
        Self::default()
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(self)?;
        fs::write(&path, content)?;
        Ok(())
    }
}
