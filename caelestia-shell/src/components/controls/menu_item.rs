//! MenuItem
//! Parity with `shell/components/controls/MenuItem.qml`.

use crate::components::icon::{IconSize, MaterialIcon};
use crate::components::state_layer::StateLayer;
use crate::components::styled_text::TypographyScale;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, CursorStyle, Div, FontWeight, SharedString, Styled};

pub struct MenuItem;

impl MenuItem {
    /// Renders a dropdown menu item with optional leading icon, label text,
    /// optional keyboard shortcut badge or submenu indicator, and hover state.
    pub fn render(
        label: impl Into<SharedString>,
        leading_icon: Option<&'static str>,
        shortcut: Option<impl Into<SharedString>>,
        has_submenu: bool,
        is_hovered: bool,
        is_pressed: bool,
        is_disabled: bool,
        c: &CaelestiaColours,
    ) -> Div {
        let mut item = div()
            .relative()
            .flex()
            .flex_row()
            .items_center()
            .w_full()
            .h(px(36.0))
            .px(px(SPACING_MEDIUM))
            .rounded(px(ROUNDING_SMALL));

        if !is_disabled {
            item = item.cursor(CursorStyle::PointingHand);
        }

        // State layer
        item = item.child(StateLayer::overlay(
            is_hovered,
            is_pressed,
            is_disabled,
            ROUNDING_SMALL,
            c.on_surface,
        ));

        // Leading icon
        if let Some(icon_name) = leading_icon {
            item = item.child(
                div()
                    .mr(px(SPACING_SMALL))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(MaterialIcon::with_size(
                        icon_name,
                        IconSize::Custom(18.0),
                        if is_disabled { c.disabled_content } else { c.on_surface_variant },
                    )),
            );
        }

        // Label
        item = item.child(
            div()
                .flex_grow(1.0)
                .text_size(px(TypographyScale::BodyMedium.font_size()))
                .font_weight(FontWeight::NORMAL)
                .text_color(rgba(if is_disabled {
                    c.disabled_content
                } else {
                    c.on_surface
                }))
                .child(label.into()),
        );

        // Shortcut or Submenu Chevron
        if let Some(sc) = shortcut {
            item = item.child(
                div()
                    .ml(px(SPACING_MEDIUM))
                    .text_size(px(TypographyScale::LabelSmall.font_size()))
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(rgba(if is_disabled {
                        c.disabled_content
                    } else {
                        c.on_surface_variant
                    }))
                    .child(sc.into()),
            );
        } else if has_submenu {
            item = item.child(
                div()
                    .ml(px(SPACING_SMALL))
                    .child(MaterialIcon::with_size(
                        "chevron_right",
                        IconSize::Custom(16.0),
                        if is_disabled { c.disabled_content } else { c.on_surface_variant },
                    )),
            );
        }

        item
    }
}
