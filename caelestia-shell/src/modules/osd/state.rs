//! OSD State Subsystem
//! Parity with `shell/modules/osd/Content.qml` and `shell/modules/osd/Wrapper.qml`.

use super::config::OsdConfig;
use serde::{Deserialize, Serialize};
use std::time::Instant;

/// Types of sliders displayed within the OSD
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum OsdSliderType {
    Volume,
    Microphone,
    Brightness,
}

/// Reactive state holding live values and visibility timing
#[derive(Clone, Debug)]
pub struct OsdState {
    pub volume: f32,
    pub muted: bool,
    pub source_volume: f32,
    pub source_muted: bool,
    pub brightness: f32,

    pub is_open: bool,
    pub hovered: bool,
    pub offset_scale: f32,
    pub last_shown_at: Option<Instant>,
    pub sidebar_or_session_visible: bool,
}

impl Default for OsdState {
    fn default() -> Self {
        Self {
            volume: 0.5,
            muted: false,
            source_volume: 0.5,
            source_muted: false,
            brightness: 0.7,
            is_open: false,
            hovered: false,
            offset_scale: 1.0,
            last_shown_at: None,
            sidebar_or_session_visible: false,
        }
    }
}

impl OsdState {
    /// Return appropriate volume icon based on volume level and mute state
    pub fn volume_icon(volume: f32, muted: bool) -> &'static str {
        if muted {
            "volume_off"
        } else if volume <= 0.001 {
            "volume_mute"
        } else if volume < 0.5 {
            "volume_down"
        } else {
            "volume_up"
        }
    }

    /// Return microphone icon based on mute state
    pub fn mic_icon(muted: bool) -> &'static str {
        if muted {
            "mic_off"
        } else {
            "mic"
        }
    }

    /// Return Material Symbols brightness icon from brightness_1 to brightness_7
    pub fn brightness_icon(brightness: f32) -> String {
        let index = ((brightness * 6.0).round() as i32 + 1).clamp(1, 7);
        format!("brightness_{index}")
    }

    /// Check if the OSD should be active based on visibility and conflicting drawer states
    pub fn should_be_active(&self, config: &OsdConfig, utilities_open: bool, overview_open: bool) -> bool {
        self.is_open && config.enabled && !utilities_open && !overview_open
    }

    /// Check if auto-hide timer has expired
    pub fn is_expired(&self, hide_delay_ms: u64) -> bool {
        if self.hovered {
            return false;
        }
        if let Some(shown) = self.last_shown_at {
            shown.elapsed().as_millis() >= hide_delay_ms as u128
        } else {
            true
        }
    }

    /// Calculate edge margin offset matching Wrapper.qml
    pub fn edge_offset(&self, implicit_width: f32) -> f32 {
        let sidebar_offset = if self.sidebar_or_session_visible { 12.0 } else { 0.0 };
        (-implicit_width - 5.0 - sidebar_offset) * self.offset_scale
    }
}
