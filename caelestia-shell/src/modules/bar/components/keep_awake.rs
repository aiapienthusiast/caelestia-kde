//! KeepAwake Widget
//! Parity with `shell/modules/bar/components/KeepAwakeWidget.qml`.

use crate::components::styled_text::StyledTextExt;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, CursorStyle, Div, Styled};

pub struct KeepAwakeWidget;

impl KeepAwakeWidget {
    /// Renders the caffeine / sleep inhibitor toggle pill.
    pub fn render(is_active: bool, c: &CaelestiaColours) -> Div {
        div()
            .w(px(28.0))
            .h(px(28.0))
            .rounded_full()
            .bg(if is_active {
                rgba(c.primary)
            } else {
                rgba(c.surface_container_low)
            })
            .text_color(if is_active {
                rgba(c.on_primary)
            } else {
                rgba(c.on_surface)
            })
            .flex()
            .items_center()
            .justify_center()
            .cursor(CursorStyle::PointingHand)
            .child(StyledTextExt::icon_size(div().child("coffee"), 14.0))
    }
}
