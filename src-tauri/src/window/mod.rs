#[cfg(test)]
mod tests;

use std::collections::HashSet;

pub struct WindowManager {
    active_instances: HashSet<String>,
}

impl WindowManager {
    pub fn new() -> Self {
        Self {
            active_instances: HashSet::new(),
        }
    }

    pub fn register_instance(&mut self, _widget_type: &str) -> Result<String, String> {
        // Red phase stub: intentionally not implemented
        Err("not implemented: register instance".to_string())
    }

    pub fn close_instance(&mut self, _instance_id: &str) -> Result<(), String> {
        // Red phase stub: intentionally not implemented
        Err("not implemented: close instance".to_string())
    }

    pub fn is_open(&self, _instance_id: &str) -> bool {
        false
    }

    pub fn open_instances(&self) -> Vec<String> {
        Vec::new()
    }
}
