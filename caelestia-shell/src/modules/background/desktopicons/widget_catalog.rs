//! Desktop widget catalog and gallery definitions
//! Ported with 100% parity from `WidgetCatalog.qml` and `WidgetGallery.qml`.

use serde::{Deserialize, Serialize};
use super::layout_engine::Span;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopWidgetDef {
    pub widget_id: String,
    pub name: String,
    pub description: String,
    pub default_span: Span,
    pub icon_name: String,
    pub category: String,
}

pub struct DesktopWidgetCatalog;

impl DesktopWidgetCatalog {
    pub fn all_widgets() -> Vec<DesktopWidgetDef> {
        vec![
            DesktopWidgetDef {
                widget_id: "clock".to_string(),
                name: "Clock".to_string(),
                description: "Digital and analog desktop clock with date".to_string(),
                default_span: Span { w: 2, h: 2 },
                icon_name: "preferences-system-time".to_string(),
                category: "Time".to_string(),
            },
            DesktopWidgetDef {
                widget_id: "calendar".to_string(),
                name: "Calendar".to_string(),
                description: "Monthly agenda and upcoming events".to_string(),
                default_span: Span { w: 3, h: 3 },
                icon_name: "view-calendar".to_string(),
                category: "Productivity".to_string(),
            },
            DesktopWidgetDef {
                widget_id: "media".to_string(),
                name: "Media Player".to_string(),
                description: "Now playing track with controls and album art".to_string(),
                default_span: Span { w: 3, h: 2 },
                icon_name: "multimedia-audio-player".to_string(),
                category: "Media".to_string(),
            },
            DesktopWidgetDef {
                widget_id: "lyrics".to_string(),
                name: "Desktop Lyrics".to_string(),
                description: "Synchronized karaoke style music lyrics".to_string(),
                default_span: Span { w: 4, h: 2 },
                icon_name: "audio-speakers".to_string(),
                category: "Media".to_string(),
            },
            DesktopWidgetDef {
                widget_id: "note".to_string(),
                name: "Sticky Note".to_string(),
                description: "Quick scratchpad note with rich markdown formatting".to_string(),
                default_span: Span { w: 2, h: 2 },
                icon_name: "accessories-notes".to_string(),
                category: "Productivity".to_string(),
            },
            DesktopWidgetDef {
                widget_id: "shapes".to_string(),
                name: "Organic Shapes".to_string(),
                description: "Decorative background soft-body shapes".to_string(),
                default_span: Span { w: 2, h: 2 },
                icon_name: "draw-spiral".to_string(),
                category: "Appearance".to_string(),
            },
            DesktopWidgetDef {
                widget_id: "system".to_string(),
                name: "System Monitor".to_string(),
                description: "CPU, memory, battery, and disk telemetry".to_string(),
                default_span: Span { w: 2, h: 2 },
                icon_name: "utilities-system-monitor".to_string(),
                category: "System".to_string(),
            },
        ]
    }

    pub fn find_widget(widget_id: &str) -> Option<DesktopWidgetDef> {
        Self::all_widgets().into_iter().find(|w| w.widget_id == widget_id)
    }
}
