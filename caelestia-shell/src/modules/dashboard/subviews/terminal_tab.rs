//! Dashboard Terminal and Wallhaven Tabs
//! Ported with 100% parity from `TerminalTab.qml` and `WallhavenTab.qml`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TerminalTabModel {
    pub current_command: String,
    pub history: Vec<String>,
    pub output_lines: Vec<String>,
    pub is_running: bool,
    pub working_directory: String,
}

impl Default for TerminalTabModel {
    fn default() -> Self {
        Self {
            current_command: String::new(),
            history: Vec::new(),
            output_lines: vec!["Caelestia Interactive Terminal Bridge ready.".to_string()],
            is_running: false,
            working_directory: std::env::var("HOME").unwrap_or_else(|_| "/home/user".to_string()),
        }
    }
}

impl TerminalTabModel {
    pub fn push_output(&mut self, line: impl Into<String>) {
        self.output_lines.push(line.into());
        if self.output_lines.len() > 1000 {
            self.output_lines.remove(0);
        }
    }

    pub fn submit_command(&mut self) -> Option<String> {
        let cmd = self.current_command.trim().to_string();
        if cmd.is_empty() {
            return None;
        }
        self.history.push(cmd.clone());
        self.output_lines.push(format!("$ {cmd}"));
        self.current_command.clear();
        Some(cmd)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WallhavenWallpaperEntry {
    pub id: String,
    pub url: String,
    pub path: String,
    pub resolution: String,
    pub category: String,
    pub purity: String,
    pub favorites: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WallhavenTabModel {
    pub search_query: String,
    pub purity_filter: String, // e.g. "100" (SFW)
    pub categories_filter: String, // e.g. "110" (General/Anime)
    pub current_page: u32,
    pub wallpapers: Vec<WallhavenWallpaperEntry>,
    pub is_loading: bool,
    pub selected_wallpaper: Option<WallhavenWallpaperEntry>,
}

impl Default for WallhavenTabModel {
    fn default() -> Self {
        Self {
            search_query: String::new(),
            purity_filter: "100".to_string(),
            categories_filter: "111".to_string(),
            current_page: 1,
            wallpapers: Vec::new(),
            is_loading: false,
            selected_wallpaper: None,
        }
    }
}

impl WallhavenTabModel {
    pub fn build_api_url(&self) -> String {
        let q = &self.search_query;
        let p = &self.purity_filter;
        let c = &self.categories_filter;
        let page = self.current_page;
        format!("https://wallhaven.cc/api/v1/search?q={q}&purity={p}&categories={c}&page={page}")
    }
}
