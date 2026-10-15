pub mod models;

#[cfg(test)]
mod tests;

use models::AppConfig;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

pub struct ConfigStore {
    pub config_path: PathBuf,
    cached_config: Option<AppConfig>,
}

impl ConfigStore {
    pub fn new(config_path: PathBuf) -> Self {
        Self {
            config_path,
            cached_config: None,
        }
    }

    pub fn has_cached_config(&self) -> bool {
        self.cached_config.is_some()
    }

    pub fn get_cached_config(&self) -> Option<&AppConfig> {
        self.cached_config.as_ref()
    }

    pub fn load(&mut self) -> Result<AppConfig, String> {
        if let Some(ref config) = self.cached_config {
            return Ok(config.clone());
        }

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
            self.cached_config = Some(default_config.clone());
            return Ok(default_config);
        }

        let content = fs::read_to_string(&self.config_path)
            .map_err(|e| format!("failed to read config file: {}", e))?;
        match serde_json::from_str::<AppConfig>(&content) {
            Ok(config) => {
                self.cached_config = Some(config.clone());
                Ok(config)
            }
            Err(_) => {
                // NFR-REL-002: Safely recover from corrupt config by backing up old file and creating default
                let backup_path = PathBuf::from(format!("{}.bak", self.config_path.display()));
                let _ = fs::rename(&self.config_path, &backup_path);

                let default_config = AppConfig::default();
                if let Ok(serialized) = serde_json::to_string_pretty(&default_config) {
                    let _ = fs::write(&self.config_path, serialized);
                }
                self.cached_config = Some(default_config.clone());
                Ok(default_config)
            }
        }
    }

    pub fn save_position(
        &mut self,
        instance_id: &str,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
    ) -> Result<(), String> {
        if self.cached_config.is_none() {
            let _ = self.load()?;
        }

        let config = self
            .cached_config
            .as_mut()
            .ok_or_else(|| "config is not loaded in memory".to_string())?;

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

        let serialized = serde_json::to_string_pretty(config)
            .map_err(|e| format!("failed to serialize config: {}", e))?;

        let parent = self
            .config_path
            .parent()
            .unwrap_or_else(|| std::path::Path::new("."));
        fs::create_dir_all(parent)
            .map_err(|e| format!("failed to create config directory: {}", e))?;

        let mut temp_file = tempfile::NamedTempFile::new_in(parent)
            .map_err(|e| format!("failed to create temporary config file: {}", e))?;
        temp_file
            .write_all(serialized.as_bytes())
            .map_err(|e| format!("failed to write temporary config: {}", e))?;
        temp_file
            .persist(&self.config_path)
            .map_err(|e| format!("failed to atomically persist config file: {}", e))?;

        Ok(())
    }
}
