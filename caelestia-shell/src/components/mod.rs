//! Caelestia Shell UI Components Library
//! 100% parity with `shell/components/` and Material 3 design tokens.

pub mod anim;
pub mod calendar;
pub mod containers;
pub mod controls;
pub mod drawer_visibilities;
pub mod effects;
pub mod filedialog;
pub mod icon;
pub mod images;
pub mod logo;
pub mod misc;
pub mod screen_state;
pub mod state_layer;
pub mod styled_rect;
pub mod styled_text;
pub mod widgets;

// Flatten top-level foundational elements
pub use anim::*;
pub use calendar::*;
pub use containers::*;
pub use controls::*;
pub use drawer_visibilities::DrawerVisibilities;
pub use effects::*;
pub use filedialog::*;
pub use icon::{IconSize, MaterialIcon};
pub use images::*;
pub use logo::{Logo, LogoConfig};
pub use misc::*;
pub use screen_state::ScreenState;
pub use state_layer::StateLayer;
pub use styled_rect::{styled_clipping_rect, styled_rect, StyledRectExt};
pub use styled_text::{StyledText, StyledTextExt, TypographyScale};
pub use widgets::*;

