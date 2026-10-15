use crate::config::ConfigStore;
use crate::window::WindowManager;
use std::sync::Mutex;
use tempfile::tempdir;

#[test]
fn test_builder_registers_state_and_handlers() {
    let dir = tempdir().expect("temp dir");
    let config_path = dir.path().join("config.json");

    let config_store = Mutex::new(ConfigStore::new(config_path));
    let window_manager = Mutex::new(WindowManager::new());

    let _configured_builder = crate::configure_builder(
        tauri::Builder::default(),
        config_store,
        window_manager,
    );
}
