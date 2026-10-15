pub mod models;

#[cfg(test)]
mod tests;

use models::AppConfig;
use std::path::PathBuf;

pub struct ConfigStore {
    pub config_path: PathBuf,
}

impl ConfigStore {
    pub fn new(config_path: PathBuf) -> Self {
        Self { config_path }
    }

    pub fn load(&mut self) -> Result<AppConfig, String> {
        // Red phase stub: intentionally not implemented
        Err("not implemented: default config creation when file is missing".to_string())
    }
}
