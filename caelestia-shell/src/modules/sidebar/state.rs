//! Sidebar State Coordinator
//! Complete parity with `shell/modules/sidebar/Wrapper.qml` and `Content.qml`.

use super::ai_assistant::AiAssistantModel;
use super::config::SidebarConfig;
use super::news::NewsModel;
use super::tabs::SidebarTab;
use tracing::info;

/// Master state coordinator for the Sidebar drawer
pub struct SidebarState {
    pub is_open: bool,
    pub pinned: bool,
    pub width: f32,
    pub min_width: f32,
    pub max_width: f32,
    pub active_tab: SidebarTab,
    pub ai_assistant: AiAssistantModel,
    pub news: NewsModel,
    pub is_resizing: bool,
}

impl SidebarState {
    pub fn new(config: &SidebarConfig, screen_width: f32) -> Self {
        let max_w = (screen_width * config.max_width_ratio).max(config.min_width);
        Self {
            is_open: false,
            pinned: config.pinned,
            width: config.default_width.clamp(config.min_width, max_w),
            min_width: config.min_width,
            max_width: max_w,
            active_tab: SidebarTab::from_id(&config.initial_tab),
            ai_assistant: AiAssistantModel::new(),
            news: NewsModel::new(),
            is_resizing: false,
        }
    }

    /// Open sidebar
    pub fn open(&mut self) {
        self.is_open = true;
    }

    /// Close sidebar (only closes if not pinned, unless forced)
    pub fn close(&mut self) {
        if !self.pinned {
            self.is_open = false;
        }
    }

    /// Force close even when pinned
    pub fn force_close(&mut self) {
        self.is_open = false;
    }

    /// Toggle sidebar visibility
    pub fn toggle(&mut self) {
        if self.is_open {
            self.force_close();
        } else {
            self.open();
        }
    }

    /// Switch active tab
    pub fn switch_tab(&mut self, tab: SidebarTab) {
        info!("Sidebar switching to tab: {:?}", tab);
        self.active_tab = tab;
    }

    /// Set pinned state
    pub fn set_pinned(&mut self, pinned: bool) {
        info!("Sidebar pinned: {}", pinned);
        self.pinned = pinned;
        if pinned {
            self.is_open = true;
        }
    }

    /// Toggle pinned state
    pub fn toggle_pinned(&mut self) {
        let next = !self.pinned;
        self.set_pinned(next);
    }

    /// Update sidebar width during edge drag, adhering to min and max boundaries
    pub fn set_width(&mut self, new_width: f32) {
        self.width = new_width.clamp(self.min_width, self.max_width);
    }

    /// Reset width to default
    pub fn reset_width(&mut self, default_width: f32) {
        self.width = default_width.clamp(self.min_width, self.max_width);
    }
}
