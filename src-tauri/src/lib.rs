pub mod commands;
pub mod config;
pub mod platform;
pub mod tray;
pub mod window;

#[cfg(test)]
mod runtime_tests;

use std::sync::Mutex;
use crate::commands::{
    cmd_close_widget,
    cmd_load_window_config,
    cmd_open_widget,
    cmd_save_window_position,
};
use crate::config::ConfigStore;
use crate::window::WindowManager;

pub fn configure_builder<R: tauri::Runtime>(
    builder: tauri::Builder<R>,
    config_store: Mutex<ConfigStore>,
    window_manager: Mutex<WindowManager>,
) -> tauri::Builder<R> {
    builder
        .manage(config_store)
        .manage(window_manager)
        .invoke_handler(tauri::generate_handler![
            cmd_load_window_config,
            cmd_save_window_position,
            cmd_open_widget,
            cmd_close_widget,
        ])
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let config_path = std::env::var("APPDATA")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| std::path::PathBuf::from("."))
        .join("my-widget")
        .join("config.json");

    let config_store = Mutex::new(ConfigStore::new(config_path));
    let window_manager = Mutex::new(WindowManager::new());

    let builder = configure_builder(tauri::Builder::default(), config_store, window_manager);

    builder
        .setup(|app| {
            let _ = crate::tray::setup_tray(app.handle());

            #[cfg(target_os = "windows")]
            {
                use tauri::Manager;
                if let Some(window) = app.get_webview_window("main") {
                    if let Ok(hwnd) = window.hwnd() {
                        let _ = crate::platform::windows::pin_window_to_desktop(
                            windows::Win32::Foundation::HWND(hwnd.0 as *mut _),
                        );
                    }
                }
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
