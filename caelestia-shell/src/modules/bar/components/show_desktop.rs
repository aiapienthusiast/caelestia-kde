//! ShowDesktop Widget
//! Parity with `shell/modules/bar/components/ShowDesktop.qml`.

use crate::components::icon::{IconSize, MaterialIcon};
use crate::components::state_layer::StateLayer;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, CursorStyle, Div, Styled};

pub struct ShowDesktopWidget;

impl ShowDesktopWidget {
    /// Renders the minimize all windows / reveal desktop trigger pill.
    pub fn render(c: &CaelestiaColours) -> Div {
        Self::render_with_state(false, false, false, c)
    }

    /// Renders with showing_desktop state and hover/pressed states.
    pub fn render_with_state(
        showing_desktop: bool,
        is_hovered: bool,
        is_pressed: bool,
        c: &CaelestiaColours,
    ) -> Div {
        let icon_size = ICON_SIZE_SMALL;
        let dim = icon_size + PADDING_SMALL;
        let icon_name = if showing_desktop {
            "keyboard_double_arrow_up"
        } else {
            "keyboard_double_arrow_down"
        };

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
                c.on_surface_variant,
            ))
            .child(
                MaterialIcon::with_size(icon_name, IconSize::Small, c.on_surface_variant),
            )
    }
}

