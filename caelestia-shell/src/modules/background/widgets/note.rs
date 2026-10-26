//! Desktop sticky note widget
//! Ported with 100% parity from `NoteWidget.qml`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopNoteWidgetModel {
    pub note_id: String,
    pub title: String,
    pub content: String,
    pub color_tint: String, // hex color or M3 container name
    pub is_pinned: bool,
    pub modified_timestamp: u64,
}

impl Default for DesktopNoteWidgetModel {
    fn default() -> Self {
        Self {
            note_id: "default_note".to_string(),
            title: "Quick Note".to_string(),
            content: String::new(),
            color_tint: "#FBE9E7".to_string(),
            is_pinned: false,
            modified_timestamp: 0,
        }
    }
}

impl DesktopNoteWidgetModel {
    pub fn new(note_id: impl Into<String>, title: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            note_id: note_id.into(),
            title: title.into(),
            content: content.into(),
            color_tint: "#FBE9E7".to_string(),
            is_pinned: false,
            modified_timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
        }
    }

    pub fn update_content(&mut self, new_content: impl Into<String>) {
        self.content = new_content.into();
        self.modified_timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
    }
}
