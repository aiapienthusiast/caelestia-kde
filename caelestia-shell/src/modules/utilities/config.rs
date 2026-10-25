//! Utilities Configuration Subsystem
//! Parity with `shell/modules/utilities/` and `Config.utilities`.

use serde::{Deserialize, Serialize};

/// Quick Toggle definition matching `Config.utilities.quickToggles`
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuickToggleItem {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub enabled: bool,
}

/// Configuration parameters for utilities drawer and its cards
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UtilitiesConfig {
    pub enabled: bool,
    pub show_keep_awake: bool,
    pub show_screen_recorder: bool,
    pub show_quick_share: bool,
    pub show_quick_toggles: bool,
    pub quick_toggles_per_page: usize,
    pub quick_toggles_custom_order: bool,
    pub quick_toggles: Vec<QuickToggleItem>,
}

impl Default for UtilitiesConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            show_keep_awake: true,
            show_screen_recorder: true,
            show_quick_share: true,
            show_quick_toggles: true,
            quick_toggles_per_page: 6,
            quick_toggles_custom_order: false,
            quick_toggles: vec![
                QuickToggleItem {
                    id: "wifi".to_string(),
                    name: "Wi-Fi".to_string(),
                    icon: "wifi".to_string(),
                    enabled: true,
                },
                QuickToggleItem {
                    id: "bluetooth".to_string(),
                    name: "Bluetooth".to_string(),
                    icon: "bluetooth".to_string(),
                    enabled: true,
                },
                QuickToggleItem {
                    id: "nightlight".to_string(),
                    name: "Night Light".to_string(),
                    icon: "nightlight".to_string(),
                    enabled: true,
                },
                QuickToggleItem {
                    id: "dnd".to_string(),
                    name: "Do Not Disturb".to_string(),
                    icon: "notifications_off".to_string(),
                    enabled: true,
                },
                QuickToggleItem {
                    id: "mic_mute".to_string(),
                    name: "Mute Mic".to_string(),
                    icon: "mic_off".to_string(),
                    enabled: true,
                },
                QuickToggleItem {
                    id: "pause_wallpaper".to_string(),
                    name: "Pause Wallpaper".to_string(),
                    icon: "pause_circle".to_string(),
                    enabled: true,
                },
                QuickToggleItem {
                    id: "hotspot".to_string(),
                    name: "Hotspot".to_string(),
                    icon: "wifi_tethering".to_string(),
                    enabled: true,
                },
                QuickToggleItem {
                    id: "restart_shell".to_string(),
                    name: "Restart Shell".to_string(),
                    icon: "restart_alt".to_string(),
                    enabled: true,
                },
            ],
        }
    }
}
