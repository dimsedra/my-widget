use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WindowInstanceConfig {
    pub widget_type: String,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub visible: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AppConfig {
    pub version: u32,
    pub instances: HashMap<String, WindowInstanceConfig>,
}

impl Default for AppConfig {
    fn default() -> Self {
        let mut instances = HashMap::new();
        instances.insert(
            "todo_default".to_string(),
            WindowInstanceConfig {
                widget_type: "todo".to_string(),
                x: 100,
                y: 150,
                width: 320,
                height: 450,
                visible: true,
            },
        );
        instances.insert(
            "countdown_default".to_string(),
            WindowInstanceConfig {
                widget_type: "countdown".to_string(),
                x: 1400,
                y: 150,
                width: 280,
                height: 200,
                visible: true,
            },
        );

        Self {
            version: 1,
            instances,
        }
    }
}
