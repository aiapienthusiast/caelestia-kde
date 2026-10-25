//! Desktop Status Bar Subsystem
//! Complete parity with `shell/modules/bar/`.
//!
//! Provides the primary desktop status bar with left, middle, and right zones,
//! hosting workspaces indicators, active window dock, status cluster, clock,
//! and contextual popouts.

pub mod clock;
pub mod config;
pub mod dock;
pub mod popouts;
pub mod status;
pub mod view;
pub mod widgets;
pub mod workspaces;

pub use clock::{BarClockModel, ClockSnapshot};
pub use config::BarConfig;
pub use dock::{BarDockModel, DockSnapshot};
pub use popouts::{BarPopoutsState, PopoutType};
pub use status::{BarStatusSnapshot, StatusIconsModel};
pub use view::BarView;
pub use widgets::{BarWidgetEntry, BarWidgetId, BarWidgets, BarZone};
pub use workspaces::{WorkspacePill, WorkspacesModel};
