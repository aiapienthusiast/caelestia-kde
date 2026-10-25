//! SliderRow
//! Parity with `shell/components/controls/SliderRow.qml`.

use crate::components::controls::slider::StyledSlider;
use crate::components::icon::{IconSize, MaterialIcon};
use crate::components::styled_text::TypographyScale;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, Div, FontWeight, SharedString, Styled};

pub struct SliderRow;

impl SliderRow {
    /// Renders a settings row containing a title, current value readout, and continuous slider.
    pub fn render(
        title: impl Into<SharedString>,
        value_text: impl Into<SharedString>,
        value: f32, // 0.0 to 1.0
        slider_width: f32,
        leading_icon: Option<&'static str>,
        is_hovered: bool,
        is_dragging: bool,
        is_disabled: bool,
        c: &CaelestiaColours,
    ) -> Div {
        let mut row = div()
            .relative()
            .flex()
            .flex_col()
            .w_full()
            .px(px(SPACING_MEDIUM))
            .py(px(SPACING_SMALL))
            .rounded(px(ROUNDING_MEDIUM))
            .bg(rgba(c.surface_container_low));

        // Top line: [Icon] Title ......... Value
        let mut top_line = div().flex().flex_row().items_center().w_full();

        if let Some(icon_name) = leading_icon {
            top_line = top_line.child(
                div()
                    .mr(px(SPACING_MEDIUM))
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

        top_line = top_line.child(
            div()
                .flex_grow(1.0)
                .text_size(px(TypographyScale::BodyLarge.font_size()))
                .font_weight(FontWeight::NORMAL)
                .text_color(rgba(if is_disabled {
                    c.disabled_content
                } else {
                    c.on_surface
                }))
                .child(title.into()),
        );

        top_line = top_line.child(
            div()
                .text_size(px(TypographyScale::LabelMedium.font_size()))
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgba(if is_disabled {
                    c.disabled_content
                } else {
                    c.primary
                }))
                .child(value_text.into()),
        );

        row = row.child(top_line);

        // Slider line
        let slider_widget = StyledSlider::render(
            value,
            slider_width,
            is_hovered,
            is_dragging,
            is_disabled,
            c,
        );

        row = row.child(div().mt(px(SPACING_SMALL)).w_full().child(slider_widget));

        row
    }
}
