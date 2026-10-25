//! ListRow
//! Parity with `shell/components/controls/ListRow.qml`.

use crate::components::icon::{IconSize, MaterialIcon};
use crate::components::state_layer::StateLayer;
use crate::components::styled_text::TypographyScale;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, CursorStyle, Div, FontWeight, SharedString, Styled};

pub struct ListRow;

impl ListRow {
    /// Renders a Material 3 list row item with optional leading icon, headline, subtitle,
    /// trailing action or widget, and hover/pressed state layer.
    pub fn render(
        headline: impl Into<SharedString>,
        subtitle: Option<impl Into<SharedString>>,
        leading_icon: Option<&'static str>,
        trailing: Option<Div>,
        is_hovered: bool,
        is_pressed: bool,
        is_disabled: bool,
        c: &CaelestiaColours,
    ) -> Div {
        let mut row = div()
            .relative()
            .flex()
            .flex_row()
            .items_center()
            .w_full()
            .min_h(px(48.0))
            .px(px(SPACING_MEDIUM))
            .py(px(SPACING_SMALL))
            .rounded(px(ROUNDING_MEDIUM))
            .bg(rgba(c.surface_container_low));

        if !is_disabled {
            row = row.cursor(CursorStyle::PointingHand);
        }

        // State layer overlay
        row = row.child(StateLayer::overlay(
            is_hovered,
            is_pressed,
            is_disabled,
            ROUNDING_MEDIUM,
            c.on_surface,
        ));

        // Leading Icon
        if let Some(icon_name) = leading_icon {
            row = row.child(
                div()
                    .mr(px(SPACING_MEDIUM))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(MaterialIcon::with_size(
                        icon_name,
                        IconSize::Custom(20.0),
                        if is_disabled { c.disabled_content } else { c.on_surface_variant },
                    )),
            );
        }

        // Text column
        let mut text_col = div()
            .flex()
            .flex_col()
            .flex_grow(1.0)
            .child(
                div()
                    .text_size(px(TypographyScale::BodyLarge.font_size()))
                    .font_weight(FontWeight::NORMAL)
                    .text_color(rgba(if is_disabled {
                        c.disabled_content
                    } else {
                        c.on_surface
                    }))
                    .child(headline.into()),
            );

        if let Some(sub) = subtitle {
            text_col = text_col.child(
                div()
                    .text_size(px(TypographyScale::BodySmall.font_size()))
                    .font_weight(TypographyScale::BodySmall.default_weight())
                    .text_color(rgba(if is_disabled {
                        c.disabled_content
                    } else {
                        c.on_surface_variant
                    }))
                    .child(sub.into()),
            );
        }

        row = row.child(text_col);

        // Trailing widget (switch, badge, button, chevron)
        if let Some(trailing_widget) = trailing {
            row = row.child(
                div()
                    .ml(px(SPACING_MEDIUM))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(trailing_widget),
            );
        }

        row
    }
}
