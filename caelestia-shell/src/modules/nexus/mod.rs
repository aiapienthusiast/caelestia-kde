//! Nexus Control Center & Settings Drawer.
//! Parity with `shell/modules/nexus/`.

pub mod drawer;
pub mod nav_pane;
pub mod pages;
pub mod state;

pub use drawer::NexusDrawer;
pub use nav_pane::NavPane;
pub use state::{NexusCategory, NexusState, CATEGORIES};
