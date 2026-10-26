//! IconButton
//! Parity with `shell/components/controls/IconButton.qml`.

use super::button_base::{ButtonBase, ButtonStyle, ButtonType};
use crate::components::icon::{IconSize, MaterialIcon};
use crate::components::state_layer::StateLayer;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{prelude::*, px, Div, SharedString};

pub struct IconButton {
    glyph: SharedString,
    colours: Option<CaelestiaColours>,
    size_px: Option<f32>,
    disabled: bool,
}

impl IconButton {
    /// Convenience constructor for an icon button with default styling
    pub fn new(glyph: impl Into<SharedString>) -> Self {
        Self {
            glyph: glyph.into(),
            colours: None,
            size_px: None,
            disabled: false,
        }
    }

    pub fn with_colours(mut self, c: CaelestiaColours) -> Self {
        self.colours = Some(c);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn size_px(mut self, size: f32) -> Self {
        self.size_px = Some(size);
        self
    }

    /// Renders an M3 IconButton with full visual and behavioral parity.
    pub fn render(
        glyph: impl Into<SharedString>,
        style: &ButtonStyle,
        c: &CaelestiaColours,
        size_px: Option<f32>,
    ) -> Div {
        let dim = size_px.unwrap_or(40.0);
        let radius = ButtonBase::calculate_radius(style, dim);

        // Colors directly matching IconButton.qml:19-30
        let active_color = if style.button_type == ButtonType::Filled {
            c.primary
        } else {
            c.secondary
        };

        let inactive_color = if !style.is_toggle && style.button_type == ButtonType::Filled {
            c.primary
        } else if style.button_type == ButtonType::Filled {
            c.surface_container
        } else {
            c.secondary_container
        };

        let active_on_color = match style.button_type {
            ButtonType::Filled => c.on_primary,
            ButtonType::Tonal => c.on_secondary,
            ButtonType::Text => c.primary,
        };

        let inactive_on_color = if !style.is_toggle && style.button_type == ButtonType::Filled {
            c.on_primary
        } else if style.button_type == ButtonType::Tonal {
            c.on_secondary_container
        } else {
            c.on_surface_variant
        };

        let bg_color = ButtonBase::calculate_bg(style, c, active_color, inactive_color);
        let on_color = ButtonBase::calculate_on_colour(style, c, active_on_color, inactive_on_color);

        let border_color = if style.is_focused {
            Some(c.primary)
        } else if style.button_type == ButtonType::Tonal {
            Some(c.outline_variant)
        } else {
            None
        };

        let container = ButtonBase::container(style, bg_color, radius, border_color)
            .w(px(dim))
            .h(px(dim));

        container
            .child(StateLayer::overlay(
                style.is_hovered,
                style.is_pressed,
                style.is_disabled,
                radius,
                on_color,
            ))
            .child(MaterialIcon::with_size(glyph, IconSize::Medium, on_color))
    }
}

impl IntoElement for IconButton {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        let dim = self.size_px.unwrap_or(36.0);
        let on_color = self.colours.as_ref().map(|c| c.on_surface).unwrap_or(0xffffffff);
        let hover_bg = self.colours.as_ref().map(|c| c.surface_container_high).unwrap_or(0xffffff15);

        let mut d = gpui::div()
            .flex()
            .items_center()
            .justify_center()
            .w(px(dim))
            .h(px(dim))
            .rounded(px(dim / 2.0));

        if self.disabled {
            d = d.opacity(0.38).cursor_not_allowed();
        } else {
            d = d.cursor_pointer().hover(move |s| s.bg(gpui::rgba(hover_bg)));
        }

        d.child(MaterialIcon::with_size(self.glyph, IconSize::Medium, on_color))
    }
}
