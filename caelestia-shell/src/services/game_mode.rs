//! Game Mode Performance Optimizer Service
//! 100% parity with `shell/services/GameMode.qml`.

use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Mutex;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameModeState {
    pub enabled: bool,
    pub auto_enabled: bool,
    pub restore_video_wallpaper: bool,
    pub active_game_title: Option<String>,
}

impl Default for GameModeState {
    fn default() -> Self {
        Self {
            enabled: false,
            auto_enabled: false,
            restore_video_wallpaper: false,
            active_game_title: None,
        }
    }
}

pub struct GameModeService {
    state: Mutex<GameModeState>,
    auto_rules: Mutex<Vec<String>>,
    state_file: PathBuf,
}

impl Default for GameModeService {
    fn default() -> Self {
        let cache_dir = std::env::var("XDG_CACHE_HOME")
            .unwrap_or_else(|_| format!("{}/.cache", std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string())));
        Self::with_cache(PathBuf::from(cache_dir).join("caelestia/gamemode-state"))
    }
}

impl GameModeService {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_cache(state_file: PathBuf) -> Self {
        Self {
            state: Mutex::new(GameModeState::default()),
            auto_rules: Mutex::new(vec![
                "^steam_app_\\d+$".to_string(),
                "^heroic$".to_string(),
                "^lutris$".to_string(),
                "^gamescope$".to_string(),
                "^wine$".to_string(),
                "^proton$".to_string(),
                "^osu!$".to_string(),
                "^minecraft$".to_string(),
            ]),
            state_file,
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.state.lock().map(|s| s.enabled).unwrap_or(false)
    }

    pub fn set_rules(&self, rules: Vec<String>) {
        if let Ok(mut r) = self.auto_rules.lock() {
            *r = rules;
        }
    }

    /// Evaluates window attributes (class, initial_class, title) against game rules
    pub fn matches_rule(&self, window_class: &str, window_title: &str) -> bool {
        if let Ok(rules) = self.auto_rules.lock() {
            for rule in rules.iter() {
                if rule.is_empty() {
                    continue;
                }
                if window_class.eq_ignore_ascii_case(rule) || window_title.eq_ignore_ascii_case(rule) {
                    return true;
                }
                if let Ok(re) = Regex::new(rule) {
                    if re.is_match(window_class) || re.is_match(window_title) {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Reacts to window list changes, automatically engaging or disengaging Game Mode
    pub fn update_windows(&self, windows: &[(String, String)]) {
        let mut matched_title = None;
        for (class, title) in windows {
            if self.matches_rule(class, title) {
                matched_title = Some(title.clone());
                break;
            }
        }

        if let Ok(mut state) = self.state.lock() {
            if matched_title.is_some() && !state.enabled {
                state.auto_enabled = true;
                state.active_game_title = matched_title;
                self.apply_system_tuning(true);
                state.enabled = true;
            } else if matched_title.is_none() && state.enabled && state.auto_enabled {
                state.auto_enabled = false;
                state.active_game_title = None;
                self.apply_system_tuning(false);
                state.enabled = false;
            }
        }
    }

    /// Manually toggle or set Game Mode
    pub fn set_enabled(&self, enabled: bool) {
        if let Ok(mut state) = self.state.lock() {
            if state.enabled == enabled {
                return;
            }
            if !enabled {
                state.auto_enabled = false;
                state.active_game_title = None;
            }
            state.enabled = enabled;
            self.apply_system_tuning(enabled);
        }
    }

    fn apply_system_tuning(&self, enable: bool) {
        if enable {
            // Backup current blur & animation settings if not already backed up
            if !self.state_file.exists() {
                if let Some(parent) = self.state_file.parent() {
                    let _ = fs::create_dir_all(parent);
                }
                let prev_blur = Self::read_kwin_config("kwinrc", "Plugins", "blurEnabled", "true");
                let prev_anim = Self::read_kwin_config("kdeglobals", "KDE", "AnimationDurationFactor", "1");
                let _ = fs::write(&self.state_file, format!("{}\n{}\n", prev_blur, prev_anim));
            }

            // Disable compositor blur and animations for max frame rates
            let _ = Command::new("kwriteconfig6")
                .args(["--file", "kwinrc", "--group", "Plugins", "--key", "blurEnabled", "false"])
                .status();
            let _ = Command::new("kwriteconfig6")
                .args(["--file", "kdeglobals", "--group", "KDE", "--key", "AnimationDurationFactor", "--notify", "0"])
                .status();
            let _ = Command::new("qdbus6")
                .args(["org.kde.KWin", "/KWin", "reconfigure"])
                .status();
        } else {
            // Restore previous settings from backup file
            if let Ok(content) = fs::read_to_string(&self.state_file) {
                let lines: Vec<&str> = content.lines().collect();
                let blur = lines.first().copied().unwrap_or("true");
                let anim = lines.get(1).copied().unwrap_or("1");

                let _ = Command::new("kwriteconfig6")
                    .args(["--file", "kwinrc", "--group", "Plugins", "--key", "blurEnabled", blur])
                    .status();
                let _ = Command::new("kwriteconfig6")
                    .args(["--file", "kdeglobals", "--group", "KDE", "--key", "AnimationDurationFactor", "--notify", anim])
                    .status();
                let _ = Command::new("qdbus6")
                    .args(["org.kde.KWin", "/KWin", "reconfigure"])
                    .status();
            }
            let _ = fs::remove_file(&self.state_file);
        }
    }

    fn read_kwin_config(file: &str, group: &str, key: &str, default: &str) -> String {
        Command::new("kreadconfig6")
            .args(["--file", file, "--group", group, "--key", key, "--default", default])
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_else(|_| default.to_string())
    }
}
