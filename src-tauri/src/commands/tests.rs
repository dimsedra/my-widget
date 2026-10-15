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

#[test]
fn test_boundary_and_error_handling() {
    let temp_dir = tempdir().expect("failed to create temp dir");
    let config_path = temp_dir.path().join("config.json");
    let store = Mutex::new(ConfigStore::new(config_path));
    let window_mgr = Mutex::new(crate::window::WindowManager::new());

    // Boundary check 1: Reject zero or too-small width/height
    let small_result = save_window_position_logic(&store, "todo_default", 100, 100, 50, 50);
    assert!(small_result.is_err(), "should reject dimension smaller than 100px");
    assert!(small_result.unwrap_err().contains("invalid window dimensions"));

    // Boundary check 2: Reject excessive dimension (> 4000px)
    let huge_result = save_window_position_logic(&store, "todo_default", 100, 100, 5000, 5000);
    assert!(huge_result.is_err(), "should reject dimension greater than 4000px");

    // Error path: Reject closing non-existent instance
    let close_err = close_widget_logic(&window_mgr, "non_existent_instance");
    assert!(close_err.is_err(), "should reject non-existent instance close");
    assert!(close_err.unwrap_err().contains("not found"));

    // Error path: Reject unknown widget type
    let unknown_type = open_widget_logic(&window_mgr, "unknown_widget_type");
    assert!(unknown_type.is_err(), "should reject unsupported widget type");
    assert!(unknown_type.unwrap_err().contains("unsupported widget type"));
}

