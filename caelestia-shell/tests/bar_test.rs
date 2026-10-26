use caelestia_shell::modules::bar::*;
use caelestia_shell::services::audio::AudioService;
use caelestia_shell::services::bluetooth::BluetoothService;
use caelestia_shell::services::kwin::{KWinService, VirtualWorkspace, WindowItem};
use caelestia_shell::services::network::{NetworkKind, NetworkService};

#[test]
fn test_bar_widgets_registry_and_zone_filtering() {
    let entries = BarWidgets::default_entries();

    let left = BarWidgets::zone_entries(&entries, BarZone::Left);
    assert!(left.contains(&BarWidgetId::Logo));
    assert!(left.contains(&BarWidgetId::Workspaces));

    let middle = BarWidgets::zone_entries(&entries, BarZone::Middle);
    assert_eq!(middle, vec![BarWidgetId::Dock]);

    let right = BarWidgets::zone_entries(&entries, BarZone::Right);
    assert!(right.contains(&BarWidgetId::Clock));
    assert!(right.contains(&BarWidgetId::StatusIcons));
    assert!(right.contains(&BarWidgetId::Power));
}

#[test]
fn test_workspaces_pill_model_computation() {
    let mut kwin = KWinService::new();
    kwin.state.workspaces = vec![
        VirtualWorkspace {
            id: 0,
            uuid: "ws-0".to_string(),
            name: "1".to_string(),
            active: true,
        },
        VirtualWorkspace {
            id: 1,
            uuid: "ws-1".to_string(),
            name: "2".to_string(),
            active: false,
        },
        VirtualWorkspace {
            id: 2,
            uuid: "ws-2".to_string(),
            name: "3".to_string(),
            active: false,
        },
    ];
    kwin.state.active_index = 0;

    // Window on workspace 2 (index 1)
    kwin.state.window_list = vec![
        WindowItem {
            address: "win-1".to_string(),
            title: "Editor".to_string(),
            app_id: "code".to_string(),
            desktops: vec!["1".to_string()],
            ..Default::default()
        },
    ];

    let pills = WorkspacesModel::compute_pills(&kwin, 6, true, None);
    assert_eq!(pills.len(), 3);
    assert!(pills[0].active);
    assert!(!pills[0].occupied);

    assert!(!pills[1].active);
    assert!(pills[1].occupied); // Occupied by Editor

    assert!(!pills[2].active);
    assert!(!pills[2].occupied);

    // If show_unoccupied is false, unoccupied workspace 3 should be omitted
    let filtered_pills = WorkspacesModel::compute_pills(&kwin, 6, false, None);
    assert_eq!(filtered_pills.len(), 2);
    assert_eq!(filtered_pills[0].name, "1"); // active
    assert_eq!(filtered_pills[1].name, "2"); // occupied
}

#[test]
fn test_status_icons_model_snapshots() {
    let mut audio = AudioService::new();
    let mut network = NetworkService::new_mock(true, Some("Home_WiFi".to_string()), Vec::new());
    let mut bluetooth = BluetoothService::new();

    // 1. Audio Muted
    audio.state.sink_volume = 0.8;
    audio.state.sink_muted = true;

    // 2. Network Wi-Fi Connected
    network.state.kind = NetworkKind::Wifi;
    network.state.wifi_enabled = true;
    network.state.active_ssid = Some("Home_WiFi".to_string());

    // 3. Bluetooth Connected
    bluetooth.state.powered = true;
    bluetooth.state.connected_devices = vec![caelestia_shell::services::bluetooth::BluetoothDevice {
        mac: "AA:BB:CC:DD:EE:FF".to_string(),
        name: "Headphones".to_string(),
        connected: true,
        battery_pct: Some(85),
    }];

    let snap = StatusIconsModel::snapshot(&audio, &network, &bluetooth);
    assert_eq!(snap.volume_icon, "volume_off");
    assert!(snap.volume_muted);
    assert_eq!(snap.volume_pct, 80);

    assert_eq!(snap.network_icon, "wifi");
    assert_eq!(snap.network_name, "Home_WiFi");

    assert_eq!(snap.bluetooth_icon, "bluetooth_connected");
    assert_eq!(snap.bluetooth_connected_count, 1);

    // Unmute audio and switch to Ethernet
    audio.state.sink_muted = false;
    audio.state.sink_volume = 0.45;
    network.state.kind = NetworkKind::Ethernet;

    let snap2 = StatusIconsModel::snapshot(&audio, &network, &bluetooth);
    assert_eq!(snap2.volume_icon, "volume_down");
    assert!(!snap2.volume_muted);
    assert_eq!(snap2.network_icon, "lan");
    assert_eq!(snap2.network_name, "Ethernet");
}

#[test]
fn test_bar_clock_formatting() {
    let clock_24h = BarClockModel::current(true, false);
    assert_eq!(clock_24h.time_str.len(), 5); // "HH:mm"
    assert!(clock_24h.time_str.contains(':'));
    assert!(!clock_24h.date_str.is_empty());

    let clock_12h = BarClockModel::current(false, false);
    assert!(clock_12h.time_str.contains("AM") || clock_12h.time_str.contains("PM"));

    let clock_seconds = BarClockModel::current(true, true);
    assert_eq!(clock_seconds.time_str.len(), 8); // "HH:mm:ss"
}

#[test]
fn test_bar_dock_active_window_snapshot() {
    let win = WindowItem {
        address: "w1".to_string(),
        title: "Rust Analyzer - main.rs".to_string(),
        app_id: "code".to_string(),
        ..Default::default()
    };

    let snap = BarDockModel::from_active_window(Some(&win));
    assert!(snap.has_window);
    assert_eq!(snap.title, "Rust Analyzer - main.rs");
    assert_eq!(snap.app_id, "code");

    let empty_snap = BarDockModel::from_active_window(None);
    assert!(!empty_snap.has_window);
    assert!(empty_snap.title.is_empty());
}

#[test]
fn test_bar_popouts_state_toggle_and_close() {
    let mut popouts = BarPopoutsState::default();
    assert!(!popouts.is_open);
    assert_eq!(popouts.current, None);

    // Open audio popout
    popouts.open(PopoutType::Audio, 450.0);
    assert!(popouts.is_open);
    assert_eq!(popouts.current, Some(PopoutType::Audio));
    assert_eq!(popouts.current_center, 450.0);

    // Toggle same popout closes it
    popouts.toggle(PopoutType::Audio, 450.0);
    assert!(!popouts.is_open);
    assert_eq!(popouts.current, None);

    // Toggle different popout opens that one
    popouts.toggle(PopoutType::Calendar, 800.0);
    assert!(popouts.is_open);
    assert_eq!(popouts.current, Some(PopoutType::Calendar));
    assert_eq!(popouts.current_center, 800.0);

    // Explicit close
    popouts.close();
    assert!(!popouts.is_open);
}

#[test]
fn test_all_popout_types_render() {
    use caelestia_shell::infrastructure::theme::CaelestiaColours;

    let c = CaelestiaColours::default();

    let all_popouts = [
        PopoutType::Audio,
        PopoutType::Battery,
        PopoutType::Bluetooth,
        PopoutType::Calendar,
        PopoutType::DockContext,
        PopoutType::Github,
        PopoutType::Greeter,
        PopoutType::KbLayout,
        PopoutType::KeepAwake,
        PopoutType::LockStatus,
        PopoutType::Media,
        PopoutType::Network,
        PopoutType::NightLight,
        PopoutType::Notes,
        PopoutType::Notifications,
        PopoutType::Performance,
        PopoutType::PeripheralBattery,
        PopoutType::Power,
        PopoutType::Privacy,
        PopoutType::ScreenCapture,
        PopoutType::Shortcuts,
        PopoutType::TrayMenu,
        PopoutType::Updates,
        PopoutType::Weather,
        PopoutType::WirelessPassword,
    ];

    assert_eq!(all_popouts.len(), 25);

    for popout in all_popouts {
        let _div = BarPopouts::render(popout, 500.0, &c);
    }
}

#[test]
fn test_tray_registry_service() {
    use caelestia_shell::services::tray_registry::{TrayItem, TrayItemCategory, TrayItemStatus, TrayRegistryService};

    let mut tray = TrayRegistryService::new();
    assert_eq!(tray.state.items.len(), 0);

    let item = TrayItem {
        id: "discord".to_string(),
        title: "Discord".to_string(),
        icon_name: String::new(),
        icon_theme_path: None,
        category: TrayItemCategory::Communications,
        status: TrayItemStatus::Active,
        service_name: "org.kde.StatusNotifierItem-1234-1".to_string(),
        object_path: "/StatusNotifierItem".to_string(),
        menu_path: Some("/MenuBar".to_string()),
        tooltip: Some("Discord - Chatting".to_string()),
        is_collapsed: false,
    };

    tray.register_item(item);
    assert_eq!(tray.state.items.len(), 1);

    // Verify fallback icon mapping from trayicons.js
    let resolved_icon = tray.resolve_icon("discord", "");
    assert_eq!(resolved_icon, "discord");

    let default_icon = tray.resolve_icon("unknown-app", "");
    assert_eq!(default_icon, "application-x-executable");

    // Unregister
    tray.unregister_item("discord");
    assert_eq!(tray.state.items.len(), 0);
}

#[test]
fn test_all_bar_components_render() {
    use caelestia_shell::infrastructure::theme::CaelestiaColours;
    use caelestia_shell::modules::bar::components::*;

    let c = CaelestiaColours::default();

    // 1. ClockWidget
    let clock_snap = ClockSnapshot {
        time_str: "12:34".to_string(),
        date_str: "Sun, Oct 25".to_string(),
        hours: 12,
        minutes: 34,
        seconds: 0,
        hour_str: "12".to_string(),
        minute_str: "34".to_string(),
        second_str: "00".to_string(),
        am_pm_str: "pm".to_string(),
        day_of_week: "Sun".to_string(),
        day_of_month: "25".to_string(),
    };
    let _ = ClockWidget::render(&clock_snap, &c);

    // 2. DockWidget
    let dock_snap = DockSnapshot {
        title: "Editor".to_string(),
        app_id: "code".to_string(),
        has_window: true,
    };
    let _ = DockWidget::render(&dock_snap, &c);

    // 3. GithubActivityWidget
    let _ = GithubActivityWidget::render(42, &c);

    // 4. GithubStore
    let store = GithubStore::new();
    let snap = store.snapshot();
    assert_eq!(snap.total, 0);

    // 5. GreeterWidget
    let _ = GreeterWidget::render("Alice", &c);

    // 6. KbLayoutIndicatorWidget
    let _ = KbLayoutIndicatorWidget::render("US", &c);

    // 7. KeepAwakeWidget
    let _ = KeepAwakeWidget::render(true, &c);

    // 8. MediaWidget
    let _ = MediaWidget::render("Track", true, 0.5, &c);

    // 9. NotesWidget
    let _ = NotesWidget::render(&c);

    // 10. NotificationsIndicatorWidget
    let _ = NotificationsIndicatorWidget::render(5, &c);

    // 11. OsIconWidget
    let _ = OsIconWidget::render(&c);

    // 12. PerformanceWidget
    let _ = PerformanceWidget::render(15, 45, &c);

    // 13. PowerWidget
    let _ = PowerWidget::render(&c);

    // 14. ScreenCaptureWidget
    let _ = ScreenCaptureWidget::render(false, &c);

    // 15. ShortcutsWidget
    let _ = ShortcutsWidget::render(&c);

    // 16. ShowDesktopWidget
    let _ = ShowDesktopWidget::render(&c);

    // 17. StatusIconsWidget
    let status_snap = BarStatusSnapshot {
        volume_icon: "volume_up",
        volume_pct: 65,
        volume_muted: false,
        network_icon: "wifi",
        network_name: "Home".to_string(),
        bluetooth_icon: "bluetooth",
        bluetooth_connected_count: 1,
        battery_pct: 90,
        battery_charging: true,
    };
    let _ = StatusIconsWidget::render(&status_snap, &c);

    // 18. TrayWidget
    let _ = TrayWidget::render(&[], &c);

    // 19. TrayItemWidget
    let _ = TrayItemWidget::render("discord", "Discord", "discord", &c);

    // 20. UpdateIndicatorWidget
    let _ = UpdateIndicatorWidget::render(12, &c);

    // 21. WeatherWidget
    let _ = WeatherWidget::render(21, "sunny", &c);

    // Performance submodules (7)
    let _ = performance::PerfBattery::render(80, true, &c);
    let _ = performance::PerfCpu::render(25, &c);
    let _ = performance::PerfGpu::render(40, &c);
    let _ = performance::PerfMemory::render(8.0, 16.0, &c);
    let _ = performance::PerfNetwork::render(1250, 320, &c);
    let _ = performance::PerfStat::render("IO", "12 MB/s", &c);
    let _ = performance::PerfStorage::render(120, &c);

    // Status submodules (3)
    let _ = status::BatteryStatusWidget::render(85, false, &c);
    let _ = status::BluetoothStatusWidget::render("bluetooth", 1, &c);
    let _ = status::LockStatusWidget::render(true, false, &c);

    // Workspaces submodules (6)
    let _ = workspaces::ActiveIndicatorWidget::render(0, 10.0, true, &c);
    let _ = workspaces::GapMarkersWidget::render(&[10.0, 30.0], true, &c);
    let _ = workspaces::OccupiedBgWidget::render(&[(0.0, 50.0)], true, &c);
    let _ = workspaces::SpecialWorkspacesWidget::render(&["special:scratch".to_string()], "special:scratch", &c);
    let pill = WorkspacePill {
        index: 0,
        id: 1,
        name: "1".to_string(),
        active: true,
        occupied: true,
    };
    let _ = workspaces::WorkspacePillWidget::render(&pill, &c);
    let _ = workspaces::WorkspacesStripWidget::render(&[pill], true, &c);
}

