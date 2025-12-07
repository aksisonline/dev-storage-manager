use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub scan_path: PathBuf,
    pub threshold_days: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            scan_path: dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")),
            threshold_days: 30,
        }
    }
}

impl Config {
    fn config_path() -> PathBuf {
        if let Some(config_dir) = dirs::config_dir() {
            let app_config_dir = config_dir.join("dev-storage-cleaner");
            fs::create_dir_all(&app_config_dir).ok();
            app_config_dir.join("config.json")
        } else {
            PathBuf::from("config.json")
        }
    }

    pub fn load() -> Self {
        let config_path = Self::config_path();

        if config_path.exists() {
            if let Ok(contents) = fs::read_to_string(&config_path) {
                if let Ok(config) = serde_json::from_str(&contents) {
                    return config;
                }
            }
        }

        Self::default()
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = Self::config_path();
        let contents = serde_json::to_string_pretty(self)?;
        fs::write(&config_path, contents)?;
        Ok(())
    }
}
