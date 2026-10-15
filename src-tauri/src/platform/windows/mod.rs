#[cfg(test)]
mod tests;

pub struct PinningConfig {
    pub ex_style_flags: u32,
    pub hwnd_insert_after: isize,
    pub set_window_pos_flags: u32,
}

pub fn get_desktop_pinning_params() -> PinningConfig {
    // Red phase stub: intentionally empty
    PinningConfig {
        ex_style_flags: 0,
        hwnd_insert_after: 0,
        set_window_pos_flags: 0,
    }
}
