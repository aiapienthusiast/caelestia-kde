//! Power Widget
//! Parity with `shell/modules/bar/components/Power.qml`.

use crate::components::styled_text::StyledTextExt;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, CursorStyle, Div, Styled};

pub struct PowerWidget;

impl PowerWidget {
    /// Renders the power / session launcher button pill.
    pub fn render(c: &CaelestiaColours) -> Div {
        div()
            .w(px(32.0))
            .h(px(32.0))
            .rounded_full()
            .bg(rgba(c.error_container))
            .text_color(rgba(c.on_error_container))
            .flex()
            .items_center()
            .justify_center()
            .cursor(CursorStyle::PointingHand)
            .child(StyledTextExt::icon_size(div().child("power_settings_new"), 16.0))
    }
}
