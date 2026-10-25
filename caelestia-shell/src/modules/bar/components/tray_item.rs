//! TrayItem Widget
//! Parity with `shell/modules/bar/components/TrayItem.qml`.

use crate::components::styled_text::StyledTextExt;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, Div, Styled};

pub struct TrayItemWidget;

impl TrayItemWidget {
    pub fn render(
        _id: &str,
        _title: &str,
        icon: &str,
        c: &CaelestiaColours,
    ) -> Div {
        div()
            .flex()
            .items_center()
            .justify_center()
            .w(px(24.0))
            .h(px(24.0))
            .rounded_full()
            .hover(|s| s.bg(rgba(c.surface_container_highest)))
            .cursor_pointer()
            .child(
                StyledTextExt::icon_size(div().child(icon.to_string()), 16.0)
                    .text_color(rgba(c.secondary)),
            )
    }
}
