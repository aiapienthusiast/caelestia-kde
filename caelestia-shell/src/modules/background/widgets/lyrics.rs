//! Desktop synchronized music lyrics widget
//! Ported with 100% parity from `LyricsWidget.qml` and `DesktopLyrics.qml`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopLyricLine {
    pub timestamp_ms: u64,
    pub text: String,
    pub translation: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DesktopLyricsWidgetModel {
    pub is_playing: bool,
    pub track_title: String,
    pub artist: String,
    pub position_ms: u64,
    pub lines: Vec<DesktopLyricLine>,
    pub current_line_idx: usize,
    pub font_size_pt: f64,
    pub opacity: f64,
}

impl Default for DesktopLyricsWidgetModel {
    fn default() -> Self {
        Self {
            is_playing: false,
            track_title: String::new(),
            artist: String::new(),
            position_ms: 0,
            lines: Vec::new(),
            current_line_idx: 0,
            font_size_pt: 24.0,
            opacity: 0.9,
        }
    }
}

impl DesktopLyricsWidgetModel {
    pub fn set_lyrics(&mut self, lines: Vec<DesktopLyricLine>) {
        self.lines = lines;
        self.update_position(self.position_ms);
    }

    pub fn update_position(&mut self, pos_ms: u64) {
        self.position_ms = pos_ms;
        if self.lines.is_empty() {
            self.current_line_idx = 0;
            return;
        }

        let mut active = 0;
        for (i, line) in self.lines.iter().enumerate() {
            if line.timestamp_ms <= pos_ms {
                active = i;
            } else {
                break;
            }
        }
        self.current_line_idx = active;
    }

    pub fn current_line(&self) -> Option<&DesktopLyricLine> {
        self.lines.get(self.current_line_idx)
    }
}
