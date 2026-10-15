#[cfg(test)]
mod tests;

use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{
    GetWindowLongPtrW, SetWindowLongPtrW, SetWindowPos, GWL_EXSTYLE, HWND_BOTTOM,
    SET_WINDOW_POS_FLAGS, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE, WS_EX_TOOLWINDOW,
};

pub struct PinningConfig {
    pub ex_style_flags: u32,
    pub hwnd_insert_after: isize,
    pub set_window_pos_flags: u32,
}

pub fn get_desktop_pinning_params() -> PinningConfig {
    PinningConfig {
        ex_style_flags: WS_EX_TOOLWINDOW.0,
        hwnd_insert_after: HWND_BOTTOM.0 as isize,
        set_window_pos_flags: (SWP_NOSIZE | SWP_NOMOVE | SWP_NOACTIVATE).0,
    }
}

pub fn pin_window_to_desktop(hwnd: HWND) -> Result<(), String> {
    unsafe {
        // 1. Apply extended tool window style (WS_EX_TOOLWINDOW) so it won't appear in Alt+Tab
        let current_ex_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
        let new_ex_style = current_ex_style | (WS_EX_TOOLWINDOW.0 as isize);
        let _ = SetWindowLongPtrW(hwnd, GWL_EXSTYLE, new_ex_style);

        // 2. Set Z-order to HWND_BOTTOM without activation or resize
        let params = get_desktop_pinning_params();
        let res = SetWindowPos(
            hwnd,
            Some(HWND(params.hwnd_insert_after as *mut _)),
            0,
            0,
            0,
            0,
            SET_WINDOW_POS_FLAGS(params.set_window_pos_flags),
        );

        res.map_err(|e| format!("failed to set window position to HWND_BOTTOM: {}", e))
    }
}
