use caelestia_shell::modules::nexus::pages::*;

#[test]
fn test_nexus_network_settings_page() {
    let mut model = NetworkSettingsPageModel::default();
    assert!(model.wifi_enabled);
    assert_eq!(model.available_networks.len(), 2);

    model.toggle_wifi();
    assert!(!model.wifi_enabled);
    assert_eq!(model.available_networks.len(), 0);

    model.toggle_wifi();
    assert!(model.wifi_enabled);

    model.toggle_hotspot();
    assert!(model.hotspot_enabled);

    model.add_vpn_profile(VpnConnectionProfile {
        name: "Proton-NL".to_string(),
        vpn_type: "WireGuard".to_string(),
        is_active: false,
        auto_connect: true,
    });
    assert_eq!(model.vpn_profiles.len(), 2);

    assert!(model.toggle_vpn("Proton-NL"));
    assert!(model.vpn_profiles.iter().find(|v| v.name == "Proton-NL").unwrap().is_active);
}

#[test]
fn test_nexus_services_settings_page() {
    let mut model = ServicesSettingsPageModel::default();
    assert!(model.gamemode_enabled);
    assert!(model.quickshare_enabled);

    model.toggle_gamemode();
    assert!(!model.gamemode_enabled);

    model.toggle_focus_mode();
    assert!(model.focus_mode_active);
    assert!(model.dnd_enabled);

    model.toggle_idle_inhibitor();
    assert!(model.idle_inhibitor_active);
}

#[test]
fn test_nexus_wallpaper_settings_page() {
    let mut model = WallpaperSettingsPageModel::default();
    assert_eq!(model.m3_scheme_variant, "TonalSpot");

    model.set_variant("Expressive");
    assert_eq!(model.m3_scheme_variant, "Expressive");

    model.set_wallpaper("/home/user/Pictures/wallpaper.png");
    assert_eq!(model.current_wallpaper_path, "/home/user/Pictures/wallpaper.png");

    model.set_accent("#FF4081");
    assert_eq!(model.custom_accent_hex.as_deref(), Some("#FF4081"));
}
