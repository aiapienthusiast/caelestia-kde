//! ContentWindow Subsystem
//! Complete parity with `shell/modules/drawers/ContentWindow.qml` and `Drawers.qml`.
//!
//! Root Wayland layer-shell window coordinator per screen, hosting the Bar,
//! Dashboard, Launcher, Sidebar, and OSD inside a unified interactive canvas.

use super::exclusions::{BarPosition, ExclusionConfig, ExclusionZones};
use super::interactions::{DrawerTarget, InteractionsConfig, InteractionsTracker};
use super::panels::{NotifPosition, PanelsConfig, PanelsLayout};
use super::regions::InputRegions;
use serde::{Deserialize, Serialize};

/// Master visibility registry for all shell overlay drawers
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DrawerVisibilities {
    pub dashboard: bool,
    pub launcher: bool,
    pub overview: bool,
    pub session: bool,
    pub sidebar: bool,
    pub utilities: bool,
    pub osd: bool,
    pub sidebar_pinned: bool,
}

impl DrawerVisibilities {
    /// True if any overlay drawer is open
    pub fn has_open_overlay(&self) -> bool {
        self.dashboard
            || self.launcher
            || self.overview
            || self.session
            || self.sidebar
            || self.utilities
            || self.osd
    }

    /// True if the screen should capture clicks to dismiss overlays
    pub fn captures_overlay_input(&self) -> bool {
        self.has_open_overlay()
    }

    /// Close all transient overlays
    pub fn dismiss_all(&mut self) {
        self.dashboard = false;
        self.launcher = false;
        self.overview = false;
        self.session = false;
        if !self.sidebar_pinned {
            self.sidebar = false;
        }
        self.utilities = false;
        self.osd = false;
    }

    /// Toggle a specific drawer
    pub fn toggle(&mut self, target: DrawerTarget) {
        match target {
            DrawerTarget::Dashboard => self.dashboard = !self.dashboard,
            DrawerTarget::Launcher => self.launcher = !self.launcher,
            DrawerTarget::Sidebar => self.sidebar = !self.sidebar,
            DrawerTarget::Session => self.session = !self.session,
            DrawerTarget::Osd => self.osd = !self.osd,
            DrawerTarget::Utilities => self.utilities = !self.utilities,
        }
    }
}

/// Root coordinator representing ContentWindow.qml per screen
pub struct ContentWindowCoordinator {
    pub screen_name: String,
    pub screen_width: f64,
    pub screen_height: f64,
    pub bar_position: BarPosition,
    pub visibilities: DrawerVisibilities,
    pub exclusions: ExclusionZones,
    pub panels: PanelsLayout,
    pub regions: InputRegions,
    pub tracker: InteractionsTracker,
    pub has_fullscreen: bool,
}

impl ContentWindowCoordinator {
    pub fn new(screen_name: &str, width: f64, height: f64, bar_pos: BarPosition) -> Self {
        let bar_thickness = 48.0;
        let border_thickness = 8.0;

        let ex_config = ExclusionConfig {
            screen_width: width,
            screen_height: height,
            bar_position: bar_pos,
            bar_exclusive_zone: bar_thickness,
            border_thickness,
            sidebar_enabled: true,
            sidebar_pinned: false,
            sidebar_visible: false,
            sidebar_suspended: false,
            sidebar_width: 380.0,
        };
        let exclusions = ExclusionZones::compute(&ex_config);

        let p_config = PanelsConfig {
            bar_position: bar_pos,
            bar_width: width,
            bar_height: bar_thickness,
            border_thickness,
            islands: true,
            island_spacing: 16.0,
            notif_position: NotifPosition::Auto,
            notifs_width: 360.0,
            notifs_height: 0.0,
            sidebar_width: 380.0,
            sidebar_height: height - 100.0,
        };
        let panels = PanelsLayout::compute(&p_config);

        let regions = InputRegions::new(width, height, bar_pos, bar_thickness);

        let tracker_config = InteractionsConfig {
            screen_width: width,
            screen_height: height,
            bar_position: bar_pos,
            bar_thickness,
            border_thickness,
            hover_thickness: 16.0,
            swipe_threshold: 40.0,
        };
        let tracker = InteractionsTracker::new(tracker_config);

        Self {
            screen_name: screen_name.to_string(),
            screen_width: width,
            screen_height: height,
            bar_position: bar_pos,
            visibilities: DrawerVisibilities::default(),
            exclusions,
            panels,
            regions,
            tracker,
            has_fullscreen: false,
        }
    }

    /// Update fullscreen suppression status
    pub fn set_fullscreen(&mut self, fullscreen: bool) {
        self.has_fullscreen = fullscreen;
    }

    /// Handle pointer down event; returns true if consumed
    pub fn handle_pointer_down(&mut self, x: f64, y: f64) -> bool {
        // If an overlay is open and click is outside active UI, dismiss it
        if self.visibilities.has_open_overlay() {
            if !self.regions.should_capture_input(x, y) {
                self.visibilities.dismiss_all();
                return true;
            }
        }

        // Start edge drag gesture check
        if let Some(_target) = self.tracker.start_drag(x, y) {
            return true;
        }

        self.regions.should_capture_input(x, y)
    }

    /// Handle pointer up event
    pub fn handle_pointer_up(&mut self) -> Option<DrawerTarget> {
        if let Some((target, should_open)) = self.tracker.end_drag() {
            if should_open {
                self.visibilities.toggle(target);
                return Some(target);
            }
        }
        None
    }
}
