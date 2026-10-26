//! Network API Bridge for Plugins and Shell Subsystems
//! Parity with `shell/services/api/NetworkApi.qml`.

use crate::services::network::{AccessPoint, NetworkKind, NetworkService, NetworkState};
use crate::services::requests::RequestsService;
use crate::services::vpn::{VpnProvider, VpnService, VpnStatusInfo};
use std::sync::{Arc, Mutex, OnceLock};

/// Network API exposing NetworkManager state, HTTP clients, and VPN tunnels
#[derive(Clone)]
pub struct NetworkApi {
    pub network: Arc<Mutex<NetworkService>>,
    pub requests: Arc<RequestsService>,
    pub vpn: Arc<Mutex<VpnService>>,
}

impl Default for NetworkApi {
    fn default() -> Self {
        Self::new()
    }
}

impl NetworkApi {
    pub fn new() -> Self {
        Self {
            network: Arc::new(Mutex::new(NetworkService::new())),
            requests: Arc::new(RequestsService::new()),
            vpn: Arc::new(Mutex::new(VpnService::new())),
        }
    }

    pub fn global() -> &'static Self {
        static INSTANCE: OnceLock<NetworkApi> = OnceLock::new();
        INSTANCE.get_or_init(Self::new)
    }

    /// Read current network connection state
    pub fn state(&self) -> NetworkState {
        self.network.lock().map_or_else(
            |_| NetworkState::default(),
            |n| n.state.clone(),
        )
    }

    /// Check if primary network connection is established
    pub fn is_connected(&self) -> bool {
        self.network.lock().map_or(false, |n| n.state.connected)
    }

    /// Active network type (Wifi, Ethernet, Disconnected)
    pub fn kind(&self) -> NetworkKind {
        self.network.lock().map_or(NetworkKind::Disconnected, |n| n.state.kind)
    }

    /// List discovered Wi-Fi access points
    pub fn access_points(&self) -> Vec<AccessPoint> {
        self.network.lock().map_or(Vec::new(), |n| n.state.access_points.clone())
    }

    /// Active VPN status
    pub fn vpn_status(&self) -> VpnStatusInfo {
        self.vpn.lock().map_or_else(
            |_| VpnStatusInfo::default(),
            |v| v.status.clone(),
        )
    }

    /// Available VPN providers
    pub fn vpn_providers(&self) -> Vec<VpnProvider> {
        self.vpn.lock().map_or(Vec::new(), |v| v.providers.clone())
    }

    /// Trigger HTTP GET request
    pub fn get(&self, url: &str) -> Result<String, String> {
        self.requests.get(url, None, None).map(|resp| resp.body)
    }
}
