//! Greeter Widget
//! Parity with `shell/modules/bar/components/Greeter.qml`.

use crate::components::styled_text::StyledTextExt;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, CursorStyle, Div, Styled};

pub struct GreeterWidget;

impl GreeterWidget {
    /// Renders user avatar and greeting badge pill.
    pub fn render(username: &str, c: &CaelestiaColours) -> Div {
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap_2()
            .px_3()
            .py_1()
            .rounded_full()
            .bg(rgba(c.surface_container_low))
            .cursor(CursorStyle::PointingHand)
            .child(
                div()
                    .w(px(20.0))
                    .h(px(20.0))
                    .rounded_full()
                    .bg(rgba(c.primary))
                    .text_color(rgba(c.on_primary))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(StyledTextExt::icon_size(div().child("person"), 12.0)),
            )
            .child(
                div()
                    .text_color(rgba(c.on_surface))
                    .text_xs()
                    .font_weight(gpui::FontWeight::MEDIUM)
                    .child(username.to_string()),
            )
    }
}
