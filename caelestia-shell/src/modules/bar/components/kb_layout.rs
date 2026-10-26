//! KbLayoutIndicator Widget
//! Parity with `shell/modules/bar/components/KbLayoutIndicator.qml`.

use crate::components::state_layer::StateLayer;
use crate::components::styled_text::StyledText;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, CursorStyle, Div, FontWeight, Styled};

pub struct KbLayoutIndicatorWidget;

impl KbLayoutIndicatorWidget {
    /// Renders active keyboard layout code pill (e.g. "US", "DE").
    pub fn render(layout_code: &str, c: &CaelestiaColours) -> Div {
        Self::render_with_state(layout_code, false, false, c)
    }

    /// Renders with explicit hover and pressed states.
    pub fn render_with_state(
        layout_code: &str,
        is_hovered: bool,
        is_pressed: bool,
        c: &CaelestiaColours,
    ) -> Div {
        div()
            .relative()
            .px(px(PADDING_MEDIUM))
            .py(px(PADDING_SMALL))
            .rounded_full()
            .bg(rgba(c.surface_container))
            .cursor(CursorStyle::PointingHand)
            .child(StateLayer::overlay(
                is_hovered,
                is_pressed,
                false,
                ROUNDING_FULL,
                c.secondary,
            ))
            .child(
                StyledText::new(layout_code.to_uppercase())
                    .text_size(px(FONT_SIZE_MONO_MEDIUM))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgba(c.secondary)),
            )
    }
}

