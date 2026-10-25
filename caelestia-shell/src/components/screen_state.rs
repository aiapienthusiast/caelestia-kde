//! Screen state tracking matching `shell/components/ScreenState.qml`.
//! Holds per-output surface visibilities, active dashboard tab, and dashboard date.

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Per-screen state and drawer visibility.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScreenState {
    pub screen_name: String,
    pub bar: bool,
    pub osd: bool,
    pub session: bool,
    pub launcher: bool,
    pub dashboard: bool,
    pub utilities: bool,
    pub sidebar: bool,
    pub dashboard_tab: i32,
    pub dashboard_date: NaiveDate,
}

impl ScreenState {
    pub fn new(screen_name: impl Into<String>) -> Self {
        Self {
            screen_name: screen_name.into(),
            bar: false,
            osd: false,
            session: false,
            launcher: false,
            dashboard: false,
            utilities: false,
            sidebar: false,
            dashboard_tab: 0,
            dashboard_date: chrono::Local::now().date_naive(),
        }
    }

    /// Reloadable unique ID matching `ScreenState.qml`.
    pub fn reloadable_id(&self) -> String {
        format!("screenState-{}", self.screen_name)
    }

    /// Resets all overlay drawers for this screen.
    pub fn close_all(&mut self) {
        self.session = false;
        self.launcher = false;
        self.dashboard = false;
        self.utilities = false;
        self.sidebar = false;
    }
}
