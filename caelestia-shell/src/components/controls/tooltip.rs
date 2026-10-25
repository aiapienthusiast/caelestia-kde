//! Tooltip
//! Parity with `shell/components/controls/Tooltip.qml`.

use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, Div, SharedString, Styled};

pub struct Tooltip;

impl Tooltip {
    /// Renders an M3 Tooltip bubble matching Tooltip.qml.
    pub fn render(text: impl Into<SharedString>, c: &CaelestiaColours) -> Div {
        div()
            .bg(rgba(c.inverse_surface))
            .rounded(px(ROUNDING_EXTRA_SMALL))
            .px(px(PADDING_MEDIUM))
            .py(px(PADDING_SMALL / 2.0))
            .shadow_md()
            .child(
                div()
                    .text_size(px(FONT_SIZE_SMALL))
                    .text_color(rgba(c.inverse_on_surface))
                    .child(text.into()),
            )
    }
}
