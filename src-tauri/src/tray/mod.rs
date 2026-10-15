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
    unimplemented!()
}

pub fn parse_tray_menu_id(_id: &str) -> Option<TrayAction> {
    unimplemented!()
}

pub fn handle_tray_action(
    _action: TrayAction,
    _window_manager: &mut WindowManager,
    _config_store: &mut ConfigStore,
) -> Result<TrayActionResult, String> {
    unimplemented!()
}
