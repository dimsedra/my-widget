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
