//! Power Widget
//! Parity with `shell/modules/bar/components/Power.qml`.

use crate::components::icon::{IconSize, MaterialIcon};
use crate::components::state_layer::StateLayer;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, CursorStyle, Div, Styled};

pub struct PowerWidget;

impl PowerWidget {
    /// Renders the power / session launcher button with StateLayer and M3 error styling.
    pub fn render(c: &CaelestiaColours) -> Div {
        Self::render_with_state(false, false, c)
    }

    /// Renders with explicit hover / pressed states for StateLayer overlay parity.
    pub fn render_with_state(is_hovered: bool, is_pressed: bool, c: &CaelestiaColours) -> Div {
        let dim = ICON_SIZE_SMALL + PADDING_SMALL;

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
                c.error,
            ))
            .child(
                MaterialIcon::with_size("power_settings_new", IconSize::Small, c.error),
            )
    }
}

