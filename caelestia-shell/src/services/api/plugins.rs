//! Plugins API Bridge
//! Parity with `shell/services/api/PluginsApi.qml`.

use std::sync::{Arc, Mutex, OnceLock};

/// Discovered Plugin Metadata
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PluginMeta {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: String,
    pub enabled: bool,
}

/// Plugins API exposing discovered and active shell plugins
#[derive(Clone, Default)]
pub struct PluginsApi {
    pub available: Arc<Mutex<Vec<PluginMeta>>>,
}

impl PluginsApi {
    pub fn new() -> Self {
        Self {
            available: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn global() -> &'static Self {
        static INSTANCE: OnceLock<PluginsApi> = OnceLock::new();
        INSTANCE.get_or_init(Self::new)
    }

    /// List all discovered plugins
    pub fn list(&self) -> Vec<PluginMeta> {
        self.available.lock().map_or(Vec::new(), |p| p.clone())
    }

    /// Register or update a plugin entry
    pub fn register(&self, plugin: PluginMeta) {
        if let Ok(mut list) = self.available.lock() {
            if let Some(pos) = list.iter().position(|p| p.id == plugin.id) {
                list[pos] = plugin;
            } else {
                list.push(plugin);
            }
        }
    }

    /// Set plugin enabled status
    pub fn set_enabled(&self, id: &str, enabled: bool) -> bool {
        if let Ok(mut list) = self.available.lock() {
            if let Some(p) = list.iter_mut().find(|p| p.id == id) {
                p.enabled = enabled;
                return true;
            }
        }
        false
    }
}
