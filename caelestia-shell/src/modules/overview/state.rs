//! Overview State Management and Controller
//! Complete parity with `shell/modules/overview/WindowGrid.qml` and `Content.qml`.

use crate::services::kwin::{KWinService, WindowItem};
use serde::{Deserialize, Serialize};
use tracing::info;

/// Controller state for overview interaction, window selection, and drag-and-drop
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OverviewState {
    pub is_open: bool,
    pub current_workspace_index: usize,
    pub selected_index: Option<usize>,
    pub pre_overview_active_address: Option<String>,
    pub drag_address: Option<String>,
    pub window_info_address: Option<String>,
    pub is_window_info_open: bool,
}

impl Default for OverviewState {
    fn default() -> Self {
        Self {
            is_open: false,
            current_workspace_index: 0,
            selected_index: None,
            pre_overview_active_address: None,
            drag_address: None,
            window_info_address: None,
            is_window_info_open: false,
        }
    }
}

impl OverviewState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Open overview and synchronize to current active workspace & window
    pub fn open(&mut self, kwin: &KWinService) {
        self.is_open = true;
        self.current_workspace_index = kwin.state.active_index as usize;
        self.pre_overview_active_address = kwin.state.active_window.as_ref().map(|w| w.address.clone());

        // Select the previously active window if present in current workspace
        let current_windows = kwin.windows_for_workspace(self.current_workspace_index as i32, false);
        if let Some(ref target) = self.pre_overview_active_address {
            self.selected_index = current_windows.iter().position(|w| w.address == *target);
        } else {
            self.selected_index = if !current_windows.is_empty() { Some(0) } else { None };
        }
    }

    /// Close overview and reset selection
    pub fn close(&mut self) {
        self.is_open = false;
        self.selected_index = None;
        self.drag_address = None;
        self.is_window_info_open = false;
        self.window_info_address = None;
    }

    /// Toggle overview open/close state
    pub fn toggle(&mut self, kwin: &KWinService) {
        if self.is_open {
            self.close();
        } else {
            self.open(kwin);
        }
    }

    /// Cycle selected window card in the active grid
    pub fn cycle_selection(&mut self, backwards: bool, window_count: usize) {
        if window_count == 0 {
            self.selected_index = None;
            return;
        }

        self.selected_index = Some(match self.selected_index {
            None => {
                if backwards {
                    window_count - 1
                } else {
                    0
                }
            }
            Some(idx) => {
                if backwards {
                    if idx == 0 {
                        window_count - 1
                    } else {
                        idx - 1
                    }
                } else if idx + 1 >= window_count {
                    0
                } else {
                    idx + 1
                }
            }
        });
    }

    /// Focus and activate the selected window, then close overview
    pub fn activate_selected(&mut self, windows: &[WindowItem], kwin: &mut KWinService) {
        if let Some(idx) = self.selected_index {
            if let Some(win) = windows.get(idx) {
                let addr = win.address.clone();
                kwin.focus_window(&addr);
                kwin.set_desktop(self.current_workspace_index);
            }
        }
        self.close();
    }

    /// Move a window to another workspace (Drag-and-drop parity)
    pub fn move_window_to_workspace(
        &mut self,
        address: &str,
        target_workspace_id: i32,
        kwin: &mut KWinService,
    ) {
        info!("Overview moving window {} to workspace {}", address, target_workspace_id);
        kwin.set_window_desktop(address, target_workspace_id);
        self.drag_address = None;
    }

    /// Request closing a window from its card
    pub fn close_window(&mut self, address: &str, kwin: &mut KWinService) {
        info!("Overview closing window {}", address);
        kwin.close_window(address);
    }

    /// Switch workspace page inside overview
    pub fn set_workspace(&mut self, index: usize, kwin: &mut KWinService) {
        self.current_workspace_index = index;
        self.selected_index = None;
        kwin.set_desktop(index);
    }

    /// Open window info overlay for a specific window
    pub fn open_window_info(&mut self, address: &str) {
        self.window_info_address = Some(address.to_string());
        self.is_window_info_open = true;
    }

    /// Dismiss window info overlay
    pub fn close_window_info(&mut self) {
        self.is_window_info_open = false;
        self.window_info_address = None;
    }
}
