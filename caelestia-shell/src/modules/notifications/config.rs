//! Notifications Module Configuration
//! Parity with `GlobalConfig.notifs` and `shell/modules/notifications/`.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum FullscreenPolicy {
    On,
    Off,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationConfig {
    pub max_popups: usize,
    pub default_timeout_ms: u64,
    pub clear_threshold: f32, // percentage of width to trigger swipe clear (e.g. 0.5)
    pub open_expanded: bool,
    pub action_on_click: bool,
    pub fullscreen_policy: FullscreenPolicy,
    pub width: f32,
}

impl Default for NotificationConfig {
    fn default() -> Self {
        Self {
            max_popups: 5,
            default_timeout_ms: 5000,
            clear_threshold: 0.4,
            open_expanded: false,
            action_on_click: true,
            fullscreen_policy: FullscreenPolicy::Off,
            width: 360.0,
        }
    }
}
