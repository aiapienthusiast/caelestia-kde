//! Overview Subsystem
//! Complete parity with `shell/modules/overview/`.
//!
//! Provides interactive virtual workspace overview, window spread and dynamic
//! packing (KDE and GNOME algorithms), workspace pill indicators, and keyboard navigation.

pub mod config;
pub mod indicator;
pub mod layout;
pub mod state;
pub mod view;

pub use config::{OverviewConfig, OverviewLayoutType};
pub use indicator::{WorkspaceIndicatorModel, WorkspacePill};
pub use layout::{calculate_gnome_layout, calculate_kde_layout, calculate_layout, WindowRect};
pub use state::OverviewState;
pub use view::OverviewView;
