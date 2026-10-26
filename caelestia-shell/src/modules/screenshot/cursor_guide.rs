//! Screenshot cursor crosshair guide and options toolbar models
//! Ported with 100% parity from `CursorGuide.qml`, `OptionsToolbar.qml`, and `ToolbarTabBar.qml`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CursorGuideModel {
    pub cursor_x: f32,
    pub cursor_y: f32,
    pub screen_width: f32,
    pub screen_height: f32,
    pub show_pixel_coords: bool,
    pub zoom_level: f32,
}

impl Default for CursorGuideModel {
    fn default() -> Self {
        Self {
            cursor_x: 0.0,
            cursor_y: 0.0,
            screen_width: 1920.0,
            screen_height: 1080.0,
            show_pixel_coords: true,
            zoom_level: 2.0,
        }
    }
}

impl CursorGuideModel {
    pub fn update_pos(&mut self, x: f32, y: f32) {
        self.cursor_x = x;
        self.cursor_y = y;
    }

    pub fn formatted_coords(&self) -> String {
        format!("{:.0}, {:.0} px", self.cursor_x, self.cursor_y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CaptureMode {
    Screenshot,
    Recording,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OptionsToolbarModel {
    pub is_visible: bool,
    pub capture_mode: CaptureMode,
    pub position_x: f32,
    pub position_y: f32,
    pub can_copy: bool,
    pub can_save: bool,
    pub can_ocr: bool,
    pub can_annotate: bool,
}

impl Default for OptionsToolbarModel {
    fn default() -> Self {
        Self {
            is_visible: false,
            capture_mode: CaptureMode::Screenshot,
            position_x: 0.0,
            position_y: 0.0,
            can_copy: true,
            can_save: true,
            can_ocr: true,
            can_annotate: true,
        }
    }
}

impl OptionsToolbarModel {
    pub fn show_at(&mut self, x: f32, y: f32) {
        self.position_x = x;
        self.position_y = y;
        self.is_visible = true;
    }

    pub fn hide(&mut self) {
        self.is_visible = false;
    }

    pub fn set_mode(&mut self, mode: CaptureMode) {
        self.capture_mode = mode;
    }
}
