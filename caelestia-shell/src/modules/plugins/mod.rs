//! Plugins Module
//! Parity with `shell/modules/plugins/`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PluginManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, Default)]
pub struct PluginManager {
    pub installed_plugins: Vec<PluginManifest>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            installed_plugins: Vec::new(),
        }
    }

    pub fn register_plugin(&mut self, manifest: PluginManifest) {
        if let Some(existing) = self.installed_plugins.iter_mut().find(|p| p.id == manifest.id) {
            *existing = manifest;
        } else {
            self.installed_plugins.push(manifest);
        }
    }

    pub fn set_enabled(&mut self, id: &str, enabled: bool) -> bool {
        if let Some(plugin) = self.installed_plugins.iter_mut().find(|p| p.id == id) {
            plugin.enabled = enabled;
            true
        } else {
            false
        }
    }
}
