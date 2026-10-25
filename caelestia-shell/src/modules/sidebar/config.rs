//! Sidebar Configuration Subsystem
//! Complete parity with `shell/modules/sidebar/` and `Config.sidebar`.

use serde::{Deserialize, Serialize};

/// Sidebar drawer configuration options
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SidebarConfig {
    pub enabled: bool,
    pub pinned: bool,
    pub default_width: f32,
    pub min_width: f32,
    pub max_width_ratio: f32,
    pub enable_ai_assistant: bool,
    pub show_news: bool,
    pub show_dino_game: bool,
    pub initial_tab: String,
}

impl Default for SidebarConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            pinned: false,
            default_width: 420.0,
            min_width: 320.0,
            max_width_ratio: 0.5,
            enable_ai_assistant: true,
            show_news: true,
            show_dino_game: true,
            initial_tab: "notifications".to_string(),
        }
    }
}
