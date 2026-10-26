//! Autostart Management Service
//! 100% native Rust parity with `shell/scripts/autostart.py`.
//! Implements XDG Autostart specification: parsing, merging user and system entries,
//! and mutating enabled status (`Hidden=true` / `X-GNOME-Autostart-enabled`).

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AutostartEntry {
    pub id: String,
    pub name: String,
    pub exec: String,
    pub icon: Option<String>,
    pub comment: Option<String>,
    pub enabled: bool,
    pub is_user_entry: bool,
    pub path: PathBuf,
}

#[derive(Debug, Clone, Default)]
pub struct AutostartService {
    pub entries: Vec<AutostartEntry>,
}

impl AutostartService {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// User autostart directory: $XDG_CONFIG_HOME/autostart or ~/.config/autostart
    pub fn user_dir() -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        match std::env::var("XDG_CONFIG_HOME") {
            Ok(val) if !val.is_empty() => PathBuf::from(val).join("autostart"),
            _ => PathBuf::from(home).join(".config/autostart"),
        }
    }

    /// System autostart directories from $XDG_CONFIG_DIRS or /etc/xdg
    pub fn system_dirs() -> Vec<PathBuf> {
        let xdg_dirs = std::env::var("XDG_CONFIG_DIRS").unwrap_or_else(|_| "/etc/xdg".to_string());
        xdg_dirs
            .split(':')
            .filter(|s| !s.is_empty())
            .map(|dir| PathBuf::from(dir).join("autostart"))
            .collect()
    }

    /// Parses a single .desktop file for autostart properties
    pub fn parse_desktop_file(path: &Path) -> Option<HashMap<String, String>> {
        let content = fs::read_to_string(path).ok()?;
        let mut map = HashMap::new();
        let mut in_desktop_entry = false;

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with('[') {
                in_desktop_entry = trimmed == "[Desktop Entry]";
                continue;
            }
            if !in_desktop_entry || trimmed.starts_with('#') || !trimmed.contains('=') {
                continue;
            }
            if let Some((key, val)) = trimmed.split_once('=') {
                map.entry(key.trim().to_string())
                    .or_insert_with(|| val.trim().to_string());
            }
        }
        Some(map)
    }

    /// Scans both user and system autostart directories and resolves entries
    pub fn load_entries(&mut self) -> &Vec<AutostartEntry> {
        let mut resolved: HashMap<String, AutostartEntry> = HashMap::new();

        // 1. Load system entries first
        for dir in Self::system_dirs() {
            if let Ok(read_dir) = fs::read_dir(dir) {
                for entry in read_dir.flatten() {
                    let path = entry.path();
                    if path.extension().and_then(|e| e.to_str()) == Some("desktop") {
                        if let Some(id) = path.file_stem().and_then(|s| s.to_str()) {
                            if let Some(fields) = Self::parse_desktop_file(&path) {
                                if Self::is_applicable_to_desktop(&fields) {
                                    let enabled = Self::is_enabled(&fields);
                                    let name = fields.get("Name").cloned().unwrap_or_else(|| id.to_string());
                                    let exec = fields.get("Exec").cloned().unwrap_or_default();
                                    let icon = fields.get("Icon").cloned();
                                    let comment = fields.get("Comment").cloned();

                                    resolved.insert(
                                        id.to_string(),
                                        AutostartEntry {
                                            id: id.to_string(),
                                            name,
                                            exec,
                                            icon,
                                            comment,
                                            enabled,
                                            is_user_entry: false,
                                            path: path.clone(),
                                        },
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }

        // 2. Load user entries which override system entries
        let user_dir = Self::user_dir();
        if let Ok(read_dir) = fs::read_dir(&user_dir) {
            for entry in read_dir.flatten() {
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) == Some("desktop") {
                    if let Some(id) = path.file_stem().and_then(|s| s.to_str()) {
                        if let Some(fields) = Self::parse_desktop_file(&path) {
                            let enabled = Self::is_enabled(&fields);
                            let name = fields.get("Name").cloned().unwrap_or_else(|| id.to_string());
                            let exec = fields.get("Exec").cloned().unwrap_or_default();
                            let icon = fields.get("Icon").cloned();
                            let comment = fields.get("Comment").cloned();

                            resolved.insert(
                                id.to_string(),
                                AutostartEntry {
                                    id: id.to_string(),
                                    name,
                                    exec,
                                    icon,
                                    comment,
                                    enabled,
                                    is_user_entry: true,
                                    path: path.clone(),
                                },
                            );
                        }
                    }
                }
            }
        }

        let mut entries: Vec<AutostartEntry> = resolved.into_values().collect();
        entries.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        self.entries = entries;
        &self.entries
    }

    /// Evaluates `OnlyShowIn` and `NotShowIn` against standard environments (KDE, X-Caelestia)
    fn is_applicable_to_desktop(fields: &HashMap<String, String>) -> bool {
        if let Some(not_show) = fields.get("NotShowIn") {
            let list: Vec<&str> = not_show.split(';').map(|s| s.trim()).collect();
            if list.contains(&"KDE") || list.contains(&"X-Caelestia") {
                return false;
            }
        }
        if let Some(only_show) = fields.get("OnlyShowIn") {
            let list: Vec<&str> = only_show.split(';').map(|s| s.trim()).collect();
            if !list.is_empty() && !list.contains(&"KDE") && !list.contains(&"X-Caelestia") {
                return false;
            }
        }
        true
    }

    /// Evaluates if entry is active or disabled via `Hidden=true` or `X-GNOME-Autostart-enabled=false`
    fn is_enabled(fields: &HashMap<String, String>) -> bool {
        if let Some(hidden) = fields.get("Hidden") {
            if hidden.eq_ignore_ascii_case("true") {
                return false;
            }
        }
        if let Some(gnome) = fields.get("X-GNOME-Autostart-enabled") {
            if gnome.eq_ignore_ascii_case("false") {
                return false;
            }
        }
        true
    }

    /// Toggles the enabled state of an entry. Creates a user override in ~/.config/autostart if needed.
    pub fn set_enabled(&mut self, id: &str, enable: bool) -> std::io::Result<bool> {
        let user_dir = Self::user_dir();
        fs::create_dir_all(&user_dir)?;
        let user_path = user_dir.join(format!("{}.desktop", id));

        if user_path.exists() {
            // Modify existing user entry
            let content = fs::read_to_string(&user_path)?;
            let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
            let mut found_hidden = false;

            for line in &mut lines {
                if line.trim().starts_with("Hidden=") {
                    *line = format!("Hidden={}", if enable { "false" } else { "true" });
                    found_hidden = true;
                }
            }
            if !found_hidden {
                lines.push(format!("Hidden={}", if enable { "false" } else { "true" }));
            }
            fs::write(&user_path, lines.join("\n"))?;
        } else if let Some(sys_entry) = self.entries.iter().find(|e| e.id == id) {
            // Copy system entry content to user dir and add Hidden flag
            let content = fs::read_to_string(&sys_entry.path)?;
            let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
            lines.push(format!("Hidden={}", if enable { "false" } else { "true" }));
            fs::write(&user_path, lines.join("\n"))?;
        }

        self.load_entries();
        Ok(true)
    }

    /// Removes a user autostart entry
    pub fn remove(&mut self, id: &str) -> std::io::Result<bool> {
        let user_path = Self::user_dir().join(format!("{}.desktop", id));
        if user_path.exists() {
            fs::remove_file(user_path)?;
            self.load_entries();
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
