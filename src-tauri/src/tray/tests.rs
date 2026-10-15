use super::*;
use tempfile::tempdir;

#[test]
fn test_tray_menu_routing() {
    let dir = tempdir().expect("failed to create temp dir");
    let config_path = dir.path().join("config.json");
    let mut config_store = ConfigStore::new(config_path);
    let mut window_manager = WindowManager::new();

    // 1. Verify menu structure
    let items = get_tray_menu_items();
    assert_eq!(items.len(), 3);
    assert_eq!(items[0].id, "add_widget");
    assert_eq!(items[0].title, "Add Widget");
    assert_eq!(items[0].children.len(), 2);
    assert_eq!(items[0].children[0].id, "add_widget_todo");
    assert_eq!(items[0].children[0].title, "To-Do");
    assert_eq!(items[0].children[1].id, "add_widget_countdown");
    assert_eq!(items[0].children[1].title, "Countdown");

    assert_eq!(items[1].id, "reset_all_positions");
    assert_eq!(items[1].title, "Reset All Positions");

    assert_eq!(items[2].id, "quit");
    assert_eq!(items[2].title, "Quit");

    // 2. Verify menu ID parsing
    assert_eq!(
        parse_tray_menu_id("add_widget_todo"),
        Some(TrayAction::AddWidget("todo".to_string()))
    );
    assert_eq!(
        parse_tray_menu_id("add_widget_countdown"),
        Some(TrayAction::AddWidget("countdown".to_string()))
    );
    assert_eq!(
        parse_tray_menu_id("reset_all_positions"),
        Some(TrayAction::ResetAllPositions)
    );
    assert_eq!(parse_tray_menu_id("quit"), Some(TrayAction::Quit));
    assert_eq!(parse_tray_menu_id("invalid_id"), None);

    // 3. Verify AddWidget routing
    let result_todo = handle_tray_action(
        TrayAction::AddWidget("todo".to_string()),
        &mut window_manager,
        &mut config_store,
    )
    .expect("failed to add todo widget");

    match result_todo {
        TrayActionResult::WidgetAdded { instance_id, widget_type } => {
            assert_eq!(widget_type, "todo");
            assert!(window_manager.is_open(&instance_id));
            let cfg = config_store.load().expect("failed to load config");
            assert!(cfg.instances.contains_key(&instance_id));
        }
        _ => panic!("unexpected action result"),
    }

    let result_countdown = handle_tray_action(
        TrayAction::AddWidget("countdown".to_string()),
        &mut window_manager,
        &mut config_store,
    )
    .expect("failed to add countdown widget");

    match result_countdown {
        TrayActionResult::WidgetAdded { instance_id, widget_type } => {
            assert_eq!(widget_type, "countdown");
            assert!(window_manager.is_open(&instance_id));
            let cfg = config_store.load().expect("failed to load config");
            assert!(cfg.instances.contains_key(&instance_id));
        }
        _ => panic!("unexpected action result"),
    }

    // 4. Verify ResetAllPositions routing
    // Mutate an existing instance position to non-default
    let cfg_before = config_store.load().expect("failed to load config");
    let any_id = cfg_before.instances.keys().next().cloned().unwrap();
    config_store
        .save_position(&any_id, 9999, 8888, 500, 500)
        .expect("failed to save custom position");

    let result_reset = handle_tray_action(
        TrayAction::ResetAllPositions,
        &mut window_manager,
        &mut config_store,
    )
    .expect("failed to reset positions");
    assert_eq!(result_reset, TrayActionResult::PositionsReset);

    let cfg_after = config_store.load().expect("failed to load config");
    let reset_instance = cfg_after.instances.get(&any_id).unwrap();
    assert_ne!(reset_instance.x, 9999);
    assert_ne!(reset_instance.y, 8888);

    // 5. Verify Quit routing
    let result_quit = handle_tray_action(
        TrayAction::Quit,
        &mut window_manager,
        &mut config_store,
    )
    .expect("failed to process quit");
    assert_eq!(result_quit, TrayActionResult::QuitRequested);

    // 6. Verify error on unsupported widget
    let err = handle_tray_action(
        TrayAction::AddWidget("unknown_widget".to_string()),
        &mut window_manager,
        &mut config_store,
    );
    assert!(err.is_err());
}
