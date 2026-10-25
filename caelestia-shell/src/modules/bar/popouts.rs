//! Bar Popouts Subsystem
//! Complete parity with `shell/modules/bar/popouts/`.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PopoutType {
    Audio,
    Bluetooth,
    Network,
    Calendar,
    Battery,
    Media,
    Power,
    NightLight,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct BarPopoutsState {
    pub current: Option<PopoutType>,
    pub current_center: f32,
    pub is_open: bool,
}

impl BarPopoutsState {
    pub fn open(&mut self, popout: PopoutType, center: f32) {
        self.current = Some(popout);
        self.current_center = center;
        self.is_open = true;
    }

    pub fn close(&mut self) {
        self.current = None;
        self.is_open = false;
    }

    pub fn toggle(&mut self, popout: PopoutType, center: f32) {
        if self.is_open && self.current == Some(popout) {
            self.close();
        } else {
            self.open(popout, center);
        }
    }
}
