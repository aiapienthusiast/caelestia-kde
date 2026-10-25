//! OsIcon Widget
//! Parity with `shell/modules/bar/components/OsIcon.qml`.

use crate::components::styled_text::StyledTextExt;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, CursorStyle, Div, Styled};

pub struct OsIconWidget;

impl OsIconWidget {
    /// Renders the start menu / OS logo pill button.
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
            .child(StyledTextExt::icon_size(div().child("auto_awesome"), 16.0))
    }
}
