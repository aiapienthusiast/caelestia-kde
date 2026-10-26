//! Nexus Background Services Configuration Page
//! Ported with 100% parity from `shell/modules/nexus/pages/services/`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServicesSettingsPageModel {
    pub gamemode_enabled: bool,
    pub gamemode_registered_apps: Vec<String>,
    pub quickshare_enabled: bool,
    pub quickshare_device_name: String,
    pub quickshare_visible_to_everyone: bool,
    pub focus_mode_active: bool,
    pub focus_duration_mins: u32,
    pub dnd_enabled: bool,
    pub toast_sound_effects: bool,
    pub idle_inhibitor_active: bool,
}

impl Default for ServicesSettingsPageModel {
    fn default() -> Self {
        Self {
            gamemode_enabled: true,
            gamemode_registered_apps: vec!["steam".to_string(), "heroic".to_string()],
            quickshare_enabled: true,
            quickshare_device_name: "Caelestia Desktop".to_string(),
            quickshare_visible_to_everyone: false,
            focus_mode_active: false,
            focus_duration_mins: 25,
            dnd_enabled: false,
            toast_sound_effects: true,
            idle_inhibitor_active: false,
        }
    }
}

impl ServicesSettingsPageModel {
    pub fn toggle_gamemode(&mut self) {
        self.gamemode_enabled = !self.gamemode_enabled;
    }

    pub fn toggle_quickshare(&mut self) {
        self.quickshare_enabled = !self.quickshare_enabled;
    }

    pub fn toggle_focus_mode(&mut self) {
        self.focus_mode_active = !self.focus_mode_active;
        if self.focus_mode_active {
            self.dnd_enabled = true;
        }
    }

    pub fn toggle_idle_inhibitor(&mut self) {
        self.idle_inhibitor_active = !self.idle_inhibitor_active;
    }
}
