//! Bluetooth Auto-Reconnect Daemon
//! Parity with `shell/modules/BluetoothReconnect.qml`.

use std::collections::HashSet;

#[derive(Debug, Clone, Default)]
pub struct BluetoothReconnectCoordinator {
    attempted_addresses: HashSet<String>,
}

impl BluetoothReconnectCoordinator {
    pub fn new() -> Self {
        Self {
            attempted_addresses: HashSet::new(),
        }
    }

    /// Evaluates discovered devices against auto-reconnect whitelist and returns addresses to connect.
    pub fn check_and_reconnect(
        &mut self,
        adapter_enabled: bool,
        available_devices: &[(String, String, bool)], // (address, name, connected)
        auto_reconnect_list: &[String],
    ) -> Vec<String> {
        if !adapter_enabled || auto_reconnect_list.is_empty() {
            return Vec::new();
        }

        let mut to_reconnect = Vec::new();
        for (address, _name, connected) in available_devices {
            if auto_reconnect_list.iter().any(|a| a.eq_ignore_ascii_case(address)) {
                if !self.attempted_addresses.contains(address) {
                    self.attempted_addresses.insert(address.clone());
                    if !connected {
                        to_reconnect.push(address.clone());
                    }
                }
            }
        }

        to_reconnect
    }

    /// Resets attempted addresses cache on adapter toggle.
    pub fn reset_attempts(&mut self) {
        self.attempted_addresses.clear();
    }
}
