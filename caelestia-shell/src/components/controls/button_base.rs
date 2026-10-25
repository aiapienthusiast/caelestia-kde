//! ButtonBase
//! Parity with `shell/components/controls/ButtonBase.qml`.

use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, px, rgba, CursorStyle, Div, Styled};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ButtonType {
    Filled,
    Tonal,
    Text,
}

#[derive(Clone, Debug)]
pub struct ButtonStyle {
    pub button_type: ButtonType,
    pub is_toggle: bool,
    pub is_checked: bool,
    pub is_round: bool,
    pub is_disabled: bool,
    pub is_hovered: bool,
    pub is_pressed: bool,
    pub is_focused: bool,
    pub radius_morph: bool,
}

impl Default for ButtonStyle {
    fn default() -> Self {
        Self {
            button_type: ButtonType::Filled,
            is_toggle: false,
            is_checked: false,
            is_round: false,
            is_disabled: false,
            is_hovered: false,
            is_pressed: false,
            is_focused: false,
            radius_morph: true,
        }
    }
}

pub struct ButtonBase;

impl ButtonBase {
    /// Calculate the dynamic corner radius matching ButtonBase.qml:53-61.
    pub fn calculate_radius(style: &ButtonStyle, implicit_height: f32) -> f32 {
        if style.radius_morph && style.is_pressed {
            ROUNDING_SMALL
        } else if style.is_checked {
            ROUNDING_MEDIUM
        } else if style.is_round {
            implicit_height / 2.0
        } else {
            ROUNDING_LARGE
        }
    }

    /// Calculate button background color matching ButtonBase.qml:62.
    pub fn calculate_bg(
        style: &ButtonStyle,
        c: &CaelestiaColours,
        active_color: u32,
        inactive_color: u32,
    ) -> u32 {
        if style.button_type == ButtonType::Text {
            // Text button has transparent background
            inactive_color & 0xffffff00
        } else if style.is_disabled {
            c.disabled_container
        } else if style.is_checked {
            active_color
        } else {
            inactive_color
        }
    }

    /// Calculate button on-color (content / text / icon) matching ButtonBase.qml:43.
    pub fn calculate_on_colour(
        style: &ButtonStyle,
        c: &CaelestiaColours,
        active_on_color: u32,
        inactive_on_color: u32,
    ) -> u32 {
        if style.is_disabled {
            c.disabled_content
        } else if style.is_checked {
            active_on_color
        } else {
            inactive_on_color
        }
    }

    /// Creates an M3 container styled div for the button base.
    pub fn container(
        style: &ButtonStyle,
        bg_color: u32,
        radius_px: f32,
        border_color: Option<u32>,
    ) -> Div {
        let mut d = div()
            .relative()
            .bg(rgba(bg_color))
            .rounded(px(radius_px))
            .flex()
            .items_center()
            .justify_center();

        if let Some(border) = border_color {
            d = d.border_1().border_color(rgba(border));
        }

        if !style.is_disabled {
            d = d.cursor(CursorStyle::PointingHand);
        }

        d
    }
}
