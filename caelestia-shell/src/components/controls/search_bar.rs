//! SearchBar
//! Parity with `shell/components/controls/SearchBar.qml`.

use crate::components::controls::button_base::{ButtonStyle, ButtonType};
use crate::components::controls::icon_button::IconButton;
use crate::components::icon::{IconSize, MaterialIcon};
use crate::components::styled_text::TypographyScale;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, CursorStyle, Div, FontWeight, SharedString, Styled};

pub struct SearchBar;

impl SearchBar {
    /// Renders a Material 3 pill-shaped search bar with leading search icon,
    /// placeholder/query text, and optional trailing clear button.
    pub fn render(
        query: impl Into<SharedString>,
        placeholder: impl Into<SharedString>,
        is_focused: bool,
        is_loading: bool,
        is_disabled: bool,
        c: &CaelestiaColours,
    ) -> Div {
        let query_str = query.into();
        let placeholder_str = placeholder.into();
        let has_text = !query_str.is_empty();

        let border_color = if is_focused {
            c.primary
        } else {
            c.outline_variant
        };

        let mut bar = div()
            .relative()
            .flex()
            .flex_row()
            .items_center()
            .w_full()
            .h(px(44.0))
            .px(px(SPACING_MEDIUM))
            .rounded(px(ROUNDING_FULL))
            .bg(rgba(c.surface_container_high))
            .border_1()
            .border_color(rgba(border_color));

        if !is_disabled {
            bar = bar.cursor(CursorStyle::IBeam);
        }

        // Leading Search Icon
        bar = bar.child(
            div()
                .mr(px(SPACING_SMALL))
                .flex()
                .items_center()
                .justify_center()
                .child(MaterialIcon::with_size(
                    "search",
                    IconSize::Custom(20.0),
                    if is_focused { c.primary } else { c.on_surface_variant },
                )),
        );

        // Text area or Placeholder
        let text_element = if has_text {
            div()
                .flex_grow(1.0)
                .text_size(px(TypographyScale::BodyLarge.font_size()))
                .font_weight(FontWeight::NORMAL)
                .text_color(rgba(if is_disabled {
                    c.disabled_content
                } else {
                    c.on_surface
                }))
                .child(query_str)
        } else {
            div()
                .flex_grow(1.0)
                .text_size(px(TypographyScale::BodyLarge.font_size()))
                .font_weight(FontWeight::NORMAL)
                .text_color(rgba(c.on_surface_variant))
                .child(placeholder_str)
        };

        bar = bar.child(text_element);

        // Trailing Clear Button or Loading Indicator
        if is_loading {
            bar = bar.child(
                div()
                    .ml(px(SPACING_SMALL))
                    .child(crate::components::controls::loading::LoadingIndicator::render(
                        0.5,
                        Some(18.0),
                        c,
                    )),
            );
        } else if has_text && !is_disabled {
            let clear_style = ButtonStyle {
                button_type: ButtonType::Text,
                ..Default::default()
            };
            bar = bar.child(
                div()
                    .ml(px(SPACING_SMALL))
                    .child(IconButton::render(
                        "close",
                        &clear_style,
                        c,
                        Some(28.0),
                    )),
            );
        }

        bar
    }
}
