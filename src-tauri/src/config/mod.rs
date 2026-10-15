pub mod models;

#[cfg(test)]
mod tests;

use models::AppConfig;
use std::fs;
use std::path::PathBuf;

pub struct ConfigStore {
    pub config_path: PathBuf,
}

impl ConfigStore {
    pub fn new(config_path: PathBuf) -> Self {
        Self { config_path }
    }

    pub fn load(&mut self) -> Result<AppConfig, String> {
        if !self.config_path.exists() {
            let default_config = AppConfig::default();
            if let Some(parent) = self.config_path.parent() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("failed to create config directory: {}", e))?;
            }
            let serialized = serde_json::to_string_pretty(&default_config)
                .map_err(|e| format!("failed to serialize default config: {}", e))?;
            fs::write(&self.config_path, serialized)
                .map_err(|e| format!("failed to write default config file: {}", e))?;
            return Ok(default_config);
        }

        let content = fs::read_to_string(&self.config_path)
            .map_err(|e| format!("failed to read config file: {}", e))?;
        let config: AppConfig = serde_json::from_str(&content)
            .map_err(|e| format!("failed to parse config json: {}", e))?;

        Ok(config)
    }

    pub fn save_position(
        &mut self,
        _instance_id: &str,
        _x: i32,
        _y: i32,
        _width: u32,
        _height: u32,
    ) -> Result<(), String> {
        // Red phase stub: intentionally not implemented
        Err("not implemented: atomic save window position".to_string())
    }
}
