//! Nexus Control Center & Settings Application Window.
//! Parity with `shell/modules/nexus/`.

pub mod action;
pub mod common;
pub mod nav_pane;
pub mod pages;
pub mod state;
pub mod window;

pub use action::NexusAction;
pub use nav_pane::NavPane;
pub use state::{NexusCategory, NexusState, CATEGORIES};
pub use window::NexusWindow;
