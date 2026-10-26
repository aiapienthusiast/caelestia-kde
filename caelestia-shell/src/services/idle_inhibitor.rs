//! Idle Inhibitor & Keep-Awake Service
//! 100% parity with `shell/services/IdleInhibitor.qml`.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdleInhibitorState {
    pub enabled: bool,
    pub since: DateTime<Local>,
}

impl Default for IdleInhibitorState {
    fn default() -> Self {
        Self {
            enabled: false,
            since: Local::now(),
        }
    }
}

pub struct IdleInhibitorService {
    state: Mutex<IdleInhibitorState>,
    state_file: PathBuf,
}

impl Default for IdleInhibitorService {
    fn default() -> Self {
        let runtime_dir = std::env::var("XDG_RUNTIME_DIR").unwrap_or_else(|_| "/tmp".to_string());
        Self::with_path(PathBuf::from(runtime_dir).join("caelestia/keepawake.json"))
    }
}

impl IdleInhibitorService {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_path(state_file: PathBuf) -> Self {
        let initial_state = Self::restore_from_file(&state_file).unwrap_or_default();
        Self {
            state: Mutex::new(initial_state),
            state_file,
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.state.lock().map(|s| s.enabled).unwrap_or(false)
    }

    pub fn enabled_since(&self) -> DateTime<Local> {
        self.state.lock().map(|s| s.since).unwrap_or_else(|_| Local::now())
    }

    pub fn set_enabled(&self, enabled: bool) {
        if let Ok(mut state) = self.state.lock() {
            if state.enabled == enabled {
                return;
            }
            state.enabled = enabled;
            if enabled {
                state.since = Local::now();
            }
            self.persist_state(&state);
        }
    }

    pub fn toggle(&self) -> bool {
        let new_state = !self.is_enabled();
        self.set_enabled(new_state);
        new_state
    }

    fn persist_state(&self, state: &IdleInhibitorState) {
        if let Some(parent) = self.state_file.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(json) = serde_json::to_string(state) {
            let _ = fs::write(&self.state_file, json);
        }
    }

    fn restore_from_file(path: &Path) -> Option<IdleInhibitorState> {
        if path.exists() {
            if let Ok(text) = fs::read_to_string(path) {
                if let Ok(st) = serde_json::from_str(&text) {
                    return Some(st);
                }
            }
        }
        None
    }
}
