//! VPN service and management implementation.
//!
//! Provides 100% parity with `shell/services/VPN.qml`.
//! Supports WireGuard, Cloudflare WARP, NetBird, Tailscale, and custom VPN adapters.

use std::time::{SystemTime, UNIX_EPOCH};

/// VPN connection state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VpnState {
    Disconnected,
    Connecting,
    Connected,
    Disconnecting,
    Error,
}

impl Default for VpnState {
    fn default() -> Self {
        Self::Disconnected
    }
}

/// Detailed status payload.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct VpnStatusInfo {
    pub connected: bool,
    pub state: String,
    pub reason: String,
    pub auth_url: String,
    pub server: String,
}

/// Dynamic provider configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VpnProvider {
    pub id: String,
    pub name: String,
    pub display_name: String,
    pub interface: String,
    pub connect_cmd: Vec<String>,
    pub disconnect_cmd: Vec<String>,
    pub status_cmd: Vec<String>,
    pub service_name: String,
}

/// Adapter template for built-in providers.
#[derive(Debug, Clone)]
pub struct VpnAdapter {
    pub name: &'static str,
    pub display: &'static str,
    pub iface: &'static str,
    pub default_connect: &'static [&'static str],
    pub default_disconnect: &'static [&'static str],
    pub default_status: &'static [&'static str],
    pub service: &'static str,
}

pub const BUILTIN_ADAPTERS: &[VpnAdapter] = &[
    VpnAdapter {
        name: "wireguard",
        display: "WireGuard",
        iface: "wg0",
        default_connect: &["wg-quick", "up"],
        default_disconnect: &["wg-quick", "down"],
        default_status: &["wg", "show"],
        service: "wg-quick@wg0",
    },
    VpnAdapter {
        name: "warp",
        display: "Cloudflare WARP",
        iface: "warp0",
        default_connect: &["warp-cli", "connect"],
        default_disconnect: &["warp-cli", "disconnect"],
        default_status: &["warp-cli", "status"],
        service: "warp-svc",
    },
    VpnAdapter {
        name: "netbird",
        display: "NetBird",
        iface: "wt0",
        default_connect: &["netbird", "up"],
        default_disconnect: &["netbird", "down"],
        default_status: &["netbird", "status"],
        service: "netbird",
    },
    VpnAdapter {
        name: "tailscale",
        display: "Tailscale",
        iface: "tailscale0",
        default_connect: &["tailscale", "up"],
        default_disconnect: &["tailscale", "down"],
        default_status: &["tailscale", "status", "--json"],
        service: "tailscaled",
    },
];

/// VPN Service instance managing connections, metrics, and provider configurations.
#[derive(Debug)]
pub struct VpnService {
    pub connected: bool,
    pub status: VpnStatusInfo,
    pub connect_pending: bool,
    pub disconnect_pending: bool,
    pub selected_provider_id: String,
    pub connected_since: f64,
    pub bytes_in: String,
    pub bytes_out: String,
    pub ping_ms: i32,
    pub server_location: String,
    pub pending_switch_provider: String,
    pub providers: Vec<VpnProvider>,
}

impl Default for VpnService {
    fn default() -> Self {
        Self::new()
    }
}

impl VpnService {
    #[must_use]
    pub fn new() -> Self {
        let mut default_providers = Vec::new();
        for adapter in BUILTIN_ADAPTERS {
            default_providers.push(VpnProvider {
                id: format!("vpn-builtin-{}", adapter.name),
                name: adapter.name.to_string(),
                display_name: adapter.display.to_string(),
                interface: adapter.iface.to_string(),
                connect_cmd: adapter.default_connect.iter().map(|s| s.to_string()).collect(),
                disconnect_cmd: adapter.default_disconnect.iter().map(|s| s.to_string()).collect(),
                status_cmd: adapter.default_status.iter().map(|s| s.to_string()).collect(),
                service_name: adapter.service.to_string(),
            });
        }

        let selected_id = default_providers
            .first()
            .map(|p| p.id.clone())
            .unwrap_or_default();

        Self {
            connected: false,
            status: VpnStatusInfo {
                connected: false,
                state: "disconnected".to_string(),
                reason: String::new(),
                auth_url: String::new(),
                server: String::new(),
            },
            connect_pending: false,
            disconnect_pending: false,
            selected_provider_id: selected_id,
            connected_since: 0.0,
            bytes_in: "0 B".to_string(),
            bytes_out: "0 B".to_string(),
            ping_ms: -1,
            server_location: String::new(),
            pending_switch_provider: String::new(),
            providers: default_providers,
        }
    }

    #[must_use]
    pub fn is_connecting(&self) -> bool {
        self.connect_pending
    }

    #[must_use]
    pub fn is_disconnecting(&self) -> bool {
        self.disconnect_pending
    }

    #[must_use]
    pub fn active_provider(&self) -> Option<&VpnProvider> {
        self.providers.iter().find(|p| p.id == self.selected_provider_id)
    }

    pub fn set_active_provider(&mut self, id: &str) {
        if self.connected {
            self.pending_switch_provider = id.to_string();
            self.disconnect();
        } else {
            self.selected_provider_id = id.to_string();
        }
    }

    pub fn add_provider(&mut self, provider: VpnProvider) {
        self.providers.push(provider);
    }

    pub fn update_provider(&mut self, index: usize, provider: VpnProvider) -> bool {
        if index < self.providers.len() {
            self.providers[index] = provider;
            true
        } else {
            false
        }
    }

    pub fn delete_provider(&mut self, index: usize) -> bool {
        if index < self.providers.len() {
            self.providers.remove(index);
            if self.providers.is_empty() {
                self.selected_provider_id.clear();
            } else if !self.providers.iter().any(|p| p.id == self.selected_provider_id) {
                self.selected_provider_id = self.providers[0].id.clone();
            }
            true
        } else {
            false
        }
    }

    /// Connect initiates connection for active provider.
    pub fn connect(&mut self) {
        if self.connected || self.connect_pending {
            return;
        }
        self.connect_pending = true;
        self.disconnect_pending = false;
        self.status.state = "connecting".to_string();
    }

    /// Disconnect drops the active connection.
    pub fn disconnect(&mut self) {
        if !self.connected && !self.connect_pending {
            return;
        }
        self.disconnect_pending = true;
        self.connect_pending = false;
        self.status.state = "disconnecting".to_string();
    }

    /// Simulates or reports connection completed.
    pub fn mark_connected(&mut self, server: &str) {
        self.connected = true;
        self.connect_pending = false;
        self.disconnect_pending = false;
        self.status = VpnStatusInfo {
            connected: true,
            state: "connected".to_string(),
            reason: String::new(),
            auth_url: String::new(),
            server: server.to_string(),
        };
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64();
        self.connected_since = now;
    }

    /// Simulates or reports connection closed.
    pub fn mark_disconnected(&mut self, reason: &str) {
        self.connected = false;
        self.connect_pending = false;
        self.disconnect_pending = false;
        self.connected_since = 0.0;
        self.status = VpnStatusInfo {
            connected: false,
            state: "disconnected".to_string(),
            reason: reason.to_string(),
            auth_url: String::new(),
            server: String::new(),
        };

        if !self.pending_switch_provider.is_empty() {
            let next_id = std::mem::take(&mut self.pending_switch_provider);
            self.selected_provider_id = next_id;
            self.connect();
        }
    }

    /// Parses WireGuard `wg show` output into connection status.
    #[must_use]
    pub fn parse_wireguard_status(output: &str) -> (bool, Option<String>) {
        let has_endpoint = output.contains("endpoint:");
        let mut endpoint = None;
        for line in output.lines() {
            if let Some(rest) = line.trim().strip_prefix("endpoint:") {
                endpoint = Some(rest.trim().to_string());
                break;
            }
        }
        (has_endpoint, endpoint)
    }

    /// Parses Cloudflare WARP CLI output: `warp-cli status`.
    #[must_use]
    pub fn parse_warp_status(output: &str) -> (bool, String) {
        let lower = output.to_lowercase();
        let connected = lower.contains("status update: connected");
        let state = if connected {
            "connected".to_string()
        } else if lower.contains("connecting") {
            "connecting".to_string()
        } else {
            "disconnected".to_string()
        };
        (connected, state)
    }

    /// Parses generic interface status from `ip link show <iface>`.
    #[must_use]
    pub fn parse_interface_status(output: &str, iface: &str) -> bool {
        output.contains(iface) && (output.contains("state UP") || output.contains("state UNKNOWN"))
    }
}
