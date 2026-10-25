//! What's New Release Notes Entries
//! Parity with `shell/modules/whatsnew/Entries.qml`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WhatsNewEntry {
    pub id: String,
    pub revision: u32,
    pub icon: String,
    pub title: String,
    pub description: String,
    pub settings_page: Option<String>,
}

pub struct WhatsNewModel;

impl WhatsNewModel {
    pub fn default_entries() -> Vec<WhatsNewEntry> {
        vec![
            WhatsNewEntry {
                id: "shimeji-cat".to_string(),
                revision: 62,
                icon: "pets".to_string(),
                title: "Shimeji Cat update".to_string(),
                description: "Multiple Shimeji Cats can now be summoned at once. Also a fix regarding the cat hiding when clicking desktop has been pushed. Update from plugin store to access new features!".to_string(),
                settings_page: Some("plugins".to_string()),
            },
            WhatsNewEntry {
                id: "desktop_multimonitor".to_string(),
                revision: 63,
                icon: "desktop_windows".to_string(),
                title: "Multi-Monitor Desktop Support".to_string(),
                description: "Desktop icons and widgets now fully support multi-monitor setups with screen-aware positioning. Each connected display maintains its own grid layout and arrangement independently.".to_string(),
                settings_page: None,
            },
            WhatsNewEntry {
                id: "desktop_widget_locking".to_string(),
                revision: 64,
                icon: "lock".to_string(),
                title: "Widget Locking & Outlines".to_string(),
                description: "Desktop widgets can now be locked individually or altogether directly from their context menus, preventing accidental dragging or resizing while interacting with them.".to_string(),
                settings_page: None,
            },
        ]
    }
}
