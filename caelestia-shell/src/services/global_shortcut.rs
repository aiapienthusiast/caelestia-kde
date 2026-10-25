//! Global Shortcut Service & Collision Coordinator
//! Parity with `shell/plugin/src/Caelestia/Services/globalshortcut.cpp`.
//!
//! Manages system-wide global shortcuts via `org.kde.KGlobalAccel` over D-Bus,
//! handles collision detection with existing KDE keybindings, and manages
//! `stolen-shortcuts.json` persistence for clean restoration across sessions/crashes.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::{Arc, Mutex, OnceLock};

/// Record of a shortcut stolen from another KDE component to restore on exit/crash.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StolenShortcut {
    pub component: String,
    pub action: String,
    #[serde(rename = "componentFriendlyName", default)]
    pub component_friendly_name: String,
    #[serde(rename = "actionFriendlyName", default)]
    pub action_friendly_name: String,
    pub keys: Vec<String>,
    #[serde(rename = "triggerKey", default)]
    pub trigger_key: String,
}

/// A registered global shortcut entry in the Caelestia shell.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisteredShortcut {
    pub name: String,
    pub description: String,
    pub key: String,
    pub enabled: bool,
    pub stolen_shortcuts: Vec<StolenShortcut>,
}

/// Global shortcut dispatcher singleton managing collisions and crash recovery.
#[derive(Debug, Default)]
pub struct GlobalShortcutDispatcher {
    registry: HashMap<String, RegisteredShortcut>,
    collision_index: HashMap<String, String>,
}

static DISPATCHER_INSTANCE: OnceLock<Arc<Mutex<GlobalShortcutDispatcher>>> = OnceLock::new();

impl GlobalShortcutDispatcher {
    /// Returns the global static dispatcher instance.
    pub fn global() -> Arc<Mutex<Self>> {
        DISPATCHER_INSTANCE
            .get_or_init(|| {
                let dispatcher = Arc::new(Mutex::new(Self::default()));
                // Run crash recovery on first access
                Self::run_crash_recovery(&dispatcher);
                dispatcher
            })
            .clone()
    }

    /// Path to `~/.config/caelestia/stolen-shortcuts.json`
    pub fn stolen_shortcuts_path() -> PathBuf {
        let base = std::env::var("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
                PathBuf::from(home).join(".config")
            });
        base.join("caelestia").join("stolen-shortcuts.json")
    }

    /// Crash recovery: if `stolen-shortcuts.json` exists on startup, restore everything and clear.
    pub fn run_crash_recovery(dispatcher: &Arc<Mutex<Self>>) {
        let path = Self::stolen_shortcuts_path();
        if !path.exists() {
            return;
        }

        if let Ok(data) = fs::read_to_string(&path) {
            if let Ok(entries) = serde_json::from_str::<Vec<StolenShortcut>>(&data) {
                let mut lock = dispatcher.lock().unwrap_or_else(|e| e.into_inner());
                for stolen in &entries {
                    Self::restore_kglobalaccel_shortcut(stolen);

                    let label = if !stolen.component_friendly_name.is_empty() {
                        &stolen.component_friendly_name
                    } else {
                        &stolen.component
                    };
                    let action_label = if !stolen.action_friendly_name.is_empty() {
                        &stolen.action_friendly_name
                    } else {
                        &stolen.action
                    };
                    let friendly = format!("{} - {}", label, action_label);

                    for k in &stolen.keys {
                        lock.collision_index.insert(k.clone(), friendly.clone());
                    }
                }
            }
        }

        // Clean up recovery file after restoration
        let _ = fs::remove_file(&path);
    }

    /// Invokes `gdbus` to restore shortcut keys on `org.kde.KGlobalAccel`
    pub fn restore_kglobalaccel_shortcut(stolen: &StolenShortcut) {
        if crate::infrastructure::is_test_environment() {
            return;
        }

        let escaped_component = stolen.component.replace('\\', "\\\\").replace('\'', "\\'");
        let escaped_action = stolen.action.replace('\\', "\\\\").replace('\'', "\\'");

        // Format array of key sequence tuples
        let seq_strings: Vec<String> = stolen
            .keys
            .iter()
            .map(|k| format!("(['{}'],)", k))
            .collect();
        let array_str = if seq_strings.is_empty() {
            "[([0, 0, 0, 0],)]".to_string()
        } else {
            format!("[{}]", seq_strings.join(", "))
        };

        let _ = Command::new("gdbus")
            .args([
                "call",
                "--session",
                "--dest",
                "org.kde.kglobalaccel",
                "--object-path",
                "/kglobalaccel",
                "--method",
                "org.kde.KGlobalAccel.setShortcutKeys",
                &format!("['{}', '{}', '', '']", escaped_component, escaped_action),
                &array_str,
                "4",
            ])
            .spawn();
    }

    /// Clean exit: restores all currently stolen shortcuts and removes recovery file.
    pub fn clean_exit(&mut self) {
        for sc in self.registry.values() {
            for stolen in &sc.stolen_shortcuts {
                Self::restore_kglobalaccel_shortcut(stolen);
            }
        }
        let _ = fs::remove_file(Self::stolen_shortcuts_path());
    }

    /// Persists all currently stolen shortcuts to `stolen-shortcuts.json`.
    pub fn persist_stolen_shortcuts(&mut self) {
        let mut all_stolen = Vec::new();
        for sc in self.registry.values() {
            all_stolen.extend(sc.stolen_shortcuts.clone());
        }

        let path = Self::stolen_shortcuts_path();
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        if let Ok(json) = serde_json::to_string_pretty(&all_stolen) {
            let _ = fs::write(&path, json);
        }

        self.rebuild_collision_index();
    }

    /// Registers or updates a shortcut in the registry.
    pub fn register(&mut self, shortcut: RegisteredShortcut) {
        self.registry.insert(shortcut.name.clone(), shortcut);
        self.persist_stolen_shortcuts();
    }

    /// Unregisters a shortcut by name.
    pub fn unregister(&mut self, name: &str) {
        if let Some(sc) = self.registry.remove(name) {
            for stolen in &sc.stolen_shortcuts {
                Self::restore_kglobalaccel_shortcut(stolen);
            }
            self.persist_stolen_shortcuts();
        }
    }

    /// Returns collision warning string for a key combination, if any.
    pub fn collision_for_key(&self, key: &str) -> Option<String> {
        self.collision_index.get(key).cloned()
    }

    /// Rebuilds the collision index across all stolen shortcuts and duplicate internal shortcuts.
    pub fn rebuild_collision_index(&mut self) {
        self.collision_index.clear();

        // 1. External stolen shortcuts collisions
        for sc in self.registry.values() {
            for stolen in &sc.stolen_shortcuts {
                let comp = if !stolen.component_friendly_name.is_empty() {
                    &stolen.component_friendly_name
                } else {
                    &stolen.component
                };
                let act = if !stolen.action_friendly_name.is_empty() {
                    &stolen.action_friendly_name
                } else {
                    &stolen.action
                };
                let label = format!("{} - {}", comp, act);
                for k in &stolen.keys {
                    self.collision_index.insert(k.clone(), label.clone());
                }
            }
        }

        // 2. Internal collisions (duplicate keys within Caelestia)
        let mut key_owners: HashMap<String, Vec<String>> = HashMap::new();
        for sc in self.registry.values() {
            if sc.enabled && !sc.key.is_empty() {
                for part in sc.key.split(';') {
                    let trimmed = part.trim();
                    if !trimmed.is_empty() {
                        let label = if !sc.description.is_empty() {
                            sc.description.clone()
                        } else {
                            sc.name.clone()
                        };
                        key_owners
                            .entry(trimmed.to_string())
                            .or_default()
                            .push(label);
                    }
                }
            }
        }

        for (key, mut owners) in key_owners {
            if owners.len() > 1 && !self.collision_index.contains_key(&key) {
                owners.sort();
                owners.dedup();
                self.collision_index
                    .insert(key, format!("Caelestia - {}", owners.join(", ")));
            }
        }
    }

    pub fn all_shortcuts(&self) -> Vec<RegisteredShortcut> {
        self.registry.values().cloned().collect()
    }
}
