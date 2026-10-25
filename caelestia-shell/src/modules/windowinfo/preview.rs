//! Window Preview Container
//! Parity with `shell/modules/windowinfo/Preview.qml`.

use crate::components::styled_text::StyledTextExt;
use crate::infrastructure::theme::CaelestiaColours;
use crate::services::kwin::WindowItem;
use gpui::{div, prelude::*, px, rgba, Div, Styled};

pub struct WindowInfoPreview;

impl WindowInfoPreview {
    pub fn render(window: Option<&WindowItem>, c: &CaelestiaColours) -> Div {
        div()
            .flex()
            .items_center()
            .justify_center()
            .w(px(260.0))
            .h(px(160.0))
            .rounded_lg()
            .bg(rgba(c.surface_container_highest))
            .border_1()
            .border_color(rgba(c.outline_variant))
            .child(
                StyledTextExt::icon_size(
                    div().child(if window.is_some() { "window" } else { "desktop_windows" }),
                    48.0,
                )
                .text_color(rgba(c.primary)),
            )
    }
}
