//! Shortcuts Widget
//! Parity with `shell/modules/bar/components/ShortcutsWidget.qml`.

use crate::components::styled_text::StyledTextExt;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, CursorStyle, Div, Styled};

pub struct ShortcutsWidget;

impl ShortcutsWidget {
    /// Renders the keybinding cheat sheet launcher button pill.
    pub fn render(c: &CaelestiaColours) -> Div {
        div()
            .w(px(28.0))
            .h(px(28.0))
            .rounded_full()
            .bg(rgba(c.surface_container_low))
            .text_color(rgba(c.on_surface))
            .flex()
            .items_center()
            .justify_center()
            .cursor(CursorStyle::PointingHand)
            .child(StyledTextExt::icon_size(div().child("keyboard"), 14.0))
    }
}
