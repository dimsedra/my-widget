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

#[test]
fn test_corrupt_config_auto_recovery() {
    let temp_dir = tempdir().expect("failed to create temp dir");
    let config_path = temp_dir.path().join("config.json");

    // Intentionally write corrupt/malformed JSON
    std::fs::write(&config_path, "{ malformed json: true, broken...").expect("write corrupt config");

    let mut store = ConfigStore::new(config_path.clone());
    // Should NOT panic or return Err on corrupt config; must fallback to default config
    let config = store
        .load()
        .expect("should safely recover from corrupt config and return default");

    assert_eq!(config.version, 1);
    assert!(config.instances.contains_key("todo_default"));

    // Invariant: The corrupt file must be preserved as a backup: config.json.bak
    let backup_path = temp_dir.path().join("config.json.bak");
    assert!(backup_path.exists(), "backup of corrupt config must be created");

    let backup_content = std::fs::read_to_string(&backup_path).expect("read backup");
    assert_eq!(backup_content, "{ malformed json: true, broken...");

    // The active config file must now contain the valid recovered JSON
    let active_content = std::fs::read_to_string(&config_path).expect("read recovered config");
    assert!(active_content.contains("\"version\": 1"));
}

#[test]
fn test_atomic_overwrite_zero_loss() {
    let temp_dir = tempdir().expect("failed to create temp dir");
    let config_path = temp_dir.path().join("config.json");

    let mut store = ConfigStore::new(config_path.clone());
    let _ = store.load().expect("should initialize default");

    // Invariant 1: Store has cached config in memory
    assert!(store.has_cached_config());

    // Invariant 2: Subsequent save_position updates cache and persists atomically
    store
        .save_position("todo_default", 500, 600, 320, 450)
        .expect("saving position should succeed");

    assert_eq!(
        store
            .get_cached_config()
            .expect("cache should exist")
            .instances
            .get("todo_default")
            .unwrap()
            .x,
        500
    );

    // Invariant 3: Target file exists continuously and temp file does not linger
    assert!(config_path.exists());
    let temp_file_path = config_path.with_extension("tmp");
    assert!(!temp_file_path.exists());
}


