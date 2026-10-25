//! MaterialIcon
//! Parity with `shell/components/MaterialIcon.qml`.

use crate::domain::tokens::*;
use gpui::{div, prelude::*, px, rgba, Div, FontWeight, SharedString};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IconSize {
    Small,
    Medium,
    Large,
    ExtraLarge,
    Custom(f32),
}

impl IconSize {
    pub fn pixels(self) -> f32 {
        match self {
            Self::Small => ICON_SIZE_SMALL,
            Self::Medium => ICON_SIZE_MEDIUM,
            Self::Large => ICON_SIZE_LARGE,
            Self::ExtraLarge => ICON_SIZE_EXTRA_LARGE,
            Self::Custom(size) => size,
        }
    }
}

pub struct MaterialIcon;

impl MaterialIcon {
    /// Create a new MaterialIcon with default medium size.
    pub fn new(glyph: impl Into<SharedString>) -> Div {
        div()
            .text_size(px(IconSize::Medium.pixels()))
            .font_weight(FontWeight::NORMAL)
            .flex()
            .items_center()
            .justify_center()
            .child(glyph.into())
    }

    /// Create a MaterialIcon with explicit size and color.
    pub fn with_size(glyph: impl Into<SharedString>, size: IconSize, color: u32) -> Div {
        div()
            .text_size(px(size.pixels()))
            .text_color(rgba(color))
            .font_weight(FontWeight::NORMAL)
            .flex()
            .items_center()
            .justify_center()
            .child(glyph.into())
    }

    /// Helper for small icons (16px).
    pub fn small(glyph: impl Into<SharedString>, color: u32) -> Div {
        Self::with_size(glyph, IconSize::Small, color)
    }

    /// Helper for medium icons (22px).
    pub fn medium(glyph: impl Into<SharedString>, color: u32) -> Div {
        Self::with_size(glyph, IconSize::Medium, color)
    }

    /// Helper for large icons (28px).
    pub fn large(glyph: impl Into<SharedString>, color: u32) -> Div {
        Self::with_size(glyph, IconSize::Large, color)
    }
}
