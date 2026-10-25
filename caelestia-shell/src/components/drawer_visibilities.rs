//! Drawer visibilities state matching `shell/components/DrawerVisibilities.qml`.
//! Controls open/closed visibility across all shell surface overlays.

use serde::{Deserialize, Serialize};

/// Visibility state of the shell drawers and overlays.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct DrawerVisibilities {
    pub bar: bool,
    pub osd: bool,
    pub session: bool,
    pub launcher: bool,
    pub dashboard: bool,
    pub utilities: bool,
    pub sidebar: bool,
    pub sidebar_suspended: bool,
    pub overview: bool,
    pub skip_launcher_anim: bool,
}

impl DrawerVisibilities {
    pub fn new() -> Self {
        Self::default()
    }

    /// Checks if any interactive drawer (launcher, dashboard, session, utilities, sidebar, overview) is currently open.
    pub fn is_any_drawer_open(&self) -> bool {
        self.launcher
            || self.dashboard
            || self.session
            || self.utilities
            || self.sidebar
            || self.overview
    }

    /// Closes all modal and overlay drawers.
    pub fn close_all(&mut self) {
        self.session = false;
        self.launcher = false;
        self.dashboard = false;
        self.utilities = false;
        self.sidebar = false;
        self.overview = false;
    }

    /// Toggles the launcher visibility.
    pub fn toggle_launcher(&mut self) {
        let next = !self.launcher;
        self.close_all();
        self.launcher = next;
    }

    /// Toggles the dashboard visibility.
    pub fn toggle_dashboard(&mut self) {
        let next = !self.dashboard;
        self.close_all();
        self.dashboard = next;
    }

    /// Toggles the utilities drawer visibility.
    pub fn toggle_utilities(&mut self) {
        let next = !self.utilities;
        self.close_all();
        self.utilities = next;
    }

    /// Toggles the session dialog visibility.
    pub fn toggle_session(&mut self) {
        let next = !self.session;
        self.close_all();
        self.session = next;
    }

    /// Toggles the sidebar visibility.
    pub fn toggle_sidebar(&mut self) {
        let next = !self.sidebar;
        self.close_all();
        self.sidebar = next;
    }

    /// Toggles the overview visibility.
    pub fn toggle_overview(&mut self) {
        let next = !self.overview;
        self.close_all();
        self.overview = next;
    }
}
