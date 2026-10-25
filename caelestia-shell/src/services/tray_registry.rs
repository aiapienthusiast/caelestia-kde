//! StatusNotifierItem (SNI) Tray Registry Service
//! Complete parity with `shell/services/TrayRegistry.qml` and `shell/services/trayicons.js`.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrayItemCategory {
    ApplicationStatus,
    Communications,
    SystemServices,
    Hardware,
    Other,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrayItemStatus {
    Passive,
    Active,
    NeedsAttention,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrayItem {
    pub id: String,
    pub title: String,
    pub icon_name: String,
    pub icon_theme_path: Option<String>,
    pub category: TrayItemCategory,
    pub status: TrayItemStatus,
    pub service_name: String,
    pub object_path: String,
    pub menu_path: Option<String>,
    pub tooltip: Option<String>,
    pub is_collapsed: bool,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TrayRegistryState {
    pub items: Vec<TrayItem>,
    pub collapsed_count: usize,
    pub pinned_ids: Vec<String>,
    pub hidden_ids: Vec<String>,
}

pub struct TrayRegistryService {
    pub state: TrayRegistryState,
    fallback_icon_map: HashMap<String, String>,
}

impl Default for TrayRegistryService {
    fn default() -> Self {
        Self::new()
    }
}

impl TrayRegistryService {
    pub fn new() -> Self {
        let mut fallback = HashMap::new();
        // Fallback mappings from shell/services/trayicons.js
        fallback.insert("discord".to_string(), "discord".to_string());
        fallback.insert("steam".to_string(), "steam".to_string());
        fallback.insert("telegram-desktop".to_string(), "telegram".to_string());
        fallback.insert("slack".to_string(), "slack".to_string());
        fallback.insert("spotify".to_string(), "spotify".to_string());
        fallback.insert("obs".to_string(), "obs".to_string());
        fallback.insert("keepassxc".to_string(), "keepassxc".to_string());

        Self {
            state: TrayRegistryState::default(),
            fallback_icon_map: fallback,
        }
    }

    /// Register a new SNI tray item.
    pub fn register_item(&mut self, item: TrayItem) {
        if let Some(pos) = self.state.items.iter().position(|i| i.id == item.id) {
            self.state.items[pos] = item;
        } else {
            self.state.items.push(item);
        }
        self.recalculate();
    }

    /// Unregister an SNI item by ID.
    pub fn unregister_item(&mut self, id: &str) {
        self.state.items.retain(|i| i.id != id);
        self.recalculate();
    }

    /// Resolve an icon name using the fallback map if needed.
    pub fn resolve_icon(&self, id: &str, raw_icon: &str) -> String {
        if !raw_icon.is_empty() {
            return raw_icon.to_string();
        }
        if let Some(mapped) = self.fallback_icon_map.get(id) {
            return mapped.clone();
        }
        "application-x-executable".to_string()
    }

    /// Recalculate collapsed counts and visibility states.
    fn recalculate(&mut self) {
        self.state.collapsed_count = self.state.items.iter().filter(|i| i.is_collapsed).count();
    }

    /// Return visible tray items for the bar dock.
    pub fn visible_items(&self) -> Vec<&TrayItem> {
        self.state
            .items
            .iter()
            .filter(|i| !self.state.hidden_ids.contains(&i.id))
            .collect()
    }
}
