//! SelectRow
//! Parity with `modules/nexus/common/SelectRow.qml`.

use crate::components::controls::list_row::ListRow;
use crate::components::icon::{IconSize, MaterialIcon};
use crate::components::styled_text::TypographyScale;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, Div, FontWeight, SharedString, Styled};

pub struct SelectRow;

impl SelectRow {
    /// Renders a dropdown selection row with title, description, currently selected value, and chevron.
    pub fn render(
        title: impl Into<SharedString>,
        description: Option<impl Into<SharedString>>,
        selected_label: impl Into<SharedString>,
        leading_icon: Option<&'static str>,
        is_hovered: bool,
        is_pressed: bool,
        is_disabled: bool,
        c: &CaelestiaColours,
    ) -> Div {
        let trailing = div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(SPACING_SMALL))
            .child(
                div()
                    .text_size(px(TypographyScale::LabelMedium.font_size()))
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(rgba(if is_disabled {
                        c.disabled_content
                    } else {
                        c.primary
                    }))
                    .child(selected_label.into()),
            )
            .child(MaterialIcon::with_size(
                "expand_more",
                IconSize::Custom(18.0),
                if is_disabled { c.disabled_content } else { c.on_surface_variant },
            ));

        ListRow::render(
            title,
            description,
            leading_icon,
            Some(trailing),
            is_hovered,
            is_pressed,
            is_disabled,
            c,
        )
    }
}
