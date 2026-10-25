//! Caelestia Shell UI Components Library
//! 100% parity with `shell/components/` and Material 3 design tokens.

pub mod containers;
pub mod controls;
pub mod icon;
pub mod state_layer;
pub mod styled_rect;
pub mod styled_text;

// Flatten top-level foundational elements
pub use containers::*;
pub use controls::*;
pub use icon::{IconSize, MaterialIcon};
pub use state_layer::StateLayer;
pub use styled_rect::{styled_clipping_rect, styled_rect, StyledRectExt};
pub use styled_text::{StyledText, TypographyScale};
