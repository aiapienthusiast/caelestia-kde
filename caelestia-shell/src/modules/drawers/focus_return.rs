//! Focus restoration tracking on drawer close
//! Ported with 100% parity from `shell/modules/drawers/FocusReturn.qml`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviousWindowFocus {
    pub window_id: u32,
    pub app_id: String,
    pub title: String,
    pub surface_id: Option<u64>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct FocusReturnCoordinator {
    pub saved_focus: Option<PreviousWindowFocus>,
    pub any_drawer_open: bool,
}

impl FocusReturnCoordinator {
    pub fn on_drawer_opening(&mut self, current_active_window: Option<PreviousWindowFocus>) {
        if !self.any_drawer_open {
            self.saved_focus = current_active_window;
            self.any_drawer_open = true;
        }
    }

    pub fn on_drawer_closing(&mut self) -> Option<PreviousWindowFocus> {
        self.any_drawer_open = false;
        self.saved_focus.take()
    }

    pub fn clear(&mut self) {
        self.saved_focus = None;
        self.any_drawer_open = false;
    }
}
