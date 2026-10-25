//! StepperRow
//! Parity with `shell/components/controls/CustomSpinBox.qml` and `modules/nexus/common/StepperRow.qml`.

use crate::components::controls::button_base::{ButtonStyle, ButtonType};
use crate::components::controls::icon_button::IconButton;
use crate::components::controls::list_row::ListRow;
use crate::components::styled_text::TypographyScale;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, Div, FontWeight, SharedString, Styled};

pub struct StepperRow;

impl StepperRow {
    /// Renders a numeric stepper row with title, description, current value readout, and +/- buttons.
    pub fn render(
        title: impl Into<SharedString>,
        description: Option<impl Into<SharedString>>,
        value_text: impl Into<SharedString>,
        leading_icon: Option<&'static str>,
        is_hovered: bool,
        is_pressed: bool,
        is_disabled: bool,
        c: &CaelestiaColours,
    ) -> Div {
        let btn_style = ButtonStyle {
            button_type: ButtonType::Text,
            is_disabled,
            ..Default::default()
        };

        let controls = div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(SPACING_SMALL))
            .child(IconButton::render(
                "remove",
                &btn_style,
                c,
                Some(28.0),
            ))
            .child(
                div()
                    .min_w(px(36.0))
                    .text_align(gpui::TextAlign::Center)
                    .text_size(px(TypographyScale::LabelMedium.font_size()))
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(rgba(if is_disabled {
                        c.disabled_content
                    } else {
                        c.primary
                    }))
                    .child(value_text.into()),
            )
            .child(IconButton::render(
                "add",
                &btn_style,
                c,
                Some(28.0),
            ));

        ListRow::render(
            title,
            description,
            leading_icon,
            Some(controls),
            is_hovered,
            is_pressed,
            is_disabled,
            c,
        )
    }
}
