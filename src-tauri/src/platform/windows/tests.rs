//! Invariant: Windows desktop pinning must configure extended window styles
//! with WS_EX_TOOLWINDOW (0x80) to prevent taskbar/Alt+Tab display,
//! and specify HWND_BOTTOM with SWP_NOSIZE | SWP_NOMOVE | SWP_NOACTIVATE.

use super::*;
use windows::Win32::UI::WindowsAndMessaging::{
    HWND_BOTTOM, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE, WS_EX_TOOLWINDOW,
};

#[test]
fn test_desktop_pinning_flags() {
    let params = get_desktop_pinning_params();

    // Verify WS_EX_TOOLWINDOW flag is specified
    assert_eq!(
        params.ex_style_flags & WS_EX_TOOLWINDOW.0,
        WS_EX_TOOLWINDOW.0,
        "WS_EX_TOOLWINDOW must be present in extended styles"
    );

    // Verify HWND_BOTTOM target
    assert_eq!(
        params.hwnd_insert_after,
        HWND_BOTTOM.0 as isize,
        "Window must be placed at HWND_BOTTOM"
    );

    // Verify SetWindowPos flags
    let expected_swp = (SWP_NOSIZE | SWP_NOMOVE | SWP_NOACTIVATE).0;
    assert_eq!(
        params.set_window_pos_flags & expected_swp,
        expected_swp,
        "SetWindowPos flags must include SWP_NOSIZE | SWP_NOMOVE | SWP_NOACTIVATE"
    );
}
