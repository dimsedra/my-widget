#[cfg(test)]
mod tests;

use std::collections::HashSet;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct WindowManager {
    active_instances: HashSet<String>,
}

impl Default for WindowManager {
    fn default() -> Self {
        Self::new()
    }
}

impl WindowManager {
    pub fn new() -> Self {
        Self {
            active_instances: HashSet::new(),
        }
    }

    pub fn register_instance(&mut self, widget_type: &str) -> Result<String, String> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| format!("system time error: {}", e))?
            .as_nanos();
        let instance_id = format!("{}_{}", widget_type, timestamp);
        self.active_instances.insert(instance_id.clone());
        Ok(instance_id)
    }

    pub fn close_instance(&mut self, instance_id: &str) -> Result<(), String> {
        if self.active_instances.remove(instance_id) {
            Ok(())
        } else {
            Err(format!("window instance '{}' not found or already closed", instance_id))
        }
    }

    pub fn is_open(&self, instance_id: &str) -> bool {
        self.active_instances.contains(instance_id)
    }

    pub fn open_instances(&self) -> Vec<String> {
        self.active_instances.iter().cloned().collect()
    }
}
