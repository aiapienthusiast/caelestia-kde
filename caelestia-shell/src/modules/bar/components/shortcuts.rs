//! Shortcuts Widget
//! Parity with `shell/modules/bar/components/ShortcutsWidget.qml`.

use crate::components::icon::{IconSize, MaterialIcon};
use crate::components::state_layer::StateLayer;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, CursorStyle, Div, Styled};

pub struct ShortcutsWidget;

impl ShortcutsWidget {
    /// Renders the shortcuts/quick toggles bar launcher button.
    pub fn render(c: &CaelestiaColours) -> Div {
        Self::render_with_state(false, false, false, c)
    }

    /// Renders with popout open state and hover/pressed states.
    pub fn render_with_state(
        is_open: bool,
        is_hovered: bool,
        is_pressed: bool,
        c: &CaelestiaColours,
    ) -> Div {
        let icon_color = if is_open {
            c.on_secondary_container
        } else {
            c.primary
        };

        let dim = 32.0;

        div()
            .relative()
            .w(px(dim))
            .h(px(dim))
            .rounded_full()
            .flex()
            .items_center()
            .justify_center()
            .cursor(CursorStyle::PointingHand)
            .child(StateLayer::overlay(
                is_hovered,
                is_pressed,
                false,
                ROUNDING_FULL,
                icon_color,
            ))
            .child(
                MaterialIcon::with_size("toggle_on", IconSize::Medium, icon_color),
            )
    }
}

