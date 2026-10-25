//! OsIcon Widget
//! Parity with `shell/modules/bar/components/OsIcon.qml`.

use crate::components::logo::Logo;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, CursorStyle, Div, Styled};

pub struct OsIconWidget;

impl OsIconWidget {
    /// Renders the start menu / OS logo pill button with Caelestia Logo.
    pub fn render(c: &CaelestiaColours) -> Div {
        div()
            .w(px(32.0))
            .h(px(32.0))
            .rounded_full()
            .bg(rgba(c.primary))
            .text_color(rgba(c.on_primary))
            .flex()
            .items_center()
            .justify_center()
            .cursor(CursorStyle::PointingHand)
            .child(
                Logo::new()
                    .with_size(20.0, 14.0)
                    .with_colors(rgba(c.on_primary), rgba(c.on_primary)),
            )
    }
}
