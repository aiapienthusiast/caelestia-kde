use caelestia_shell::infrastructure::theme::CaelestiaColours;
use caelestia_shell::modules::battery_monitor::{BatteryMonitor, BatteryToastEvent};
use caelestia_shell::modules::bluetooth_reconnect::BluetoothReconnectCoordinator;
use caelestia_shell::modules::gsf_loader::GsfFontLoader;
use caelestia_shell::modules::plugins::{PluginManifest, PluginManager};
use caelestia_shell::modules::screen_corners::{ScreenCorner, ScreenCornersConfig, ScreenCornersCoordinator};
use caelestia_shell::modules::service_loader::{ServiceLoader, ShellServiceId};
use caelestia_shell::modules::shortcuts::GlobalShortcutsModule;
use caelestia_shell::modules::whatsnew::{WhatsNewModel, WhatsNewView};
use caelestia_shell::modules::windowinfo::{
    WindowInfoButtons, WindowInfoDetails, WindowInfoPreview, WindowInfoView,
};
use caelestia_shell::services::kwin::WindowItem;

#[test]
fn test_gsf_font_loader() {
    let loader = GsfFontLoader::new();
    let _ = loader.is_system_font_available();
    let _ = loader.resolve_font_source();
}

#[test]
fn test_battery_monitor_events() {
    let mut monitor = BatteryMonitor::new();
    monitor.set_settled(true);

    // Unplug charger -> triggers event
    let event = monitor.update_power_state(true, true);
    assert_eq!(event, Some(BatteryToastEvent::ChargerUnplugged));

    // Drops to 15% (<= 20% warning threshold)
    let warnings = monitor.update_percentage(15);
    assert_eq!(warnings.len(), 1);
    if let BatteryToastEvent::WarnLevelTriggered { critical, .. } = &warnings[0] {
        assert!(!critical);
    } else {
        panic!("Expected WarnLevelTriggered event");
    }

    // Drops to 5% (<= 10% critical threshold)
    let criticals = monitor.update_percentage(5);
    assert_eq!(criticals.len(), 1);
    if let BatteryToastEvent::WarnLevelTriggered { critical, .. } = &criticals[0] {
        assert!(critical);
    } else {
        panic!("Expected critical WarnLevelTriggered event");
    }

    // Plug in charger -> resets warnings
    let plug_event = monitor.update_power_state(false, true);
    assert_eq!(plug_event, Some(BatteryToastEvent::ChargerPlugged));
    assert!(monitor.warn_levels.iter().all(|w| !w.warned));
}

#[test]
fn test_bluetooth_reconnect_coordinator() {
    let mut coordinator = BluetoothReconnectCoordinator::new();
    let auto_reconnect = vec!["AA:BB:CC:DD:EE:FF".to_string()];

    // Disconnected device in auto-reconnect list
    let devices = vec![
        ("AA:BB:CC:DD:EE:FF".to_string(), "Headphones".to_string(), false),
        ("11:22:33:44:55:66".to_string(), "Speaker".to_string(), false),
    ];

    let to_connect = coordinator.check_and_reconnect(true, &devices, &auto_reconnect);
    assert_eq!(to_connect, vec!["AA:BB:CC:DD:EE:FF"]);

    // Second check shouldn't re-attempt
    let second_check = coordinator.check_and_reconnect(true, &devices, &auto_reconnect);
    assert!(second_check.is_empty());

    // Resetting attempts enables reconnecting again
    coordinator.reset_attempts();
    let third_check = coordinator.check_and_reconnect(true, &devices, &auto_reconnect);
    assert_eq!(third_check, vec!["AA:BB:CC:DD:EE:FF"]);
}

#[test]
fn test_screen_corners_coordinator() {
    let mut coordinator = ScreenCornersCoordinator::new();
    let mut cfg = ScreenCornersConfig::default();

    cfg.overview_enabled = true;
    cfg.hover_top_left = true;
    cfg.hover_bottom_right = true;

    let claimed = coordinator.sync(&cfg);
    assert_eq!(claimed.len(), 2);
    assert!(coordinator.is_claimed(ScreenCorner::TopLeft));
    assert!(coordinator.is_claimed(ScreenCorner::BottomRight));
    assert!(!coordinator.is_claimed(ScreenCorner::TopRight));
}

#[test]
fn test_service_loader_phases() {
    let immediate = ServiceLoader::immediate_services();
    assert!(immediate.contains(&ShellServiceId::Notifications));
    assert!(immediate.contains(&ShellServiceId::ConfigMigrations));

    let deferred = ServiceLoader::deferred_services(true);
    assert!(deferred.contains(&ShellServiceId::Vpn));
    assert!(deferred.contains(&ShellServiceId::TrayRegistry));
    assert!(deferred.contains(&ShellServiceId::Weather));

    let mut loader = ServiceLoader::new();
    assert!(!loader.is_initialized(ShellServiceId::Notifications));
    loader.mark_initialized(ShellServiceId::Notifications);
    assert!(loader.is_initialized(ShellServiceId::Notifications));
}

#[test]
fn test_global_shortcuts_module() {
    let shortcuts = GlobalShortcutsModule::new();
    let launcher_sc = shortcuts.find_by_action("caelestia-toggle-launcher");
    assert!(launcher_sc.is_some());
    assert_eq!(launcher_sc.unwrap().default_key, "Super_L");

    let krohnkite_sc = shortcuts.find_by_action("KrohnkiteFocusUp");
    assert!(krohnkite_sc.is_some());
}

#[test]
fn test_whatsnew_and_windowinfo_rendering() {
    let c = CaelestiaColours::default();

    // WhatsNew
    let entries = WhatsNewModel::default_entries();
    assert!(!entries.is_empty());
    let _ = WhatsNewView::render(&entries, 0, &c);

    // WindowInfo
    let win = WindowItem {
        address: "win-123".to_string(),
        title: "Test Terminal".to_string(),
        app_id: "org.kde.konsole".to_string(),
        is_fullscreen: false,
        ..Default::default()
    };
    let _ = WindowInfoButtons::render(Some(&win), &c);
    let _ = WindowInfoDetails::render(Some(&win), &c);
    let _ = WindowInfoPreview::render(Some(&win), &c);
    let _ = WindowInfoView::render(Some(&win), &c);

    // PluginManager
    let mut manager = PluginManager::new();
    manager.register_plugin(PluginManifest {
        id: "cat".to_string(),
        name: "Shimeji Cat".to_string(),
        version: "1.0.0".to_string(),
        author: "Caelestia".to_string(),
        description: "Desktop pet".to_string(),
        enabled: true,
    });
    assert_eq!(manager.installed_plugins.len(), 1);
    assert!(manager.set_enabled("cat", false));
}
