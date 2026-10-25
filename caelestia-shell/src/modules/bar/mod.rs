//! Desktop Status Bar Subsystem
//! Complete parity with `shell/modules/bar/`.
//!
//! Provides the primary desktop status bar with left, middle, and right zones,
//! hosting workspaces indicators, active window dock, status cluster, clock,
//! system tray, and contextual popouts.

pub mod components;
pub mod config;
pub mod popouts;
pub mod view;
pub mod widgets;

pub use components::*;
pub use config::BarConfig;
pub use popouts::{BarPopouts, BarPopoutsState, PopoutType};
pub use view::BarView;
pub use widgets::{BarWidgetEntry, BarWidgetId, BarWidgets, BarZone};
