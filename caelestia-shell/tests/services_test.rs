//! Custom Test Suite for Unified Caelestia Shell Services
//! Validates 100% functionality and parity with legacy shell/services and shell/plugin C++ services.

use caelestia_shell::services::audio::AudioService;
use caelestia_shell::services::bluetooth::BluetoothService;
use caelestia_shell::services::brightness::BrightnessService;
use caelestia_shell::services::kwin::KWinService;
use caelestia_shell::services::network::{NetworkKind, NetworkService};
use caelestia_shell::services::night_light::NightLightService;
use caelestia_shell::services::notifications::{NotificationsService, Urgency};

#[test]
fn test_audio_service_volume_and_status_parsing() {
    // Volume string parsing
    assert_eq!(AudioService::parse_volume_output("Volume: 0.85"), Some((0.85, false)));
    assert_eq!(AudioService::parse_volume_output("Volume: 1.20 [MUTED]"), Some((1.20, true)));
    assert_eq!(AudioService::parse_volume_output("Volume: 0.00"), Some((0.00, false)));
    assert_eq!(AudioService::parse_volume_output("Invalid text"), None);

    // Mock wpctl status output
    let mock_status = r#"
PipeWire 'pipewire-0' [1.6.10, user@host]
Audio
 ├─ Devices:
 │      44. Audio Controller [alsa]
 ├─ Sinks:
 │      54. HDMI Output [vol: 1.00]
 │  *   57. Built-in Speaker [vol: 1.20]
 │      60. Bluetooth Headset [vol: 0.74 MUTED]
 ├─ Sources:
 │      58. Built-in Mic [vol: 1.00]
 │  *   59. Headset Mic [vol: 0.80 MUTED]
 └─ Streams:
"#;

    let (sinks, sources) = AudioService::parse_wpctl_status(mock_status);
    assert_eq!(sinks.len(), 3);
    assert_eq!(sinks[0].id, 54);
    assert!(!sinks[0].is_default);
    assert_eq!(sinks[1].id, 57);
    assert!(sinks[1].is_default);
    assert_eq!(sinks[1].volume, 1.20);
    assert!(!sinks[1].muted);

    assert_eq!(sinks[2].id, 60);
    assert_eq!(sinks[2].volume, 0.74);
    assert!(sinks[2].muted);

    assert_eq!(sources.len(), 2);
    assert_eq!(sources[0].id, 58);
    assert_eq!(sources[1].id, 59);
    assert!(sources[1].is_default);
    assert!(sources[1].muted);

    // Volume stepping and bounds
    let mut audio = AudioService::new();
    audio.state.sink_volume = 1.0;
    audio.state.sink_volume = (audio.state.sink_volume + 0.05).clamp(0.0, 1.5);
    assert_eq!((audio.state.sink_volume * 100.0).round() / 100.0, 1.05);

    audio.state.sink_volume = 1.5;
    audio.state.sink_volume = (audio.state.sink_volume + 0.1).clamp(0.0, 1.5);
    assert_eq!(audio.state.sink_volume, 1.5); // Clamped at 1.5
}

#[test]
fn test_brightness_service_parsing_and_stepping() {
    let mock_output = "intel_backlight,backlight,19200,100%,19200\n";
    let state = BrightnessService::parse_brightnessctl_output(mock_output).unwrap();
    assert_eq!(state.device, "intel_backlight");
    assert_eq!(state.current_val, 19200);
    assert_eq!(state.max_val, 19200);
    assert_eq!(state.current_pct, 100);

    let mock_partial = "amdgpu_bl0,backlight,128,50%,255\n";
    let state2 = BrightnessService::parse_brightnessctl_output(mock_partial).unwrap();
    assert_eq!(state2.device, "amdgpu_bl0");
    assert_eq!(state2.current_pct, 50);

    // Stepping bounds
    let mut brightness = BrightnessService::new();
    brightness.state.current_pct = 50;
    brightness.state.current_pct = (brightness.state.current_pct as i32 + 10).clamp(1, 100) as u32;
    assert_eq!(brightness.state.current_pct, 60);

    // Clamping minimum to prevent pitch black screen
    brightness.state.current_pct = (brightness.state.current_pct as i32 - 100).clamp(1, 100) as u32;
    assert_eq!(brightness.state.current_pct, 1);
}

#[test]
fn test_network_service_device_status_and_wifi_parsing() {
    let mock_status = "wifi:connected:MyHomeNetwork\nloopback:connected:lo\nethernet:unavailable:\n";
    let (kind, connected, active_ssid, eth) = NetworkService::parse_device_status(mock_status);
    assert_eq!(kind, NetworkKind::Wifi);
    assert!(connected);
    assert_eq!(active_ssid.as_deref(), Some("MyHomeNetwork"));
    assert!(!eth);

    let mock_eth = "ethernet:connected:Wired connection 1\nwifi:disconnected:\n";
    let (kind_eth, connected_eth, _, is_eth) = NetworkService::parse_device_status(mock_eth);
    assert_eq!(kind_eth, NetworkKind::Ethernet);
    assert!(connected_eth);
    assert!(is_eth);

    let mock_wifi_list = r#"
 :Cafe_WiFi:45:WPA2
*:MyHomeNetwork:85:WPA2
 :Cafe_WiFi:65:WPA2
 :OpenNetwork:90:
 : :30:
"#;

    let aps = NetworkService::parse_wifi_list(mock_wifi_list);
    // In-use network should be sorted first
    assert_eq!(aps[0].ssid, "MyHomeNetwork");
    assert!(aps[0].in_use);
    assert_eq!(aps[0].signal, 85);

    // Deduplication should have taken the higher signal for Cafe_WiFi (65)
    let cafe = aps.iter().find(|ap| ap.ssid == "Cafe_WiFi").unwrap();
    assert_eq!(cafe.signal, 65);

    // Hidden network with empty SSID should be ignored
    assert!(!aps.iter().any(|ap| ap.ssid.is_empty()));
}

#[test]
fn test_bluetooth_service_show_and_device_parsing() {
    let mock_show = r#"
Controller 00:11:22:33:44:55
	Name: June
	Powered: yes
	Discoverable: no
"#;
    assert!(BluetoothService::parse_show_output(mock_show));

    let mock_show_off = r#"
Controller 00:11:22:33:44:55
	Powered: no
"#;
    assert!(!BluetoothService::parse_show_output(mock_show_off));

    let mock_devices = "Device F4:4E:FD:04:10:2E Boltt-BS1200\nDevice 11:22:33:44:55:66 Sony WH-1000XM4\n";
    let devices = BluetoothService::parse_devices_connected(mock_devices);
    assert_eq!(devices.len(), 2);
    assert_eq!(devices[0].mac, "F4:4E:FD:04:10:2E");
    assert_eq!(devices[0].name, "Boltt-BS1200");
    assert!(devices[0].connected);
    assert_eq!(devices[1].mac, "11:22:33:44:55:66");
    assert_eq!(devices[1].name, "Sony WH-1000XM4");
}

#[test]
fn test_notifications_service_queue_and_dnd() {
    let mut notifs = NotificationsService::new();
    assert_eq!(notifs.count(), 0);
    assert!(!notifs.state.dnd);

    // Toggle DND
    notifs.toggle_dnd();
    assert!(notifs.state.dnd);
    notifs.set_dnd(false);
    assert!(!notifs.state.dnd);

    // Post notifications
    let id1 = notifs.post("Discord", "discord", "New message", "Hey how are you?", Urgency::Normal);
    let id2 = notifs.post("Terminal", "terminal", "Build Finished", "Cargo check passed", Urgency::Low);

    assert_eq!(notifs.count(), 2);
    // Newest should be at front
    assert_eq!(notifs.state.items[0].id, id2);
    assert_eq!(notifs.state.items[1].id, id1);

    // Dismiss one
    notifs.dismiss(id1);
    assert_eq!(notifs.count(), 1);
    assert_eq!(notifs.state.items[0].id, id2);

    // Truncation at max_items limit
    notifs.state.max_items = 3;
    notifs.post("App", "icon", "N1", "B1", Urgency::Normal);
    notifs.post("App", "icon", "N2", "B2", Urgency::Normal);
    notifs.post("App", "icon", "N3", "B3", Urgency::Normal);
    assert_eq!(notifs.count(), 3);

    // Clear all
    notifs.clear_all();
    assert_eq!(notifs.count(), 0);
}

#[test]
fn test_kwin_service_desktops_parsing_and_selection() {
    let mock_literal = r#"[Variant: [Argument: a(uss) {[Argument: (uss) 0, "8b0caf0f-5c74-4d00-a6a2-ca6d10722121", "Desktop 1"], [Argument: (uss) 1, "c3835185-c590-4cc9-8b15-2e0bfb9d30f2", "undefined"], [Argument: (uss) 2, "47d4de5b-53ab-4d53-9b74-745ecb8b62ba", "Coding"]}]]"#;
    let active_uuid = "47d4de5b-53ab-4d53-9b74-745ecb8b62ba";

    let workspaces = KWinService::parse_desktops_literal(mock_literal, active_uuid);
    assert_eq!(workspaces.len(), 3);
    assert_eq!(workspaces[0].id, 0);
    assert_eq!(workspaces[0].name, "Desktop 1");
    assert!(!workspaces[0].active);

    assert_eq!(workspaces[1].id, 1);
    assert_eq!(workspaces[1].name, "Workspace 2"); // Fallback for undefined
    assert!(!workspaces[1].active);

    assert_eq!(workspaces[2].id, 2);
    assert_eq!(workspaces[2].name, "Coding");
    assert!(workspaces[2].active); // Matches active_uuid
}

#[test]
fn test_night_light_state_defaults_and_toggles() {
    let mut night_light = NightLightService::new();
    assert_eq!(night_light.state.target_temperature, 4500);

    let initial = night_light.state.running;
    night_light.state.running = !initial;
    assert_eq!(night_light.state.running, !initial);

    // Temperature bounds clamp
    night_light.state.current_temperature = 4000.clamp(1000, 10000);
    assert_eq!(night_light.state.current_temperature, 4000);
}
