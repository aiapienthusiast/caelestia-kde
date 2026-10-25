//! GithubActivity Widget
//! Parity with `shell/modules/bar/components/GithubActivity.qml` and `GithubStore.qml`.

use crate::components::styled_text::StyledTextExt;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, rgba, CursorStyle, Div, Styled};

pub struct GithubActivityWidget;

impl GithubActivityWidget {
    /// Renders GitHub notification / activity pill.
    pub fn render(unread_count: usize, c: &CaelestiaColours) -> Div {
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
            .child(StyledTextExt::icon_size(div().child("commit"), 14.0))
            .child(
                div()
                    .text_xs()
                    .font_weight(gpui::FontWeight::MEDIUM)
                    .child(unread_count.to_string()),
            )
    }
}
