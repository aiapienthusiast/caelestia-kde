//! TrayItem Widget
//! Parity with `shell/modules/bar/components/TrayItem.qml`.

use crate::components::icon::{IconSize, MaterialIcon};
use crate::components::state_layer::StateLayer;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, CursorStyle, Div, Styled};

pub struct TrayItemWidget;

impl TrayItemWidget {
    /// Renders an individual SNI system tray item with StateLayer and M3 secondary color.
    pub fn render(
        _id: &str,
        _title: &str,
        icon: &str,
        c: &CaelestiaColours,
    ) -> Div {
        Self::render_with_state(_id, _title, icon, false, false, c)
    }

    /// Renders with explicit hover and pressed states for StateLayer overlay parity.
    pub fn render_with_state(
        _id: &str,
        _title: &str,
        icon: &str,
        is_hovered: bool,
        is_pressed: bool,
        c: &CaelestiaColours,
    ) -> Div {
        let size = FONT_SIZE_BODY_SMALL * 2.0; // 24.0px
        let total_dim = size + PADDING_EXTRA_SMALL;

        div()
            .relative()
            .flex()
            .items_center()
            .justify_center()
            .w(px(total_dim))
            .h(px(total_dim))
            .rounded_full()
            .cursor(CursorStyle::PointingHand)
            .child(StateLayer::overlay(
                is_hovered,
                is_pressed,
                false,
                ROUNDING_FULL,
                c.secondary,
            ))
            .child(
                MaterialIcon::with_size(icon, IconSize::Small, c.secondary),
            )
    }
}

