//! Bar Workspaces Sub-components
//! Parity with `shell/modules/bar/components/workspaces/`.

pub mod active_indicator;
pub mod gap_markers;
pub mod occupied_bg;
pub mod special_workspaces;
pub mod workspace;
pub mod workspaces;

pub use active_indicator::ActiveIndicatorWidget;
pub use gap_markers::GapMarkersWidget;
pub use occupied_bg::OccupiedBgWidget;
pub use special_workspaces::SpecialWorkspacesWidget;
pub use workspace::WorkspacePillWidget;
pub use workspaces::{WorkspacePill, WorkspacesModel, WorkspacesStripWidget};

