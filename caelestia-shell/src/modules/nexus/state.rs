//! Nexus State and Navigation Dictionary
//! Parity with `shell/modules/nexus/NexusState.qml` and `PageDictionary.qml`.

#![allow(dead_code)]

#[derive(Clone, Debug, PartialEq)]
pub struct NexusCategory {
    pub key: &'static str,
    pub label: &'static str,
    pub icon: &'static str,
    pub description: &'static str,
    pub group: &'static str,
}

pub static CATEGORIES: &[NexusCategory] = &[
    NexusCategory {
        key: "appearance",
        label: "Appearance",
        icon: "palette",
        description: "Wallpapers, fonts, colors",
        group: "personalization",
    },
    NexusCategory {
        key: "desktop",
        label: "Desktop & Tiling",
        icon: "desktop_windows",
        description: "KDE Desktop, addons, right click menu",
        group: "personalization",
    },
    NexusCategory {
        key: "panels",
        label: "Panels",
        icon: "dock_to_bottom",
        description: "Dashboard, taskbar, launcher, sidebar",
        group: "personalization",
    },
    NexusCategory {
        key: "network",
        label: "Network",
        icon: "wifi",
        description: "Wi-Fi and VPN connections",
        group: "connectivity",
    },
    NexusCategory {
        key: "bluetooth",
        label: "Bluetooth",
        icon: "bluetooth",
        description: "Paired devices, audio, peripherals",
        group: "connectivity",
    },
    NexusCategory {
        key: "display",
        label: "Display",
        icon: "monitor",
        description: "Resolution, refresh rate, display scaling",
        group: "hardware",
    },
    NexusCategory {
        key: "audio",
        label: "Audio",
        icon: "volume_up",
        description: "Volume, input and output devices",
        group: "hardware",
    },
    NexusCategory {
        key: "power",
        label: "Power & Battery",
        icon: "battery_charging_full",
        description: "Power profiles, sleep timers",
        group: "hardware",
    },
    NexusCategory {
        key: "apps",
        label: "Apps",
        icon: "apps",
        description: "Default applications, autostart",
        group: "system",
    },
    NexusCategory {
        key: "services",
        label: "Services & Features",
        icon: "auto_awesome",
        description: "AI, shortcuts, plugins",
        group: "system",
    },
    NexusCategory {
        key: "about",
        label: "About",
        icon: "info",
        description: "System and Caelestia shell version",
        group: "system",
    },
];

#[derive(Clone, Debug)]
pub struct NexusState {
    pub selected_index: usize,
    pub search_query: String,
    pub search_active: bool,
    pub hovered_category: Option<usize>,
}

impl Default for NexusState {
    fn default() -> Self {
        Self {
            selected_index: 0,
            search_query: String::new(),
            search_active: false,
            hovered_category: None,
        }
    }
}

impl NexusState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn current_category(&self) -> &'static NexusCategory {
        &CATEGORIES[self.selected_index.min(CATEGORIES.len() - 1)]
    }

    pub fn select_category(&mut self, index: usize) {
        if index < CATEGORIES.len() {
            self.selected_index = index;
            self.search_active = false;
        }
    }

    pub fn set_search_query(&mut self, query: impl Into<String>) {
        self.search_query = query.into();
        self.search_active = !self.search_query.trim().is_empty();
    }

    pub fn clear_search(&mut self) {
        self.search_query.clear();
        self.search_active = false;
    }

    /// Returns matching categories based on query.
    pub fn filtered_categories(&self) -> Vec<(usize, &'static NexusCategory)> {
        if !self.search_active {
            return CATEGORIES.iter().enumerate().collect();
        }
        let q = self.search_query.to_lowercase();
        CATEGORIES
            .iter()
            .enumerate()
            .filter(|(_, cat)| {
                cat.label.to_lowercase().contains(&q)
                    || cat.description.to_lowercase().contains(&q)
                    || cat.key.contains(&q)
            })
            .collect()
    }
}
