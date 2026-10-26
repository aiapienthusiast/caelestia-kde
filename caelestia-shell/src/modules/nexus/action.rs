//! Declarative action event types for Nexus Control Center window.
//! High-performance, allocation-minimal message dispatch across all settings domains.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NexusAction {
    // Appearance & Theming
    TogglePitchBlack,
    ToggleTransparency,
    ToggleBlur,
    SetBaseOpacity(f64),
    SetCornerRadiusScale(f64),
    SetBorderThickness(f64),
    ResetAppearanceSettings,

    // Desktop & Tiling
    ToggleKrohnkiteTiling(bool),
    ToggleDesktopIcons(bool),
    ToggleContextMenu(bool),
    ToggleWorkspacesIndicator(bool),

    // Panels & Docks
    ToggleTaskbar(bool),
    ToggleDashboard(bool),
    ToggleLauncher(bool),
    ToggleDockMode(bool),
    ToggleSidebarToggles(bool),

    // Network & Connectivity
    ToggleWifi(bool),
    ToggleHotspot(bool),
    ToggleVpn(bool),
    ConnectWifi(String),

    // Bluetooth
    ToggleBluetooth(bool),
    ConnectBtDevice(String),

    // Display
    SetDisplayScaling(f64),
    ToggleNightLight(bool),
    SetDisplayRefreshRate(u32),

    // Audio
    SetMasterVolume(f64),
    ToggleMute(bool),
    SelectAudioSink(String),

    // Power
    SetPowerProfile(String),
    ToggleBatteryConservation(bool),

    // Applications & Autostart
    UninstallApp(String),
    ToggleAutostartApp(String, bool),

    // Background Services & Integrations
    ToggleGamemode(bool),
    ToggleQuickShare(bool),
    ToggleFocusMode(bool),
    ToggleIdleInhibitor(bool),

    // Navigation & Window Lifecycle
    SelectCategory(usize),
    SetSearchQuery(String),
    ClearSearch,
    CloseWindow,
}
