//! SectionHeader
//! Parity with `shell/components/SectionHeader.qml`.

use crate::components::styled_text::TypographyScale;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, Div, FontWeight, SharedString, Styled};

pub struct SectionHeader;

impl SectionHeader {
    /// Renders a SectionHeader with title and optional description matching SectionHeader.qml.
    pub fn render(
        title: impl Into<SharedString>,
        description: Option<impl Into<SharedString>>,
        c: &CaelestiaColours,
    ) -> Div {
        let mut col = div()
            .flex()
            .flex_col()
            .mt(px(SPACING_LARGE_INCREASED))
            .child(
                div()
                    .text_size(px(TypographyScale::TitleMedium.font_size()))
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(rgba(c.on_surface))
                    .child(title.into()),
            );

        if let Some(desc) = description {
            col = col.child(
                div()
                    .text_size(px(TypographyScale::BodySmall.font_size()))
                    .font_weight(TypographyScale::BodySmall.default_weight())
                    .text_color(rgba(c.on_surface_variant))
                    .child(desc.into()),
            );
        }

        col
    }
}
