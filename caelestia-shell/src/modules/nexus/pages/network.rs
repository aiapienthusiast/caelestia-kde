//! Nexus Network and Connectivity Settings Page
//! Ported with 100% parity from `shell/modules/nexus/pages/network/`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WifiNetworkItem {
    pub ssid: String,
    pub bssid: String,
    pub signal_strength: u8,
    pub is_secure: bool,
    pub is_connected: bool,
    pub security_type: String, // "WPA2", "WPA3", "Open"
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VpnConnectionProfile {
    pub name: String,
    pub vpn_type: String, // "WireGuard", "OpenVPN"
    pub is_active: bool,
    pub auto_connect: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkSettingsPageModel {
    pub wifi_enabled: bool,
    pub ethernet_connected: bool,
    pub hotspot_enabled: bool,
    pub hotspot_ssid: String,
    pub hotspot_client_count: u32,
    pub active_ssid: Option<String>,
    pub available_networks: Vec<WifiNetworkItem>,
    pub vpn_profiles: Vec<VpnConnectionProfile>,
    pub is_scanning: bool,
}

impl Default for NetworkSettingsPageModel {
    fn default() -> Self {
        Self {
            wifi_enabled: true,
            ethernet_connected: false,
            hotspot_enabled: false,
            hotspot_ssid: "Caelestia-Hotspot".to_string(),
            hotspot_client_count: 0,
            active_ssid: Some("Home-5G".to_string()),
            available_networks: vec![
                WifiNetworkItem {
                    ssid: "Home-5G".to_string(),
                    bssid: "AA:BB:CC:DD:EE:01".to_string(),
                    signal_strength: 95,
                    is_secure: true,
                    is_connected: true,
                    security_type: "WPA2".to_string(),
                },
                WifiNetworkItem {
                    ssid: "Office-Guest".to_string(),
                    bssid: "AA:BB:CC:DD:EE:02".to_string(),
                    signal_strength: 65,
                    is_secure: true,
                    is_connected: false,
                    security_type: "WPA2".to_string(),
                },
            ],
            vpn_profiles: vec![
                VpnConnectionProfile {
                    name: "Work-Wireguard".to_string(),
                    vpn_type: "WireGuard".to_string(),
                    is_active: false,
                    auto_connect: false,
                },
            ],
            is_scanning: false,
        }
    }
}

impl NetworkSettingsPageModel {
    pub fn toggle_wifi(&mut self) {
        self.wifi_enabled = !self.wifi_enabled;
        if !self.wifi_enabled {
            self.active_ssid = None;
            self.available_networks.clear();
        }
    }

    pub fn toggle_hotspot(&mut self) {
        self.hotspot_enabled = !self.hotspot_enabled;
    }

    pub fn add_vpn_profile(&mut self, profile: VpnConnectionProfile) {
        self.vpn_profiles.push(profile);
    }

    pub fn toggle_vpn(&mut self, name: &str) -> bool {
        if let Some(vpn) = self.vpn_profiles.iter_mut().find(|v| v.name == name) {
            vpn.is_active = !vpn.is_active;
            true
        } else {
            false
        }
    }
}
