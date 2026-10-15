#[cfg(test)]
mod tests;

use crate::config::models::AppConfig;
use crate::config::ConfigStore;
use std::sync::Mutex;

pub fn load_window_config_logic(store: &Mutex<ConfigStore>) -> Result<AppConfig, String> {
    // Red phase stub: intentionally not implemented
    Err("not implemented: load window config command logic".to_string())
}

pub fn save_window_position_logic(
    store: &Mutex<ConfigStore>,
    instance_id: &str,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
) -> Result<(), String> {
    // Red phase stub: intentionally not implemented
    Err("not implemented: save window position command logic".to_string())
}

#[tauri::command]
pub fn cmd_load_window_config(
    store: tauri::State<'_, Mutex<ConfigStore>>,
) -> Result<AppConfig, String> {
    load_window_config_logic(&store)
}

#[tauri::command]
pub fn cmd_save_window_position(
    instance_id: String,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    store: tauri::State<'_, Mutex<ConfigStore>>,
) -> Result<(), String> {
    save_window_position_logic(&store, &instance_id, x, y, width, height)
}
