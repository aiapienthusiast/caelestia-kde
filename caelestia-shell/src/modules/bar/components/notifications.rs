//! NotificationsIndicator Widget
//! Parity with `shell/modules/bar/components/NotificationsIndicator.qml`.

use crate::components::styled_text::StyledTextExt;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, rgba, CursorStyle, Div, Styled};

pub struct NotificationsIndicatorWidget;

impl NotificationsIndicatorWidget {
    /// Renders unread notifications count badge pill.
    pub fn render(count: usize, c: &CaelestiaColours) -> Div {
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap_1()
            .px_2()
            .py_1()
            .rounded_full()
            .bg(rgba(c.surface_container_low))
            .text_color(rgba(c.on_surface))
            .cursor(CursorStyle::PointingHand)
            .child(StyledTextExt::icon_size(
                div().child(if count > 0 { "notifications_active" } else { "notifications" }),
                14.0,
            ))
            .child(
                div()
                    .text_xs()
                    .font_weight(gpui::FontWeight::MEDIUM)
                    .child(count.to_string()),
            )
    }
}
