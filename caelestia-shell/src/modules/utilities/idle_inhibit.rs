//! Idle Inhibit Subsystem (Keep Awake)
//! Complete parity with `shell/modules/utilities/cards/IdleInhibit.qml` and `IdleInhibitor.qml`.

use serde::{Deserialize, Serialize};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::info;

/// Idle Inhibit (Keep Awake) manager preventing display sleep and suspend
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdleInhibitState {
    pub enabled: bool,
    pub enabled_since_secs: Option<u64>,
    pub cookie: Option<u32>,
}

impl Default for IdleInhibitState {
    fn default() -> Self {
        Self {
            enabled: false,
            enabled_since_secs: None,
            cookie: None,
        }
    }
}

pub struct IdleInhibitor {
    pub state: IdleInhibitState,
}

impl IdleInhibitor {
    pub fn new() -> Self {
        Self {
            state: IdleInhibitState::default(),
        }
    }

    /// Enable or disable sleep inhibitor
    pub fn set_enabled(&mut self, enabled: bool) {
        if self.state.enabled == enabled {
            return;
        }

        self.state.enabled = enabled;
        if enabled {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            self.state.enabled_since_secs = Some(now);
            info!("Keep Awake enabled at epoch timestamp: {}", now);

            // Invoke D-Bus ScreenSaver Inhibit
            let _ = Command::new("qdbus6")
                .args([
                    "org.freedesktop.ScreenSaver",
                    "/ScreenSaver",
                    "org.freedesktop.ScreenSaver.Inhibit",
                    "caelestia-shell",
                    "User requested Keep Awake",
                ])
                .spawn();
        } else {
            info!("Keep Awake disabled");
            self.state.enabled_since_secs = None;

            if let Some(cookie) = self.state.cookie.take() {
                let _ = Command::new("qdbus6")
                    .args([
                        "org.freedesktop.ScreenSaver",
                        "/ScreenSaver",
                        "org.freedesktop.ScreenSaver.UnInhibit",
                        &cookie.to_string(),
                    ])
                    .spawn();
            }
        }
    }

    /// Toggle keep awake state
    pub fn toggle(&mut self) {
        let next = !self.state.enabled;
        self.set_enabled(next);
    }

    /// Format human readable active duration string
    pub fn active_time_string(&self) -> Option<String> {
        let since = self.state.enabled_since_secs?;
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let elapsed = now.saturating_sub(since);

        let hours = elapsed / 3600;
        let mins = (elapsed % 3600) / 60;
        let secs = elapsed % 60;

        if hours > 0 {
            Some(format!("{:02}:{:02}:{:02}", hours, mins, secs))
        } else {
            Some(format!("{:02}:{:02}", mins, secs))
        }
    }
}
