//! Window Info Modal Subsystem
//! Complete parity with `shell/modules/windowinfo/` (`WindowInfo.qml`, `Details.qml`, `Buttons.qml`).

use crate::services::kwin::{KWinService, WindowItem};
use serde::{Deserialize, Serialize};

/// Detailed window inspection snapshot
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowInfoDetails {
    pub address: String,
    pub title: String,
    pub app_id: String,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub workspace_id: String,
    pub is_maximized: bool,
    pub is_minimized: bool,
    pub is_fullscreen: bool,
}

impl WindowInfoDetails {
    pub fn from_window_item(item: &WindowItem) -> Self {
        Self {
            address: item.address.clone(),
            title: item.title.clone(),
            app_id: item.app_id.clone(),
            x: item.x,
            y: item.y,
            width: item.width,
            height: item.height,
            workspace_id: item.desktops.first().cloned().unwrap_or_else(|| "1".to_string()),
            is_maximized: item.is_maximized,
            is_minimized: item.is_minimized,
            is_fullscreen: item.is_fullscreen,
        }
    }
}

/// Action dispatcher for the Window Info panel
pub struct WindowInfoController;

impl WindowInfoController {
    /// Move inspected window to target workspace
    pub fn move_to_workspace(address: &str, target_workspace_id: i32, kwin: &mut KWinService) {
        kwin.set_window_desktop(address, target_workspace_id);
        kwin.set_desktop(target_workspace_id as usize);
    }

    /// Toggle window maximize state
    pub fn toggle_maximize(address: &str, current_maximized: bool, kwin: &mut KWinService) {
        kwin.maximize_window(address, !current_maximized, !current_maximized);
    }

    /// Toggle window minimize state
    pub fn toggle_minimize(address: &str, current_minimized: bool, kwin: &mut KWinService) {
        if current_minimized {
            kwin.focus_window(address);
        } else {
            kwin.minimize_window(address);
        }
    }

    /// Close window
    pub fn close_window(address: &str, kwin: &mut KWinService) {
        kwin.close_window(address);
    }
}
