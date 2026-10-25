//! OSD Configuration
//! Parity with `Config.osd` in `shell/modules/osd/`.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OsdConfig {
    pub enabled: bool,
    pub enable_volume: bool,
    pub enable_microphone: bool,
    pub enable_brightness: bool,
    pub hide_delay_ms: u64,
    pub slider_width: f32,
    pub slider_height: f32,
    pub max_volume: f32,
    pub volume_step: f32,
    pub brightness_step: f32,
}

impl Default for OsdConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            enable_volume: true,
            enable_microphone: true,
            enable_brightness: true,
            hide_delay_ms: 1500,
            slider_width: 36.0,
            slider_height: 180.0,
            max_volume: 1.5,
            volume_step: 0.05,
            brightness_step: 0.05,
        }
    }
}
