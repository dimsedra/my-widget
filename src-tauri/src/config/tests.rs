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
