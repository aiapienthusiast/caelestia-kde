//! Desktop media player widget
//! Ported with 100% parity from `MediaWidget.qml`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DesktopMediaWidgetModel {
    pub player_name: String,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub art_url: Option<String>,
    pub is_playing: bool,
    pub position_ms: u64,
    pub duration_ms: u64,
    pub volume: f64,
    pub can_play_pause: bool,
    pub can_go_next: bool,
    pub can_go_previous: bool,
}

impl Default for DesktopMediaWidgetModel {
    fn default() -> Self {
        Self {
            player_name: String::new(),
            title: "No Media Playing".to_string(),
            artist: "Unknown Artist".to_string(),
            album: String::new(),
            art_url: None,
            is_playing: false,
            position_ms: 0,
            duration_ms: 0,
            volume: 1.0,
            can_play_pause: false,
            can_go_next: false,
            can_go_previous: false,
        }
    }
}

impl DesktopMediaWidgetModel {
    pub fn progress_ratio(&self) -> f64 {
        if self.duration_ms == 0 {
            0.0
        } else {
            (self.position_ms as f64 / self.duration_ms as f64).clamp(0.0, 1.0)
        }
    }

    pub fn formatted_position(&self) -> String {
        let sec = (self.position_ms / 1000) % 60;
        let min = self.position_ms / 60000;
        format!("{min}:{sec:02}")
    }

    pub fn formatted_duration(&self) -> String {
        let sec = (self.duration_ms / 1000) % 60;
        let min = self.duration_ms / 60000;
        format!("{min}:{sec:02}")
    }
}
