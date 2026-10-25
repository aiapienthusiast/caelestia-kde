//! Bluetooth Service
//! Parity with `shell/services/HotspotSwitch.qml` and BlueZ controls.
//! Manages Bluetooth power state and connected peripheral devices via bluetoothctl.

use std::process::Command;
use tracing::info;

#[derive(Clone, Debug, PartialEq)]
pub struct BluetoothDevice {
    pub mac: String,
    pub name: String,
    pub connected: bool,
    pub battery_pct: Option<u8>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BluetoothState {
    pub powered: bool,
    pub connected_devices: Vec<BluetoothDevice>,
}

impl Default for BluetoothState {
    fn default() -> Self {
        Self {
            powered: false,
            connected_devices: Vec::new(),
        }
    }
}

pub struct BluetoothService {
    pub state: BluetoothState,
}

impl BluetoothService {
    pub fn new() -> Self {
        let mut service = Self {
            state: BluetoothState::default(),
        };
        service.poll();
        service
    }

    /// Refresh bluetooth power and connected devices
    pub fn poll(&mut self) {
        if let Ok(output) = Command::new("bluetoothctl").arg("show").output() {
            if output.status.success() {
                let text = String::from_utf8_lossy(&output.stdout);
                self.state.powered = Self::parse_show_output(&text);
            }
        }

        if self.state.powered {
            if let Ok(output) = Command::new("bluetoothctl")
                .args(["devices", "Connected"])
                .output()
            {
                if output.status.success() {
                    let text = String::from_utf8_lossy(&output.stdout);
                    self.state.connected_devices = Self::parse_devices_connected(&text);
                }
            }
        } else {
            self.state.connected_devices.clear();
        }
    }

    /// Parse `bluetoothctl show` output for Powered state
    pub fn parse_show_output(output: &str) -> bool {
        for line in output.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("Powered:") {
                return trimmed.ends_with("yes");
            }
        }
        false
    }

    /// Parse `bluetoothctl devices Connected` output
    /// e.g. "Device F4:4E:FD:04:10:2E Boltt-BS1200"
    pub fn parse_devices_connected(output: &str) -> Vec<BluetoothDevice> {
        let mut devices = Vec::new();

        for line in output.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 && parts[0] == "Device" {
                let mac = parts[1].to_string();
                let name = parts[2..].join(" ");
                devices.push(BluetoothDevice {
                    mac,
                    name,
                    connected: true,
                    battery_pct: None,
                });
            }
        }

        devices
    }

    /// Toggle Bluetooth radio power
    pub fn toggle_power(&mut self) {
        let target = if self.state.powered { "off" } else { "on" };
        self.state.powered = !self.state.powered;
        let _ = Command::new("bluetoothctl")
            .args(["power", target])
            .spawn();
    }

    /// Connect to a device by MAC address
    pub fn connect(&mut self, mac: &str) {
        info!("Connecting Bluetooth device: {}", mac);
        let _ = Command::new("bluetoothctl")
            .args(["connect", mac])
            .spawn();
    }

    /// Disconnect a device by MAC address
    pub fn disconnect(&mut self, mac: &str) {
        info!("Disconnecting Bluetooth device: {}", mac);
        let _ = Command::new("bluetoothctl")
            .args(["disconnect", mac])
            .spawn();
    }
}
