//! IconTextButton
//! Parity with `shell/components/controls/IconTextButton.qml`.

use super::button_base::{ButtonBase, ButtonStyle, ButtonType};
use crate::components::icon::{IconSize, MaterialIcon};
use crate::components::state_layer::StateLayer;
use crate::components::styled_text::TypographyScale;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, Div, SharedString};

pub struct IconTextButton {
    icon: SharedString,
    text: SharedString,
    colours: Option<CaelestiaColours>,
    disabled: bool,
}

impl IconTextButton {
    /// Convenience constructor for an icon text button with default styling
    pub fn new(icon: impl Into<SharedString>, text: impl Into<SharedString>) -> Self {
        Self {
            icon: icon.into(),
            text: text.into(),
            colours: None,
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

    /// Renders an M3 IconTextButton with full visual and behavioral parity.
    pub fn render(
        icon: impl Into<SharedString>,
        text: impl Into<SharedString>,
        style: &ButtonStyle,
        c: &CaelestiaColours,
    ) -> Div {
        let height_px = 40.0;
        let radius = ButtonBase::calculate_radius(style, height_px);

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

        let active_on_color = if style.button_type == ButtonType::Text {
            c.primary
        } else if style.button_type == ButtonType::Filled {
            c.on_primary
        } else {
            c.on_secondary
        };

        let inactive_on_color = if !style.is_toggle && style.button_type == ButtonType::Filled {
            c.on_primary
        } else if style.button_type == ButtonType::Text {
            c.primary
        } else if style.button_type == ButtonType::Filled {
            c.on_surface
        } else {
            c.on_secondary_container
        };

        let bg_color = ButtonBase::calculate_bg(style, c, active_color, inactive_color);
        let on_color = ButtonBase::calculate_on_colour(style, c, active_on_color, inactive_on_color);

        let border_color = if style.is_focused {
            Some(c.primary)
        } else {
            None
        };

        let container = ButtonBase::container(style, bg_color, radius, border_color)
            .h(px(height_px))
            .px(px(PADDING_MEDIUM))
            .py(px(PADDING_SMALL));

        container
            .child(StateLayer::overlay(
                style.is_hovered,
                style.is_pressed,
                style.is_disabled,
                radius,
                on_color,
            ))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(SPACING_SMALL))
                    .child(MaterialIcon::with_size(icon, IconSize::Small, on_color))
                    .child(
                        div()
                            .text_size(px(TypographyScale::LabelLarge.font_size()))
                            .font_weight(TypographyScale::LabelLarge.default_weight())
                            .text_color(rgba(on_color))
                            .child(text.into()),
                    ),
            )
    }
}

impl IntoElement for IconTextButton {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        let on_color = self.colours.as_ref().map(|c| c.on_surface).unwrap_or(0xffffffff);
        let bg_color = self.colours.as_ref().map(|c| c.surface_container_low).unwrap_or(0x202020ff);
        let hover_bg = self.colours.as_ref().map(|c| c.surface_container_high).unwrap_or(0xffffff15);

        let mut d = div()
            .flex()
            .items_center()
            .gap(px(SPACING_SMALL))
            .h(px(36.0))
            .px(px(PADDING_MEDIUM))
            .py(px(PADDING_SMALL))
            .rounded(px(18.0))
            .bg(rgba(bg_color));

        if self.disabled {
            d = d.opacity(0.38).cursor_not_allowed();
        } else {
            d = d.cursor_pointer().hover(move |s| s.bg(rgba(hover_bg)));
        }

        d.child(MaterialIcon::with_size(self.icon, IconSize::Small, on_color))
            .child(
                div()
                    .text_size(px(TypographyScale::LabelLarge.font_size()))
                    .font_weight(TypographyScale::LabelLarge.default_weight())
                    .text_color(rgba(on_color))
                    .child(self.text),
            )
    }
}
