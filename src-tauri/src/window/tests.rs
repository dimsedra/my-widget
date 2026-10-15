//! Invariant: WindowManager must track all active OS window instances,
//! generate unique deterministic instance identifiers, and manage their lifecycle.

use super::*;

#[test]
fn test_window_manager_instance_tracking() {
    let mut manager = WindowManager::new();

    assert_eq!(manager.open_instances().len(), 0);

    // Register/open a widget instance
    let instance_id = manager
        .register_instance("todo")
        .expect("registering valid widget type should succeed");

    assert!(instance_id.starts_with("todo_"));
    assert!(manager.is_open(&instance_id));
    assert_eq!(manager.open_instances().len(), 1);

    // Register second widget
    let countdown_id = manager
        .register_instance("countdown")
        .expect("registering countdown should succeed");

    assert!(countdown_id.starts_with("countdown_"));
    assert_ne!(instance_id, countdown_id);
    assert_eq!(manager.open_instances().len(), 2);

    // Close first widget
    manager
        .close_instance(&instance_id)
        .expect("closing open instance should succeed");

    assert!(!manager.is_open(&instance_id));
    assert_eq!(manager.open_instances().len(), 1);
    assert!(manager.is_open(&countdown_id));
}

#[test]
fn test_native_window_spawning() {
    let manager = WindowManager::new();

    let todo_params = manager
        .get_launch_params("todo", "todo_test_1")
        .expect("should return launch params for todo");
    assert_eq!(todo_params.label, "todo_test_1");
    assert_eq!(todo_params.widget_type, "todo");
    assert_eq!(todo_params.width, 320.0);
    assert_eq!(todo_params.height, 450.0);
    assert!(!todo_params.decorations, "window must be frameless");
    assert!(todo_params.transparent, "window must be transparent");
    assert!(todo_params.skip_taskbar, "window must skip taskbar");

    let countdown_params = manager
        .get_launch_params("countdown", "countdown_test_1")
        .expect("should return launch params for countdown");
    assert_eq!(countdown_params.label, "countdown_test_1");
    assert_eq!(countdown_params.width, 280.0);
    assert_eq!(countdown_params.height, 200.0);
    assert!(!countdown_params.decorations);
    assert!(countdown_params.transparent);
    assert!(countdown_params.skip_taskbar);

    let err_params = manager.get_launch_params("unsupported_type", "invalid_1");
    assert!(err_params.is_err());
}

