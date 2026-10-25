//! LockStatus Indicator
//! Parity with `shell/modules/bar/components/status/LockStatus.qml`.

use crate::components::styled_text::StyledTextExt;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, rgba, Div, Styled};

pub struct LockStatusWidget;

impl LockStatusWidget {
    pub fn render(caps_active: bool, num_active: bool, c: &CaelestiaColours) -> Div {
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap_1()
            .child(StyledTextExt::icon_size(
                div().child("keyboard_capslock"),
                14.0,
            ).text_color(rgba(if caps_active { c.primary } else { c.outline })))
            .child(StyledTextExt::icon_size(
                div().child("pin"),
                14.0,
            ).text_color(rgba(if num_active { c.primary } else { c.outline })))
    }
}
