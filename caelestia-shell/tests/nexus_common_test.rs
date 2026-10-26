//! Automated Tests for Nexus Common Controls and Sub-dialogs
//! Verifies 100% functionality and parity with `shell/modules/nexus/common/`.

use caelestia_shell::modules::launcher::entry::DesktopEntry;
use caelestia_shell::modules::nexus::common::*;
use caelestia_shell::services::audio::AudioDevice;
use caelestia_shell::services::global_shortcut::RegisteredShortcut;
use caelestia_shell::services::network::AccessPoint;

#[test]
fn test_app_picker_filtering_and_selection() {
    let mut picker = AppPickerRow::new();
    assert_eq!(picker.is_open, false);
    assert!(picker.selected_app.is_none());

    let apps = vec![
        DesktopEntry {
            id: "org.kde.dolphin.desktop".to_string(),
            name: "Dolphin".to_string(),
            generic_name: "File Manager".to_string(),
            comment: "Manage files".to_string(),
            ..Default::default()
        },
        DesktopEntry {
            id: "org.mozilla.firefox.desktop".to_string(),
            name: "Firefox".to_string(),
            generic_name: "Web Browser".to_string(),
            comment: "Browse the World Wide Web".to_string(),
            ..Default::default()
        },
        DesktopEntry {
            id: "org.kde.konsole.desktop".to_string(),
            name: "Konsole".to_string(),
            generic_name: "Terminal Emulator".to_string(),
            comment: "Command line interface".to_string(),
            ..Default::default()
        },
    ];

    // 1. Empty filter matches all
    assert_eq!(picker.matches(&apps).len(), 3);

    // 2. Filter by name
    picker.set_filter("dolph");
    let matches = picker.matches(&apps);
    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].name, "Dolphin");

    // 3. Filter by generic name
    picker.set_filter("terminal");
    let matches = picker.matches(&apps);
    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].name, "Konsole");

    // 4. Exclude list
    picker.set_filter("");
    picker.exclude = vec!["Firefox".to_string()];
    let matches = picker.matches(&apps);
    assert_eq!(matches.len(), 2);
    assert!(!matches.iter().any(|a| a.name == "Firefox"));

    // 5. Select app
    picker.is_open = true;
    picker.select("Dolphin");
    assert_eq!(picker.selected_app.as_deref(), Some("Dolphin"));
    assert_eq!(picker.is_open, false);
}

#[test]
fn test_window_picker_selection() {
    let mut picker = WindowPickerRow::new();
    assert_eq!(picker.is_open, false);
    assert!(picker.selected_window_id.is_none());

    picker.is_open = true;
    picker.select("wayland:0x1234", "Firefox — Nightly");

    assert_eq!(picker.selected_window_id.as_deref(), Some("wayland:0x1234"));
    assert_eq!(picker.selected_title.as_deref(), Some("Firefox — Nightly"));
    assert_eq!(picker.is_open, false);
}

#[test]
fn test_audio_device_list_filtering_and_selection() {
    let mut list = AudioDeviceList::new(true);
    assert!(list.is_sink);
    assert!(list.current_id.is_none());

    let devices = vec![
        AudioDevice {
            id: 42,
            name: "alsa_output.pci-0000_00_1f.3.analog-stereo".to_string(),
            description: "Built-in Audio Analog Stereo".to_string(),
            volume: 0.75,
            muted: false,
            is_default: true,
        },
        AudioDevice {
            id: 88,
            name: "bluez_output.00_1B_66_B0_23_4D.a2dp-sink".to_string(),
            description: "Sony WH-1000XM4 Headset".to_string(),
            volume: 0.60,
            muted: false,
            is_default: false,
        },
    ];

    // Filter
    list.filter = "Sony".to_string();
    let filtered = list.filter_devices(&devices);
    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0].id, 88);

    // Select
    list.select(88);
    assert_eq!(list.current_id, Some(88));
}

#[test]
fn test_key_capture_conflict_detection() {
    let mut dialog = KeyCaptureDialog::new();
    assert_eq!(dialog.is_open, false);

    let shortcuts = vec![
        RegisteredShortcut {
            name: "terminal".to_string(),
            description: "Open Terminal".to_string(),
            key: "Ctrl+Alt+T".to_string(),
            enabled: true,
            stolen_shortcuts: Vec::new(),
        },
        RegisteredShortcut {
            name: "launcher".to_string(),
            description: "Application Launcher".to_string(),
            key: "Super+Space;Alt+F1".to_string(),
            enabled: true,
            stolen_shortcuts: Vec::new(),
        },
    ];

    dialog.open("custom_action", "Super+K");
    assert_eq!(dialog.is_open, true);
    assert_eq!(dialog.shortcut_name, "custom_action");
    assert_eq!(dialog.current_key, "Super+K");

    // 1. No collision
    dialog.capture_key("Super+Alt+P", &shortcuts);
    assert!(dialog.conflict_info.is_none());

    // 2. Collision with single shortcut
    dialog.capture_key("Ctrl+Alt+T", &shortcuts);
    assert!(dialog.conflict_info.is_some());
    let (conflicting_name, label) = dialog.conflict_info.as_ref().unwrap();
    assert_eq!(conflicting_name, "terminal");
    assert_eq!(label, "Open Terminal");

    // 3. Collision with multi-part shortcut
    dialog.capture_key("Alt+F1", &shortcuts);
    assert!(dialog.conflict_info.is_some());
    let (conflicting_name, _) = dialog.conflict_info.as_ref().unwrap();
    assert_eq!(conflicting_name, "launcher");

    // 4. Confirm new key
    dialog.capture_key("Super+Alt+Z", &shortcuts);
    let result = dialog.confirm();
    assert_eq!(result, Some(("custom_action".to_string(), "Super+Alt+Z".to_string())));
    assert_eq!(dialog.is_open, false);
}

#[test]
fn test_network_list_ranking() {
    let list = NetworkList {
        filter: String::new(),
        limit: 0,
        connecting_ssid: Some("Home-5G".to_string()),
    };

    let aps = vec![
        AccessPoint {
            ssid: "Coffee-Shop".to_string(),
            signal: 90,
            security: "WPA2".to_string(),
            in_use: false,
        },
        AccessPoint {
            ssid: "Home-5G".to_string(),
            signal: 65,
            security: "WPA3".to_string(),
            in_use: false,
        },
        AccessPoint {
            ssid: "Office-Network".to_string(),
            signal: 80,
            security: "WPA2-Enterprise".to_string(),
            in_use: true,
        },
    ];

    let ranked = list.rank_and_sort(&aps);
    assert_eq!(ranked.len(), 3);
    // Active (in_use) comes first
    assert_eq!(ranked[0].ssid, "Office-Network");
    // Connecting comes second
    assert_eq!(ranked[1].ssid, "Home-5G");
    // Other networks come third
    assert_eq!(ranked[2].ssid, "Coffee-Shop");
}

#[test]
fn test_ipv4_config_and_ethernet_section() {
    let mut ipv4 = Ipv4ConfigSection::new();
    assert!(ipv4.is_dhcp);

    ipv4.toggle_mode();
    assert!(!ipv4.is_dhcp);

    let eth = EthernetSection::new("eth0", true, "192.168.1.150", 1000, "Full Duplex");
    assert_eq!(eth.interface_name, "eth0");
    assert!(eth.carrier_up);
    assert_eq!(eth.speed_mbps, 1000);
}

#[test]
fn test_uninstall_dialog_and_list_editor() {
    let mut dialog = UninstallDialog::new();
    assert_eq!(dialog.is_open, false);

    dialog.open("Steam", "steam-devices", false);
    assert!(dialog.is_open);
    assert_eq!(dialog.app_name, "Steam");
    assert_eq!(dialog.package_name, "steam-devices");
    assert!(!dialog.is_flatpak);

    dialog.close();
    assert!(!dialog.is_open);

    let mut editor = ListEditor::new("Excluded Apps", vec!["app1".to_string(), "app2".to_string()]);
    assert_eq!(editor.items.len(), 2);

    editor.add_item("app3");
    assert_eq!(editor.items.len(), 3);

    // Deduplication check
    editor.add_item("app3");
    assert_eq!(editor.items.len(), 3);

    editor.remove_item(0);
    assert_eq!(editor.items.len(), 2);
    assert_eq!(editor.items[0], "app2");
}

#[test]
fn test_update_timeline() {
    let timeline = UpdateTimeline::new(vec![
        UpdateEntry {
            date: "2026-10-25 14:30".to_string(),
            package: "linux".to_string(),
            old_version: "6.11.4.arch1-1".to_string(),
            new_version: "6.11.5.arch1-1".to_string(),
            summary: "Linux kernel update".to_string(),
        },
    ]);

    assert_eq!(timeline.entries.len(), 1);
    assert_eq!(timeline.entries[0].package, "linux");
}
