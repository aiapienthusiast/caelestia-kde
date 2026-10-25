//! UpdateIndicator Widget
//! Parity with `shell/modules/bar/components/UpdateIndicator.qml`.

use crate::components::styled_text::StyledTextExt;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, rgba, CursorStyle, Div, Styled};

pub struct UpdateIndicatorWidget;

impl UpdateIndicatorWidget {
    /// Renders pending software updates badge pill.
    pub fn render(update_count: usize, c: &CaelestiaColours) -> Div {
        if update_count == 0 {
            return div();
        }

        div()
            .flex()
            .flex_row()
            .items_center()
            .gap_1()
            .px_2()
            .py_1()
            .rounded_full()
            .bg(rgba(c.primary_container))
            .text_color(rgba(c.on_primary_container))
            .cursor(CursorStyle::PointingHand)
            .child(StyledTextExt::icon_size(div().child("system_update"), 14.0))
            .child(
                div()
                    .text_xs()
                    .font_weight(gpui::FontWeight::BOLD)
                    .child(update_count.to_string()),
            )
    }
}
