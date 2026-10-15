#[cfg(test)]
mod tests;

use crate::config::ConfigStore;
use crate::window::WindowManager;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrayAction {
    AddWidget(String),
    ResetAllPositions,
    Quit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrayMenuItem {
    pub id: String,
    pub title: String,
    pub children: Vec<TrayMenuItem>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrayActionResult {
    WidgetAdded { instance_id: String, widget_type: String },
    PositionsReset,
    QuitRequested,
}

pub fn get_tray_menu_items() -> Vec<TrayMenuItem> {
    vec![
        TrayMenuItem {
            id: "add_widget".to_string(),
            title: "Add Widget".to_string(),
            children: vec![
                TrayMenuItem {
                    id: "add_widget_todo".to_string(),
                    title: "To-Do".to_string(),
                    children: Vec::new(),
                },
                TrayMenuItem {
                    id: "add_widget_countdown".to_string(),
                    title: "Countdown".to_string(),
                    children: Vec::new(),
                },
            ],
        },
        TrayMenuItem {
            id: "reset_all_positions".to_string(),
            title: "Reset All Positions".to_string(),
            children: Vec::new(),
        },
        TrayMenuItem {
            id: "quit".to_string(),
            title: "Quit".to_string(),
            children: Vec::new(),
        },
    ]
}

pub fn parse_tray_menu_id(id: &str) -> Option<TrayAction> {
    match id {
        "add_widget_todo" => Some(TrayAction::AddWidget("todo".to_string())),
        "add_widget_countdown" => Some(TrayAction::AddWidget("countdown".to_string())),
        "reset_all_positions" => Some(TrayAction::ResetAllPositions),
        "quit" => Some(TrayAction::Quit),
        _ => None,
    }
}

pub fn handle_tray_action(
    action: TrayAction,
    window_manager: &mut WindowManager,
    config_store: &mut ConfigStore,
) -> Result<TrayActionResult, String> {
    match action {
        TrayAction::AddWidget(widget_type) => {
            if widget_type != "todo" && widget_type != "countdown" {
                return Err(format!("unsupported widget type: {}", widget_type));
            }

            let instance_id = window_manager.register_instance(&widget_type)?;
            let (x, y, width, height) = match widget_type.as_str() {
                "todo" => (100, 150, 320, 450),
                "countdown" => (1400, 150, 280, 200),
                _ => (100, 100, 300, 300),
            };

            config_store.save_position(&instance_id, x, y, width, height)?;

            Ok(TrayActionResult::WidgetAdded {
                instance_id,
                widget_type,
            })
        }
        TrayAction::ResetAllPositions => {
            let config = config_store.load()?;
            for (instance_id, instance) in config.instances {
                let (x, y, width, height) = match instance.widget_type.as_str() {
                    "todo" => (100, 150, 320, 450),
                    "countdown" => (1400, 150, 280, 200),
                    _ => (100, 100, instance.width, instance.height),
                };
                config_store.save_position(&instance_id, x, y, width, height)?;
            }
            Ok(TrayActionResult::PositionsReset)
        }
        TrayAction::Quit => Ok(TrayActionResult::QuitRequested),
    }
}
