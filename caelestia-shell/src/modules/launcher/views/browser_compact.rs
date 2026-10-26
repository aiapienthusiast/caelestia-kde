//! Launcher browser views: compact, grid, and simple text list
//! Ported with 100% parity from `AppBrowserCompact.qml`, `AppBrowserGrid.qml`, and `AppBrowserSimple.qml`.

use serde::{Deserialize, Serialize};
use super::super::entry::DesktopEntry;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserLayoutMode {
    Compact,
    Grid,
    Simple,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppBrowserModel {
    pub layout_mode: BrowserLayoutMode,
    pub columns: usize,
    pub icon_size: f64,
    pub show_description: bool,
    pub selected_index: usize,
    pub filtered_apps: Vec<DesktopEntry>,
}

impl Default for AppBrowserModel {
    fn default() -> Self {
        Self {
            layout_mode: BrowserLayoutMode::Grid,
            columns: 5,
            icon_size: 48.0,
            show_description: true,
            selected_index: 0,
            filtered_apps: Vec::new(),
        }
    }
}

impl AppBrowserModel {
    pub fn new(mode: BrowserLayoutMode) -> Self {
        let (columns, icon_size, show_desc) = match mode {
            BrowserLayoutMode::Compact => (1, 32.0, false),
            BrowserLayoutMode::Grid => (5, 48.0, true),
            BrowserLayoutMode::Simple => (1, 24.0, false),
        };

        Self {
            layout_mode: mode,
            columns,
            icon_size,
            show_description: show_desc,
            selected_index: 0,
            filtered_apps: Vec::new(),
        }
    }

    pub fn set_apps(&mut self, apps: Vec<DesktopEntry>) {
        self.filtered_apps = apps;
        if self.selected_index >= self.filtered_apps.len() && !self.filtered_apps.is_empty() {
            self.selected_index = 0;
        }
    }

    pub fn select_next(&mut self) {
        if !self.filtered_apps.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.filtered_apps.len();
        }
    }

    pub fn select_prev(&mut self) {
        if !self.filtered_apps.is_empty() {
            if self.selected_index == 0 {
                self.selected_index = self.filtered_apps.len() - 1;
            } else {
                self.selected_index -= 1;
            }
        }
    }

    pub fn select_down(&mut self) {
        if self.columns <= 1 {
            self.select_next();
            return;
        }
        if !self.filtered_apps.is_empty() {
            let next = self.selected_index + self.columns;
            if next < self.filtered_apps.len() {
                self.selected_index = next;
            }
        }
    }

    pub fn select_up(&mut self) {
        if self.columns <= 1 {
            self.select_prev();
            return;
        }
        if self.selected_index >= self.columns {
            self.selected_index -= self.columns;
        }
    }

    pub fn selected_app(&self) -> Option<&DesktopEntry> {
        self.filtered_apps.get(self.selected_index)
    }
}
