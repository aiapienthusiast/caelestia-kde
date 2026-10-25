//! Night Light Service
//! Parity with `nightcolorbridge.cpp` and `shell/services/HyprSunset.qml`.
//! Manages KDE KWin Night Light colour temperature and active state via D-Bus.

use std::process::Command;
use tracing::info;

#[derive(Clone, Debug, PartialEq)]
pub struct NightLightState {
    pub enabled: bool,
    pub running: bool,
    pub current_temperature: u32,
    pub target_temperature: u32,
}

impl Default for NightLightState {
    fn default() -> Self {
        Self {
            enabled: true,
            running: false,
            current_temperature: 6500,
            target_temperature: 4500,
        }
    }
}

pub struct NightLightService {
    pub state: NightLightState,
}

impl NightLightService {
    pub fn new() -> Self {
        let mut service = Self {
            state: NightLightState::default(),
        };
        if !crate::infrastructure::is_test_environment() {
            service.poll();
        }
        service
    }

    /// Refresh Night Light properties from KWin D-Bus
    pub fn poll(&mut self) {
        if crate::infrastructure::is_test_environment() {
            return;
        }
        if let Ok(output) = Command::new("qdbus6")
            .args(["org.kde.KWin", "/org/kde/KWin/NightLight", "org.kde.KWin.NightLight.running"])
            .output()
        {
            if output.status.success() {
                let text = String::from_utf8_lossy(&output.stdout);
                self.state.running = text.trim() == "true";
            }
        }

        if let Ok(output) = Command::new("qdbus6")
            .args(["org.kde.KWin", "/org/kde/KWin/NightLight", "org.kde.KWin.NightLight.enabled"])
            .output()
        {
            if output.status.success() {
                let text = String::from_utf8_lossy(&output.stdout);
                self.state.enabled = text.trim() == "true";
            }
        }

        if let Ok(output) = Command::new("qdbus6")
            .args(["org.kde.KWin", "/org/kde/KWin/NightLight", "org.kde.KWin.NightLight.currentTemperature"])
            .output()
        {
            if output.status.success() {
                let text = String::from_utf8_lossy(&output.stdout);
                if let Ok(temp) = text.trim().parse::<u32>() {
                    self.state.current_temperature = temp;
                }
            }
        }
    }

    /// Returns command specification for night light preview via qdbus6
    pub fn command_for_preview(temp: u32) -> (&'static str, Vec<String>) {
        (
            "qdbus6",
            vec![
                "org.kde.KWin".to_string(),
                "/org/kde/KWin/NightLight".to_string(),
                "org.kde.KWin.NightLight.preview".to_string(),
                temp.clamp(1000, 10000).to_string(),
            ],
        )
    }

    /// Returns command specification for stopping night light preview
    pub fn command_for_stop_preview() -> (&'static str, [&'static str; 3]) {
        (
            "qdbus6",
            [
                "org.kde.KWin",
                "/org/kde/KWin/NightLight",
                "org.kde.KWin.NightLight.stopPreview",
            ],
        )
    }

    /// Toggle Night Light active state
    pub fn toggle(&mut self) {
        self.state.running = !self.state.running;
        info!("Toggling Night Light active state to: {}", self.state.running);

        let target_temp = if self.state.running {
            self.state.target_temperature
        } else {
            6500
        };

        if !crate::infrastructure::is_test_environment() {
            if self.state.running {
                let (prog, args) = Self::command_for_preview(target_temp);
                let _ = Command::new(prog).args(args).spawn();
            } else {
                let (prog, args) = Self::command_for_stop_preview();
                let _ = Command::new(prog).args(args).spawn();
            }
        }
    }

    /// Set manual temperature preview
    pub fn set_temperature(&mut self, temp: u32) {
        let clamped = temp.clamp(1000, 10000);
        self.state.current_temperature = clamped;
        if !crate::infrastructure::is_test_environment() {
            let (prog, args) = Self::command_for_preview(clamped);
            let _ = Command::new(prog).args(args).spawn();
        }
    }
}
