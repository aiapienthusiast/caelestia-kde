//! Bar Widgets Registry
//! Complete parity with `shell/modules/bar/BarWidgets.qml`.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BarZone {
    Left,
    Middle,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BarWidgetId {
    Logo,
    Workspaces,
    Greeter,
    Dock,
    Tray,
    Media,
    Clock,
    StatusIcons,
    Performance,
    Power,
    Weather,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BarWidgetEntry {
    pub id: BarWidgetId,
    pub enabled: bool,
    pub zone: BarZone,
}

pub struct BarWidgets;

impl BarWidgets {
    /// Return the standard default widget list matching BarWidgets.qml
    pub fn default_entries() -> Vec<BarWidgetEntry> {
        vec![
            BarWidgetEntry { id: BarWidgetId::Logo, enabled: true, zone: BarZone::Left },
            BarWidgetEntry { id: BarWidgetId::Workspaces, enabled: true, zone: BarZone::Left },
            BarWidgetEntry { id: BarWidgetId::Greeter, enabled: false, zone: BarZone::Left },
            BarWidgetEntry { id: BarWidgetId::Dock, enabled: true, zone: BarZone::Middle },
            BarWidgetEntry { id: BarWidgetId::Media, enabled: true, zone: BarZone::Right },
            BarWidgetEntry { id: BarWidgetId::StatusIcons, enabled: true, zone: BarZone::Right },
            BarWidgetEntry { id: BarWidgetId::Performance, enabled: true, zone: BarZone::Right },
            BarWidgetEntry { id: BarWidgetId::Clock, enabled: true, zone: BarZone::Right },
            BarWidgetEntry { id: BarWidgetId::Power, enabled: true, zone: BarZone::Right },
        ]
    }

    /// Filter enabled entries for a specific zone
    pub fn zone_entries(entries: &[BarWidgetEntry], zone: BarZone) -> Vec<BarWidgetId> {
        entries
            .iter()
            .filter(|e| e.enabled && e.zone == zone)
            .map(|e| e.id)
            .collect()
    }
}
