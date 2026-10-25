//! Quick Toggles Card Subsystem
//! Complete parity with `shell/modules/utilities/cards/Toggles.qml`.

use super::config::QuickToggleItem;
use crate::services::audio::AudioService;
use crate::services::bluetooth::BluetoothService;
use crate::services::network::NetworkService;
use crate::services::night_light::NightLightService;
use crate::services::notifications::NotificationsService;
use serde::{Deserialize, Serialize};
use std::process::Command;
use tracing::info;

/// Controller for quick toggles grid with pagination and service synchronization
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuickTogglesModel {
    pub toggles: Vec<QuickToggleItem>,
    pub per_page: usize,
    pub current_page: usize,
    pub wallpaper_paused: bool,
    pub hotspot_active: bool,
}

impl Default for QuickTogglesModel {
    fn default() -> Self {
        Self {
            toggles: Vec::new(),
            per_page: 6,
            current_page: 0,
            wallpaper_paused: false,
            hotspot_active: false,
        }
    }
}

impl QuickTogglesModel {
    pub fn new(toggles: Vec<QuickToggleItem>, per_page: usize) -> Self {
        Self {
            toggles,
            per_page: per_page.max(2),
            current_page: 0,
            wallpaper_paused: false,
            hotspot_active: false,
        }
    }

    /// Total number of pages
    pub fn page_count(&self) -> usize {
        let n = self.toggles.len();
        if n == 0 {
            1
        } else {
            (n + self.per_page - 1) / self.per_page
        }
    }

    /// Slice of toggles for the current page
    pub fn page_slice(&self) -> &[QuickToggleItem] {
        let start = self.current_page * self.per_page;
        if start >= self.toggles.len() {
            &[]
        } else {
            let end = (start + self.per_page).min(self.toggles.len());
            &self.toggles[start..end]
        }
    }

    /// Go to next page
    pub fn next_page(&mut self) {
        if self.current_page + 1 < self.page_count() {
            self.current_page += 1;
        }
    }

    /// Go to previous page
    pub fn prev_page(&mut self) {
        if self.current_page > 0 {
            self.current_page -= 1;
        }
    }

    /// Determine if a toggle is currently active based on real services
    pub fn is_active(
        &self,
        id: &str,
        network: Option<&NetworkService>,
        bluetooth: Option<&BluetoothService>,
        night_light: Option<&NightLightService>,
        notif: Option<&NotificationsService>,
        audio: Option<&AudioService>,
    ) -> bool {
        match id {
            "wifi" => network.map(|n| n.state.wifi_enabled).unwrap_or(false),
            "bluetooth" => bluetooth.map(|b| b.state.powered).unwrap_or(false),
            "nightlight" => night_light.map(|nl| nl.state.enabled).unwrap_or(false),
            "dnd" => notif.map(|n| n.state.dnd).unwrap_or(false),
            "mic_mute" => audio.map(|a| a.state.source_muted).unwrap_or(false),
            "hotspot" => self.hotspot_active,
            "pause_wallpaper" => self.wallpaper_paused,
            _ => false,
        }
    }

    /// Execute a toggle action
    pub fn toggle_action(
        &mut self,
        id: &str,
        network: Option<&mut NetworkService>,
        bluetooth: Option<&mut BluetoothService>,
        night_light: Option<&mut NightLightService>,
        notif: Option<&mut NotificationsService>,
        audio: Option<&mut AudioService>,
    ) {
        info!("Executing quick toggle action for: {}", id);
        match id {
            "wifi" => {
                if let Some(net) = network {
                    net.toggle_wifi();
                }
            }
            "bluetooth" => {
                if let Some(bt) = bluetooth {
                    bt.toggle_power();
                }
            }
            "nightlight" => {
                if let Some(nl) = night_light {
                    nl.toggle();
                }
            }
            "dnd" => {
                if let Some(n) = notif {
                    n.toggle_dnd();
                }
            }
            "mic_mute" => {
                if let Some(a) = audio {
                    a.toggle_source_mute();
                }
            }
            "pause_wallpaper" => {
                self.wallpaper_paused = !self.wallpaper_paused;
            }
            "hotspot" => {
                self.hotspot_active = !self.hotspot_active;
                let _ = Command::new("nmcli")
                    .args([
                        "connection",
                        if self.hotspot_active { "up" } else { "down" },
                        "Hotspot",
                    ])
                    .spawn();
            }
            "restart_shell" => {
                info!("Restarting Caelestia Shell via systemctl");
                let _ = Command::new("systemctl")
                    .args(["--user", "restart", "caelestia-shell.service"])
                    .spawn();
            }
            _ => {}
        }
    }
}
