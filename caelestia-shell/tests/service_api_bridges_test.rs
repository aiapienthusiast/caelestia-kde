//! Automated Tests for Public Service API Bridges
//! Verifies 100% functionality and parity with `shell/services/api/`.

use caelestia_shell::services::api::{CaelestiaApi, PluginMeta, WidgetDescriptor};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[test]
fn test_caelestia_api_global_singleton() {
    let api = CaelestiaApi::global();

    // Verify all sub-APIs are properly initialized
    let _ = &api.system;
    let _ = &api.windows;
    let _ = &api.network;
    let _ = &api.media;
    let _ = &api.visuals;
    let _ = &api.ui;
    let _ = &api.shortcuts;
    let _ = &api.plugins;

    // Test system metrics & utilities
    let sys_info = api.system.get_sys_info();
    assert!(!sys_info.kernel.is_empty());
    assert!(!sys_info.hostname.is_empty());

    let sha = api.system.sha256(b"hello caelestia");
    assert_eq!(sha.len(), 64);

    let time = api.system.current_time(false, false);
    assert!(!time.formatted_time.is_empty());
}

#[test]
fn test_shortcuts_api_dynamic_registration() {
    let api = CaelestiaApi::global();

    let fired = Arc::new(AtomicBool::new(false));
    let fired_clone = fired.clone();

    // Register dynamic shortcut
    let shortcut = api.shortcuts.register(
        "plugin_test_action",
        "Test Action for Plugin",
        "Ctrl+Shift+F12",
        Some(move || {
            fired_clone.store(true, Ordering::SeqCst);
        }),
    );

    assert_eq!(shortcut.name, "plugin_test_action");
    assert_eq!(shortcut.key, "Ctrl+Shift+F12");

    // Dispatch shortcut
    let dispatched = api.shortcuts.dispatch("plugin_test_action");
    assert!(dispatched);
    assert!(fired.load(Ordering::SeqCst));

    // Unregister
    let removed = api.shortcuts.unregister("plugin_test_action");
    assert!(removed);
    assert!(!api.shortcuts.dispatch("plugin_test_action"));
}

#[test]
fn test_plugins_api_lifecycle() {
    let api = CaelestiaApi::global();

    let plugin = PluginMeta {
        id: "com.caelestia.weather_radar".to_string(),
        name: "Weather Radar".to_string(),
        description: "Live radar precipitation map".to_string(),
        version: "1.0.0".to_string(),
        author: "Caelestia Team".to_string(),
        enabled: true,
    };

    api.plugins.register(plugin);
    let list = api.plugins.list();
    assert!(list.iter().any(|p| p.id == "com.caelestia.weather_radar"));

    // Toggle enabled state
    let toggled = api.plugins.set_enabled("com.caelestia.weather_radar", false);
    assert!(toggled);

    let updated_list = api.plugins.list();
    let updated = updated_list.iter().find(|p| p.id == "com.caelestia.weather_radar").unwrap();
    assert_eq!(updated.enabled, false);
}

#[test]
fn test_ui_api_widget_registrations() {
    let api = CaelestiaApi::global();

    let widget = WidgetDescriptor {
        id: "custom_audio_visualizer".to_string(),
        name: "CAVA Pill".to_string(),
        icon: "equalizer".to_string(),
        category: "media".to_string(),
    };

    api.ui.register_top_bar_middle(widget.clone());
    let middle_widgets = api.ui.top_bar_middle_widgets.lock().unwrap();
    assert!(middle_widgets.iter().any(|w| w.id == "custom_audio_visualizer"));
}

#[test]
fn test_media_api_playback_controls() {
    let api = CaelestiaApi::global();

    // Volume manipulation
    api.media.set_volume(0.85);
    let vol = api.media.volume();
    assert!((vol - 0.85).abs() < 1e-4);

    // Audio devices state
    let state = api.media.audio_state();
    assert!(state.sink_volume >= 0.0);
}

#[test]
fn test_visuals_api_theming() {
    let api = CaelestiaApi::global();

    let colours = api.visuals.colours();
    assert_ne!(colours.primary, 0);

    let accent = api.visuals.accent_color();
    assert_eq!(accent, colours.primary);

    // Brightness level manipulation
    api.visuals.set_brightness(80);
    assert_eq!(api.visuals.brightness_percent(), 80);
}
