//! Menu
//! Parity with `shell/components/controls/Menu.qml`.

use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, Div, Styled};

pub struct Menu;

impl Menu {
    /// Renders a floating popup menu container styled according to Material 3 tokens.
    pub fn render(
        min_width: f32,
        children: Vec<Div>,
        c: &CaelestiaColours,
    ) -> Div {
        let mut container = div()
            .relative()
            .flex()
            .flex_col()
            .min_w(px(min_width.max(160.0)))
            .p(px(SPACING_SMALL))
            .rounded(px(ROUNDING_MEDIUM))
            .bg(rgba(c.surface_container))
            .border_1()
            .border_color(rgba(c.outline_variant))
            .shadow_lg();

        for child in children {
            container = container.child(child);
        }

        container
    }

    /// Renders a horizontal divider line inside a menu.
    pub fn divider(c: &CaelestiaColours) -> Div {
        div()
            .w_full()
            .h(px(1.0))
            .my(px(SPACING_SMALL))
            .bg(rgba(c.outline_variant))
    }
}
