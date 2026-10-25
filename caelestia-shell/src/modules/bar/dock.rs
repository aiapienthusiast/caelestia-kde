//! Bar Dock / Active Window Component
//! Parity with `shell/modules/bar/components/Dock.qml`.

use crate::services::kwin::WindowItem;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DockSnapshot {
    pub title: String,
    pub app_id: String,
    pub has_window: bool,
}

pub struct BarDockModel;

impl BarDockModel {
    pub fn from_active_window(window: Option<&WindowItem>) -> DockSnapshot {
        if let Some(win) = window {
            let title = if win.title.is_empty() {
                win.app_id.clone()
            } else {
                win.title.clone()
            };

            DockSnapshot {
                title,
                app_id: win.app_id.clone(),
                has_window: true,
            }
        } else {
            DockSnapshot {
                title: String::new(),
                app_id: String::new(),
                has_window: false,
            }
        }
    }
}
