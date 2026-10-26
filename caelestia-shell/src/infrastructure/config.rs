//! Configuration manager for Caelestia Shell settings (~/.config/caelestia/shell.json).
//! Provides read/write access and atomic JSON persistence matching GlobalConfig.qml.

#![allow(dead_code)]

use serde_json::{json, Value};
use std::fs;
use std::path::PathBuf;
use tracing::info;

#[derive(Debug, Clone)]
pub struct ShellConfig {
    raw: Value,
}

impl Default for ShellConfig {
    fn default() -> Self {
        Self {
            raw: json!({
                "appearance": {
                    "anim": { "enabled": true },
                    "blur": true,
                    "pitchBlack": false,
                    "transparency": {
                        "base": 0.58,
                        "enabled": true,
                        "layers": 0.50
                    },
                    "font": {
                        "scale": 1.0,
                        "family": "GoogleSansFlex"
                    },
                    "rounding": {
                        "scale": 1.0
                    }
                },
                "border": {
                    "thickness": 0
                }
            }),
        }
    }
}

impl ShellConfig {
    /// Load configuration from a specific path.
    pub fn load_from(path: &std::path::Path) -> Self {
        match fs::read_to_string(path) {
            Ok(content) => match serde_json::from_str::<Value>(&content) {
                Ok(val) => {
                    crate::infrastructure::loader_diagnostics::global_record_loaded(
                        crate::infrastructure::loader_diagnostics::DiagnosticCategory::Config,
                        "shell.json",
                        path.to_str(),
                        "Shell configuration loaded successfully",
                    );
                    Self { raw: val }
                }
                Err(err) => {
                    crate::infrastructure::loader_diagnostics::global_record_fallback(
                        crate::infrastructure::loader_diagnostics::DiagnosticCategory::Config,
                        "shell.json",
                        path.to_str(),
                        "Using default built-in ShellConfig parameters",
                        format!("JSON syntax error at line {}:{}: {}", err.line(), err.column(), err),
                    );
                    Self::default()
                }
            },
            Err(err) => {
                crate::infrastructure::loader_diagnostics::global_record_fallback(
                    crate::infrastructure::loader_diagnostics::DiagnosticCategory::Config,
                    "shell.json",
                    path.to_str(),
                    "Using default built-in ShellConfig parameters",
                    format!("Failed to read file: {}", err),
                );
                Self::default()
            }
        }
    }

    /// Load shell.json from ~/.config/caelestia/shell.json.
    pub fn load() -> Self {
        Self::load_from(&Self::config_path())
    }

    pub fn config_path() -> PathBuf {
        if let Ok(home) = std::env::var("HOME") {
            PathBuf::from(home).join(".config/caelestia/shell.json")
        } else {
            PathBuf::from(".config/caelestia/shell.json")
        }
    }

    /// Atomically persist configuration to a specific path using tempfile rename.
    pub fn save_to(&self, path: &std::path::Path) -> Result<(), String> {
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        let formatted = serde_json::to_string_pretty(&self.raw)
            .map_err(|e| format!("JSON serialization error: {}", e))?;

        let tmp_path = path.with_extension("tmp");
        fs::write(&tmp_path, formatted).map_err(|e| format!("Write error: {}", e))?;
        fs::rename(&tmp_path, path).map_err(|e| format!("Atomic rename error: {}", e))?;

        info!("Caelestia Shell configuration successfully saved to {:?}", path);
        Ok(())
    }

    /// Atomically persist current configuration to ~/.config/caelestia/shell.json.
    pub fn save(&self) -> Result<(), String> {
        let path = Self::config_path();
        self.save_to(&path)
    }

    // --- Appearance Settings ---

    pub fn pitch_black(&self) -> bool {
        self.raw
            .get("appearance")
            .and_then(|a| a.get("pitchBlack"))
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
    }

    pub fn set_pitch_black(&mut self, enabled: bool) {
        self.ensure_object_path(&["appearance"]);
        self.raw["appearance"]["pitchBlack"] = json!(enabled);
    }

    pub fn blur_enabled(&self) -> bool {
        self.raw
            .get("appearance")
            .and_then(|a| a.get("blur"))
            .and_then(|v| v.as_bool())
            .unwrap_or(true)
    }

    pub fn set_blur_enabled(&mut self, enabled: bool) {
        self.ensure_object_path(&["appearance"]);
        self.raw["appearance"]["blur"] = json!(enabled);
    }

    pub fn transparency_enabled(&self) -> bool {
        self.raw
            .get("appearance")
            .and_then(|a| a.get("transparency"))
            .and_then(|t| t.get("enabled"))
            .and_then(|v| v.as_bool())
            .unwrap_or(true)
    }

    pub fn set_transparency_enabled(&mut self, enabled: bool) {
        self.ensure_object_path(&["appearance", "transparency"]);
        self.raw["appearance"]["transparency"]["enabled"] = json!(enabled);
        if !enabled {
            self.set_blur_enabled(false);
        }
    }

    pub fn transparency_base(&self) -> f64 {
        self.raw
            .get("appearance")
            .and_then(|a| a.get("transparency"))
            .and_then(|t| t.get("base"))
            .and_then(|v| v.as_f64())
            .unwrap_or(0.58)
    }

    pub fn set_transparency_base(&mut self, val: f64) {
        self.ensure_object_path(&["appearance", "transparency"]);
        self.raw["appearance"]["transparency"]["base"] = json!((val * 100.0).round() / 100.0);
    }

    pub fn transparency_layers(&self) -> f64 {
        self.raw
            .get("appearance")
            .and_then(|a| a.get("transparency"))
            .and_then(|t| t.get("layers"))
            .and_then(|v| v.as_f64())
            .unwrap_or(0.50)
    }

    pub fn set_transparency_layers(&mut self, val: f64) {
        self.ensure_object_path(&["appearance", "transparency"]);
        self.raw["appearance"]["transparency"]["layers"] = json!((val * 100.0).round() / 100.0);
    }

    pub fn border_thickness(&self) -> f64 {
        self.raw
            .get("border")
            .and_then(|b| b.get("thickness"))
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0)
    }

    pub fn set_border_thickness(&mut self, thickness: f64) {
        self.ensure_object_path(&["border"]);
        self.raw["border"]["thickness"] = json!(thickness.round() as i64);
    }

    pub fn rounding_scale(&self) -> f64 {
        self.raw
            .get("appearance")
            .and_then(|a| a.get("rounding"))
            .and_then(|r| r.get("scale"))
            .and_then(|v| v.as_f64())
            .unwrap_or(1.0)
    }

    pub fn set_rounding_scale(&mut self, scale: f64) {
        self.ensure_object_path(&["appearance", "rounding"]);
        self.raw["appearance"]["rounding"]["scale"] = json!((scale * 100.0).round() / 100.0);
    }

    pub fn font_family(&self) -> String {
        self.raw
            .get("appearance")
            .and_then(|a| a.get("font"))
            .and_then(|f| f.get("body"))
            .and_then(|b| b.get("family"))
            .and_then(|v| v.as_str())
            .unwrap_or("GoogleSansFlex")
            .to_string()
    }

    pub fn set_font_family(&mut self, family: &str) {
        self.ensure_object_path(&["appearance", "font", "body"]);
        self.ensure_object_path(&["appearance", "font", "title"]);
        self.ensure_object_path(&["appearance", "font", "headline"]);
        self.ensure_object_path(&["appearance", "font", "label"]);
        self.raw["appearance"]["font"]["body"]["family"] = json!(family);
        self.raw["appearance"]["font"]["title"]["family"] = json!(family);
        self.raw["appearance"]["font"]["headline"]["family"] = json!(family);
        self.raw["appearance"]["font"]["label"]["family"] = json!(family);
    }

    // Helper to ensure nested JSON object path exists
    fn ensure_object_path(&mut self, path: &[&str]) {
        let mut curr = &mut self.raw;
        for key in path {
            if !curr.is_object() {
                *curr = json!({});
            }
            if curr.get(*key).is_none() || !curr[*key].is_object() {
                curr[*key] = json!({});
            }
            curr = &mut curr[*key];
        }
    }
}
