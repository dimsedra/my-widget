//! Invariant: Frontend IPC commands must correctly interface with the managed
//! ConfigStore, loading current configuration and saving updated coordinates.

use super::*;
use crate::config::ConfigStore;
use std::sync::Mutex;
use tempfile::tempdir;

#[test]
fn test_config_ipc_commands() {
    let temp_dir = tempdir().expect("failed to create temp dir");
    let config_path = temp_dir.path().join("config.json");
    let store = Mutex::new(ConfigStore::new(config_path));

    // Test loading config via command logic
    let config = load_window_config_logic(&store).expect("should load config");
    assert_eq!(config.version, 1);
    assert!(config.instances.contains_key("todo_default"));

    // Test saving new window position via command logic
    save_window_position_logic(&store, "todo_default", 500, 600, 400, 500)
        .expect("should save position");

    // Verify state was updated
    let reloaded = load_window_config_logic(&store).expect("should reload config");
    let todo = reloaded.instances.get("todo_default").unwrap();
    assert_eq!(todo.x, 500);
    assert_eq!(todo.y, 600);
    assert_eq!(todo.width, 400);
    assert_eq!(todo.height, 500);
}
