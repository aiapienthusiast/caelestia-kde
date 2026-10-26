//! Plugins Subsystem
//! 100% parity with `shell/modules/plugins/` and specification in `shell/modules/plugins/README.md`.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PluginSettingType {
    Toggle,
    Text,
    Number,
    Enum,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PluginSettingDecl {
    pub id: String,
    pub label: String,
    #[serde(rename = "type")]
    pub setting_type: PluginSettingType,
    #[serde(default)]
    pub default: Option<serde_json::Value>,
    #[serde(default)]
    pub options: Option<Vec<String>>,
    #[serde(default)]
    pub min: Option<f64>,
    #[serde(default)]
    pub max: Option<f64>,
    #[serde(default)]
    pub step: Option<f64>,
    #[serde(default)]
    pub placeholder: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BarWidgetManifest {
    pub id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default = "default_widget_icon")]
    pub icon: String,
    #[serde(default = "default_widget_zone")]
    pub default_zone: String, // "left", "middle", "right"
    pub component: String,
    #[serde(default)]
    pub popout: Option<String>,
    #[serde(default = "default_true")]
    pub available: bool,
    #[serde(default)]
    pub unavailable_text: Option<String>,
}

fn default_widget_icon() -> String {
    "widgets".to_string()
}

fn default_widget_zone() -> String {
    "right".to_string()
}

fn default_true() -> bool {
    true
}

fn default_z() -> f64 {
    2.5
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackgroundWidgetManifest {
    pub id: String,
    #[serde(default)]
    pub name: Option<String>,
    pub component: String,
    #[serde(default = "default_z")]
    pub z: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginManifest {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub description: String,
    #[serde(rename = "type", default = "default_plugin_type")]
    pub plugin_type: String, // "quickshell", "native", "wasm"
    #[serde(default)]
    pub ui: Option<String>,
    #[serde(default)]
    pub bar_widgets: Vec<BarWidgetManifest>,
    #[serde(default)]
    pub background_widgets: Vec<BackgroundWidgetManifest>,
    #[serde(default)]
    pub settings: Vec<PluginSettingDecl>,
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(skip)]
    pub root_dir: Option<PathBuf>,
}

fn default_plugin_type() -> String {
    "quickshell".to_string()
}

impl Default for PluginManifest {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            version: String::new(),
            author: String::new(),
            description: String::new(),
            plugin_type: default_plugin_type(),
            ui: None,
            bar_widgets: Vec::new(),
            background_widgets: Vec::new(),
            settings: Vec::new(),
            enabled: true,
            root_dir: None,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct PluginManager {
    pub installed_plugins: Vec<PluginManifest>,
    pub plugin_settings: HashMap<String, HashMap<String, serde_json::Value>>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            installed_plugins: Vec::new(),
            plugin_settings: HashMap::new(),
        }
    }

    /// Registers or updates a plugin in the active registry
    pub fn register_plugin(&mut self, manifest: PluginManifest) {
        if let Some(existing) = self.installed_plugins.iter_mut().find(|p| p.id == manifest.id) {
            *existing = manifest;
        } else {
            self.installed_plugins.push(manifest);
        }
    }

    /// Toggles the enabled state of a plugin by ID
    pub fn set_enabled(&mut self, id: &str, enabled: bool) -> bool {
        if let Some(plugin) = self.installed_plugins.iter_mut().find(|p| p.id == id) {
            plugin.enabled = enabled;
            true
        } else {
            false
        }
    }

    /// Returns active bar widgets for a specific zone ("left", "middle", "right") from enabled plugins
    pub fn active_bar_widgets(&self, zone: &str) -> Vec<(&PluginManifest, &BarWidgetManifest)> {
        let mut widgets = Vec::new();
        for plugin in &self.installed_plugins {
            if plugin.enabled {
                for widget in &plugin.bar_widgets {
                    if widget.available && widget.default_zone.eq_ignore_ascii_case(zone) {
                        widgets.push((plugin, widget));
                    }
                }
            }
        }
        widgets
    }

    /// Returns active background widgets from enabled plugins
    pub fn active_background_widgets(&self) -> Vec<(&PluginManifest, &BackgroundWidgetManifest)> {
        let mut widgets = Vec::new();
        for plugin in &self.installed_plugins {
            if plugin.enabled {
                for widget in &plugin.background_widgets {
                    widgets.push((plugin, widget));
                }
            }
        }
        widgets.sort_by(|a, b| a.1.z.partial_cmp(&b.1.z).unwrap_or(std::cmp::Ordering::Equal));
        widgets
    }

    /// Reads a setting value for a plugin (falling back to declared manifest default)
    pub fn get_setting(&self, plugin_id: &str, setting_id: &str) -> Option<serde_json::Value> {
        if let Some(stored) = self.plugin_settings.get(plugin_id).and_then(|m| m.get(setting_id)) {
            return Some(stored.clone());
        }

        // Fall back to manifest default
        self.installed_plugins
            .iter()
            .find(|p| p.id == plugin_id)
            .and_then(|p| p.settings.iter().find(|s| s.id == setting_id))
            .and_then(|s| s.default.clone())
    }

    /// Sets a persistent setting value for a plugin
    pub fn set_setting(&mut self, plugin_id: &str, setting_id: &str, value: serde_json::Value) {
        self.plugin_settings
            .entry(plugin_id.to_string())
            .or_default()
            .insert(setting_id.to_string(), value);
    }

    /// Scans a directory for plugins containing `metadata.json`
    pub fn scan_directory(&mut self, dir_path: &Path) -> std::io::Result<usize> {
        if !dir_path.exists() || !dir_path.is_dir() {
            return Ok(0);
        }

        let mut loaded = 0;
        for entry in fs::read_dir(dir_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let metadata_path = path.join("metadata.json");
                if metadata_path.exists() {
                    if let Ok(content) = fs::read_to_string(&metadata_path) {
                        if let Ok(mut manifest) = serde_json::from_str::<PluginManifest>(&content) {
                            manifest.root_dir = Some(path);
                            self.register_plugin(manifest);
                            loaded += 1;
                        }
                    }
                }
            }
        }
        Ok(loaded)
    }

    /// Discovers plugins from standard user and bundled system directories
    pub fn discover_all(&mut self) -> usize {
        let mut total = 0;
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        
        // 1. User plugins in ~/.config/caelestia/plugins/
        let user_plugins = PathBuf::from(&home).join(".config/caelestia/plugins");
        if let Ok(count) = self.scan_directory(&user_plugins) {
            total += count;
        }

        // 2. Bundled plugins in ~/.config/quickshell/caelestia/modules/plugins/
        let bundled_plugins = PathBuf::from(&home).join(".config/quickshell/caelestia/modules/plugins");
        if let Ok(count) = self.scan_directory(&bundled_plugins) {
            total += count;
        }

        total
    }
}
