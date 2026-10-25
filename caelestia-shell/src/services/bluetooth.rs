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
        if !crate::infrastructure::is_test_environment() {
            service.poll();
        }
        service
    }

    /// Refresh bluetooth power and connected devices
    pub fn poll(&mut self) {
        if crate::infrastructure::is_test_environment() {
            return;
        }
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

    /// Returns command specification for toggling bluetooth radio power
    pub fn command_for_toggle_power(powered: bool) -> (&'static str, [&'static str; 2]) {
        ("bluetoothctl", ["power", if powered { "on" } else { "off" }])
    }

    /// Returns command specification for connecting to a bluetooth device
    pub fn command_for_connect(mac: &str) -> (&'static str, Vec<String>) {
        ("bluetoothctl", vec!["connect".to_string(), mac.to_string()])
    }

    /// Returns command specification for disconnecting a bluetooth device
    pub fn command_for_disconnect(mac: &str) -> (&'static str, Vec<String>) {
        ("bluetoothctl", vec!["disconnect".to_string(), mac.to_string()])
    }

    /// Toggle Bluetooth radio power
    pub fn toggle_power(&mut self) {
        let next_powered = !self.state.powered;
        self.state.powered = next_powered;
        if !crate::infrastructure::is_test_environment() {
            let (prog, args) = Self::command_for_toggle_power(next_powered);
            let _ = Command::new(prog).args(args).spawn();
        }
    }

    /// Connect to a device by MAC address
    pub fn connect(&mut self, mac: &str) {
        info!("Connecting Bluetooth device: {}", mac);
        if !crate::infrastructure::is_test_environment() {
            let (prog, args) = Self::command_for_connect(mac);
            let _ = Command::new(prog).args(args).spawn();
        }
    }

    /// Disconnect a device by MAC address
    pub fn disconnect(&mut self, mac: &str) {
        info!("Disconnecting Bluetooth device: {}", mac);
        if !crate::infrastructure::is_test_environment() {
            let (prog, args) = Self::command_for_disconnect(mac);
            let _ = Command::new(prog).args(args).spawn();
        }
    }
}
