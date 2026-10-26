//! OsIcon Widget
//! Parity with `shell/modules/bar/components/OsIcon.qml`.

use crate::components::logo::Logo;
use crate::components::state_layer::StateLayer;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, CursorStyle, Div, Styled};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum OsIconType {
    #[default]
    Caelestia,
    Distro,
    Custom,
}

pub struct OsIconWidget;

impl OsIconWidget {
    /// Renders the start menu / OS logo pill button with default state.
    pub fn render(c: &CaelestiaColours) -> Div {
        Self::render_with_state(OsIconType::default(), false, false, c)
    }

    /// Renders with explicit icon variant, hover, and pressed state layer.
    pub fn render_with_state(
        icon_type: OsIconType,
        is_hovered: bool,
        is_pressed: bool,
        c: &CaelestiaColours,
    ) -> Div {
        let size = (FONT_SIZE_BODY_LARGE * 1.6).round();
        let total_dim = size + PADDING_MEDIUM;

        let content = match icon_type {
            OsIconType::Caelestia => Logo::new()
                .with_size(size, size * 0.7)
                .with_colors(rgba(c.secondary), rgba(c.primary)),
            OsIconType::Distro | OsIconType::Custom => Logo::new()
                .with_size(size, size * 0.7)
                .with_colors(rgba(c.secondary), rgba(c.secondary)),
        };

        div()
            .relative()
            .w(px(total_dim))
            .h(px(total_dim))
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
                c.secondary,
            ))
            .child(content)
    }
}

