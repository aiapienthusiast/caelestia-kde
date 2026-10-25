//! Drawers Subsystem
//! Complete parity with `shell/modules/drawers/`.
//!
//! Provides the root Wayland layer-shell window coordination, screen exclusions (struts),
//! multi-edge panel margins, click-through input regions, and edge-swipe interactions.

pub mod content_window;
pub mod exclusions;
pub mod interactions;
pub mod panels;
pub mod regions;

pub use content_window::{ContentWindowCoordinator, DrawerVisibilities};
pub use exclusions::{BarPosition, ExclusionConfig, ExclusionZones, UsableArea};
pub use interactions::{DrawerTarget, GestureDrag, InteractionsConfig, InteractionsTracker};
pub use panels::{NotifPosition, PanelMargins, PanelsConfig, PanelsLayout, RectBounds};
pub use regions::{DrawerOffsetState, InputRegions, OverviewHotCorners};
