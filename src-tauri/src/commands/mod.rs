#[cfg(test)]
mod tests;

use crate::config::models::AppConfig;
use crate::config::ConfigStore;
use std::sync::Mutex;

pub fn load_window_config_logic(store: &Mutex<ConfigStore>) -> Result<AppConfig, String> {
    let mut guard = store
        .lock()
        .map_err(|e| format!("mutex lock poisoned: {}", e))?;
    guard.load()
}

pub fn save_window_position_logic(
    store: &Mutex<ConfigStore>,
    instance_id: &str,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
) -> Result<(), String> {
    if width < 100 || width > 4000 || height < 100 || height > 4000 {
        return Err(format!(
            "invalid window dimensions: width ({}) and height ({}) must be between 100 and 4000",
            width, height
        ));
    }
    let mut guard = store
        .lock()
        .map_err(|e| format!("mutex lock poisoned: {}", e))?;
    guard.save_position(instance_id, x, y, width, height)
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

pub fn open_widget_logic(
    window_mgr: &Mutex<crate::window::WindowManager>,
    widget_type: &str,
) -> Result<String, String> {
    if widget_type != "todo" && widget_type != "countdown" {
        return Err(format!("unsupported widget type '{}'", widget_type));
    }
    let mut guard = window_mgr
        .lock()
        .map_err(|e| format!("mutex lock poisoned: {}", e))?;
    guard.register_instance(widget_type)
}

pub fn close_widget_logic(
    window_mgr: &Mutex<crate::window::WindowManager>,
    instance_id: &str,
) -> Result<(), String> {
    let mut guard = window_mgr
        .lock()
        .map_err(|e| format!("mutex lock poisoned: {}", e))?;
    guard.close_instance(instance_id)
}

#[tauri::command]
pub fn cmd_open_widget(
    widget_type: String,
    window_mgr: tauri::State<'_, Mutex<crate::window::WindowManager>>,
) -> Result<String, String> {
    open_widget_logic(&window_mgr, &widget_type)
}

#[tauri::command]
pub fn cmd_close_widget(
    instance_id: String,
    window_mgr: tauri::State<'_, Mutex<crate::window::WindowManager>>,
) -> Result<(), String> {
    close_widget_logic(&window_mgr, &instance_id)
}

