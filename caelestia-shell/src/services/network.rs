//! Network Service
//! Parity with `shell/services/Nmcli.qml` and `NmQt.cpp`.
//! Manages NetworkManager device statuses, WiFi networks, and connections.

use std::process::Command;
use tracing::info;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NetworkKind {
    Wifi,
    Ethernet,
    Disconnected,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AccessPoint {
    pub ssid: String,
    pub signal: u8,
    pub security: String,
    pub in_use: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NetworkState {
    pub kind: NetworkKind,
    pub connected: bool,
    pub active_ssid: Option<String>,
    pub signal_strength: u8,
    pub wifi_enabled: bool,
    pub ethernet_connected: bool,
    pub access_points: Vec<AccessPoint>,
}

impl Default for NetworkState {
    fn default() -> Self {
        Self {
            kind: NetworkKind::Disconnected,
            connected: false,
            active_ssid: None,
            signal_strength: 0,
            wifi_enabled: true,
            ethernet_connected: false,
            access_points: Vec::new(),
        }
    }
}

pub struct NetworkService {
    pub state: NetworkState,
    pub is_mock: bool,
}

impl NetworkService {
    pub fn new() -> Self {
        let mut service = Self {
            state: NetworkState::default(),
            is_mock: false,
        };
        if !crate::infrastructure::is_test_environment() {
            service.poll();
        }
        service
    }

    /// Creates an in-memory mock network service for unit tests without system calls
    pub fn new_mock(
        wifi_enabled: bool,
        connected_ssid: Option<String>,
        access_points: Vec<AccessPoint>,
    ) -> Self {
        let connected = connected_ssid.is_some();
        let kind = if connected {
            NetworkKind::Wifi
        } else {
            NetworkKind::Disconnected
        };
        let signal = connected_ssid
            .as_ref()
            .and_then(|ssid| access_points.iter().find(|ap| &ap.ssid == ssid).map(|ap| ap.signal))
            .unwrap_or(0);

        Self {
            state: NetworkState {
                kind,
                connected,
                active_ssid: connected_ssid,
                signal_strength: signal,
                wifi_enabled,
                ethernet_connected: false,
                access_points,
            },
            is_mock: true,
        }
    }

    pub fn new_simulated(
        wifi_enabled: bool,
        connected_ssid: Option<String>,
        access_points: Vec<AccessPoint>,
    ) -> Self {
        Self::new_mock(wifi_enabled, connected_ssid, access_points)
    }

    /// Refresh device status and active connections
    pub fn poll(&mut self) {
        if self.is_mock || crate::infrastructure::is_test_environment() {
            return;
        }
        if let Ok(output) = Command::new("nmcli")
            .args(["-t", "-f", "TYPE,STATE,CONNECTION", "device", "status"])
            .output()
        {
            if output.status.success() {
                let text = String::from_utf8_lossy(&output.stdout);
                let (kind, connected, active_ssid, eth) = Self::parse_device_status(&text);
                self.state.kind = kind;
                self.state.connected = connected;
                self.state.active_ssid = active_ssid;
                self.state.ethernet_connected = eth;
            }
        }

        // Check if wifi is enabled
        if let Ok(output) = Command::new("nmcli").args(["radio", "wifi"]).output() {
            if output.status.success() {
                let text = String::from_utf8_lossy(&output.stdout);
                self.state.wifi_enabled = text.trim() == "enabled";
            }
        }

        // Refresh nearby WiFi access points if WiFi is enabled
        if self.state.wifi_enabled {
            if let Ok(output) = Command::new("nmcli")
                .args(["-t", "-f", "IN-USE,SSID,SIGNAL,SECURITY", "device", "wifi", "list"])
                .output()
            {
                if output.status.success() {
                    let text = String::from_utf8_lossy(&output.stdout);
                    self.state.access_points = Self::parse_wifi_list(&text);
                    if let Some(active) = self.state.access_points.iter().find(|ap| ap.in_use) {
                        self.state.signal_strength = active.signal;
                    }
                }
            }
        }
    }

    /// Parse colon-delimited device status output from `nmcli -t -f TYPE,STATE,CONNECTION device status`
    pub fn parse_device_status(output: &str) -> (NetworkKind, bool, Option<String>, bool) {
        let mut wifi_connected = false;
        let mut ethernet_connected = false;
        let mut active_ssid = None;

        for line in output.lines() {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 2 {
                let dev_type = parts[0];
                let state = parts[1];
                let conn = if parts.len() >= 3 { parts[2] } else { "" };

                if dev_type == "ethernet" && state.starts_with("connected") {
                    ethernet_connected = true;
                } else if dev_type == "wifi" && state.starts_with("connected") {
                    wifi_connected = true;
                    if !conn.is_empty() {
                        active_ssid = Some(conn.to_string());
                    }
                }
            }
        }

        if ethernet_connected {
            (NetworkKind::Ethernet, true, active_ssid, true)
        } else if wifi_connected {
            (NetworkKind::Wifi, true, active_ssid, false)
        } else {
            (NetworkKind::Disconnected, false, None, false)
        }
    }

    /// Parse colon-delimited WiFi list from `nmcli -t -f IN-USE,SSID,SIGNAL,SECURITY device wifi list`
    pub fn parse_wifi_list(output: &str) -> Vec<AccessPoint> {
        let mut aps = Vec::new();

        for line in output.lines() {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 3 {
                let in_use = parts[0].trim() == "*";
                let ssid = parts[1].trim();
                if ssid.is_empty() {
                    continue; // Skip hidden networks without SSID
                }

                let signal = parts[2].trim().parse::<u8>().unwrap_or(0);
                let security = if parts.len() >= 4 {
                    parts[3].trim().to_string()
                } else {
                    String::new()
                };

                // Deduplicate SSIDs, keeping higher signal or currently in-use
                if let Some(existing) = aps.iter_mut().find(|a: &&mut AccessPoint| a.ssid == ssid) {
                    if in_use || (!existing.in_use && signal > existing.signal) {
                        existing.signal = signal;
                        existing.in_use = in_use;
                        if !security.is_empty() {
                            existing.security = security;
                        }
                    }
                } else {
                    aps.push(AccessPoint {
                        ssid: ssid.to_string(),
                        signal,
                        security,
                        in_use,
                    });
                }
            }
        }

        // Sort: in-use first, then by signal strength descending
        aps.sort_by(|a, b| {
            b.in_use
                .cmp(&a.in_use)
                .then_with(|| b.signal.cmp(&a.signal))
        });

        aps
    }

    /// Returns command specification for toggling wifi radio
    pub fn command_for_wifi_toggle(enabled: bool) -> (&'static str, [&'static str; 3]) {
        (
            "nmcli",
            [
                "radio",
                "wifi",
                if enabled { "on" } else { "off" },
            ],
        )
    }

    /// Returns command specification for connecting to a wifi SSID
    pub fn command_for_connect(ssid: &str, password: Option<&str>) -> (&'static str, Vec<String>) {
        let mut args = vec![
            "device".to_string(),
            "wifi".to_string(),
            "connect".to_string(),
            ssid.to_string(),
        ];
        if let Some(pass) = password {
            args.push("password".to_string());
            args.push(pass.to_string());
        }
        ("nmcli", args)
    }

    /// Returns command specification for disconnecting an SSID
    pub fn command_for_disconnect(ssid: &str) -> (&'static str, Vec<String>) {
        (
            "nmcli",
            vec![
                "connection".to_string(),
                "down".to_string(),
                ssid.to_string(),
            ],
        )
    }

    /// Toggle WiFi radio on/off
    pub fn toggle_wifi(&mut self) {
        let next_state = !self.state.wifi_enabled;
        self.state.wifi_enabled = next_state;
        if !self.is_mock && !crate::infrastructure::is_test_environment() {
            let (prog, args) = Self::command_for_wifi_toggle(next_state);
            let _ = Command::new(prog).args(args).spawn();
        }
    }

    /// Connect to a WiFi network
    pub fn connect(&mut self, ssid: &str, password: Option<&str>) {
        info!("Connecting to WiFi network: {}", ssid);
        if self.is_mock || crate::infrastructure::is_test_environment() {
            self.state.active_ssid = Some(ssid.to_string());
            self.state.connected = true;
            self.state.kind = NetworkKind::Wifi;
            for ap in &mut self.state.access_points {
                ap.in_use = ap.ssid == ssid;
            }
            return;
        }
        let (prog, args) = Self::command_for_connect(ssid, password);
        let _ = Command::new(prog).args(args).spawn();
    }

    /// Disconnect active WiFi connection
    pub fn disconnect(&mut self) {
        if let Some(ref ssid) = self.state.active_ssid.clone() {
            info!("Disconnecting WiFi network: {}", ssid);
            if self.is_mock || crate::infrastructure::is_test_environment() {
                self.state.active_ssid = None;
                self.state.connected = false;
                self.state.kind = NetworkKind::Disconnected;
                for ap in &mut self.state.access_points {
                    ap.in_use = false;
                }
                return;
            }
            let (prog, args) = Self::command_for_disconnect(&ssid);
            let _ = Command::new(prog).args(args).spawn();
        }
    }
}
