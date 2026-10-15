#[cfg(test)]
mod tests;

use std::collections::HashSet;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, PartialEq)]
pub struct WindowLaunchParams {
    pub label: String,
    pub widget_type: String,
    pub title: String,
    pub width: f64,
    pub height: f64,
    pub decorations: bool,
    pub transparent: bool,
    pub skip_taskbar: bool,
}

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
            Err(format!(
                "window instance '{}' not found or already closed",
                instance_id
            ))
        }
    }

    pub fn is_open(&self, instance_id: &str) -> bool {
        self.active_instances.contains(instance_id)
    }

    pub fn open_instances(&self) -> Vec<String> {
        self.active_instances.iter().cloned().collect()
    }

    pub fn get_launch_params(
        &self,
        widget_type: &str,
        instance_id: &str,
    ) -> Result<WindowLaunchParams, String> {
        if widget_type != "todo" && widget_type != "countdown" {
            return Err(format!("unsupported widget type '{}'", widget_type));
        }

        let (width, height, title) = match widget_type {
            "todo" => (320.0, 450.0, "To-Do Widget"),
            "countdown" => (280.0, 200.0, "Countdown Widget"),
            _ => (300.0, 300.0, "Widget"),
        };

        Ok(WindowLaunchParams {
            label: instance_id.to_string(),
            widget_type: widget_type.to_string(),
            title: title.to_string(),
            width,
            height,
            decorations: false,
            transparent: true,
            skip_taskbar: true,
        })
    }

    pub fn create_window<R: tauri::Runtime>(
        &mut self,
        app: &tauri::AppHandle<R>,
        widget_type: &str,
    ) -> Result<String, String> {
        let instance_id = self.register_instance(widget_type)?;
        let params = self.get_launch_params(widget_type, &instance_id)?;

        let builder = tauri::WebviewWindowBuilder::new(
            app,
            &params.label,
            tauri::WebviewUrl::App("index.html".into()),
        )
        .title(&params.title)
        .inner_size(params.width, params.height)
        .decorations(params.decorations)
        .transparent(params.transparent)
        .skip_taskbar(params.skip_taskbar);

        #[cfg(target_os = "windows")]
        let builder = builder.shadow(false);

        let window = builder
            .build()
            .map_err(|e| format!("failed to build native window: {}", e))?;

        #[cfg(target_os = "windows")]
        {
            if let Ok(hwnd) = window.hwnd() {
                let _ = crate::platform::windows::pin_window_to_desktop(
                    windows::Win32::Foundation::HWND(hwnd.0 as *mut _),
                );
            }
        }

        Ok(instance_id)
    }
}
