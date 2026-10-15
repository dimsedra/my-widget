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
        instance_id: &str,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
    ) -> Result<(), String> {
        let mut config = self.load()?;
        if let Some(instance) = config.instances.get_mut(instance_id) {
            instance.x = x;
            instance.y = y;
            instance.width = width;
            instance.height = height;
        } else {
            config.instances.insert(
                instance_id.to_string(),
                models::WindowInstanceConfig {
                    widget_type: "unknown".to_string(),
                    x,
                    y,
                    width,
                    height,
                    visible: true,
                },
            );
        }

        let serialized = serde_json::to_string_pretty(&config)
            .map_err(|e| format!("failed to serialize config: {}", e))?;

        let temp_path = self.config_path.with_extension("tmp");
        fs::write(&temp_path, serialized)
            .map_err(|e| format!("failed to write temporary config: {}", e))?;

        if self.config_path.exists() {
            let _ = fs::remove_file(&self.config_path);
        }
        fs::rename(&temp_path, &self.config_path)
            .map_err(|e| format!("failed to replace config file atomically: {}", e))?;

        Ok(())
    }
}
