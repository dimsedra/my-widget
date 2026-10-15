//! Invariant: When the configuration file does not exist on disk, the system
//! must safely initialize a default configuration with pre-calibrated widget instances
//! without error or crash.

use super::*;
use tempfile::tempdir;

#[test]
fn test_default_config_when_missing() {
    let temp_dir = tempdir().expect("failed to create temp dir");
    let config_path = temp_dir.path().join("config.json");

    assert!(!config_path.exists());

    let mut store = ConfigStore::new(config_path.clone());
    let config = store.load().expect("should load default config successfully");

    assert_eq!(config.version, 1);
    assert!(config.instances.contains_key("todo_default"));
    assert!(config.instances.contains_key("countdown_default"));

    let todo = config.instances.get("todo_default").unwrap();
    assert_eq!(todo.widget_type, "todo");
    assert_eq!(todo.width, 320);
    assert_eq!(todo.height, 450);

    // Should also write the default config to disk
    assert!(config_path.exists());
}

#[test]
fn test_save_window_position_atomic() {
    let temp_dir = tempdir().expect("failed to create temp dir");
    let config_path = temp_dir.path().join("config.json");

    let mut store = ConfigStore::new(config_path.clone());
    let _ = store.load().expect("should initialize default");

    // Mutate position of todo_default
    store
        .save_position("todo_default", 250, 300, 350, 500)
        .expect("saving position should succeed");

    // Read back config from disk using a fresh store instance to verify persistence
    let mut fresh_store = ConfigStore::new(config_path.clone());
    let reloaded = fresh_store.load().expect("reload should succeed");

    let todo = reloaded.instances.get("todo_default").unwrap();
    assert_eq!(todo.x, 250);
    assert_eq!(todo.y, 300);
    assert_eq!(todo.width, 350);
    assert_eq!(todo.height, 500);

    // Invariant: temporary swap file must not linger on disk
    let temp_file_path = config_path.with_extension("tmp");
    assert!(!temp_file_path.exists(), "temporary swap file must not remain on disk");
}

