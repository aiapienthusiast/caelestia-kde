//! Popout Card Wrapper
//! Parity with `shell/modules/bar/popouts/Wrapper.qml` and `Content.qml`.

use crate::components::controls::button_base::{ButtonStyle, ButtonType};
use crate::components::controls::icon_button::IconButton;
use crate::components::effects::elevation::{Elevation, ElevationLevel};
use crate::components::effects::inner_border::InnerBorder;
use crate::components::icon::{IconSize, MaterialIcon};
use crate::components::styled_text::TypographyScale;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, Div, FontWeight, Styled};

pub struct PopoutWrapper;

impl PopoutWrapper {
    /// Renders the floating popup card container with header, elevation, inner border, and content.
    pub fn render(
        title: &str,
        icon: &str,
        center_x: f32,
        content: Div,
        c: &CaelestiaColours,
    ) -> Div {
        let width = 360.0;
        let left_px = (center_x - (width / 2.0)).max(16.0);

        let mut card = div()
            .absolute()
            .top(px(48.0))
            .left(px(left_px))
            .w(px(width))
            .rounded(px(ROUNDING_LARGE))
            .bg(rgba(c.surface_container))
            .border_1()
            .border_color(rgba(c.outline_variant))
            .p(px(SPACING_MEDIUM))
            .flex()
            .flex_col()
            .gap(px(SPACING_SMALL))
            .child(InnerBorder::overlay(ROUNDING_LARGE, c));

        card = Elevation::apply(card, ElevationLevel::Level3);

        // Header line
        let header = div()
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .w_full()
            .pb(px(SPACING_SMALL))
            .border_b_1()
            .border_color(rgba(c.outline_variant))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(SPACING_SMALL))
                    .child(MaterialIcon::with_size(icon, IconSize::Custom(20.0), c.primary))
                    .child(
                        div()
                            .text_size(px(TypographyScale::TitleMedium.font_size()))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgba(c.on_surface))
                            .child(title.to_string()),
                    ),
            )
            .child(
                div().child(IconButton::render(
                    "close",
                    &ButtonStyle {
                        button_type: ButtonType::Text,
                        ..Default::default()
                    },
                    c,
                    Some(24.0),
                )),
            );

        card.child(header).child(content)
    }
}
