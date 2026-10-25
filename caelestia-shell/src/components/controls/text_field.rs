//! StyledTextField & StyledInputField
//! Parity with `shell/components/controls/StyledTextField.qml` and `StyledInputField.qml`.

use crate::components::icon::{IconSize, MaterialIcon};
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, CursorStyle, Div, SharedString, Styled};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TextFieldType {
    Outlined,
    Filled,
}

pub struct StyledTextField;

impl StyledTextField {
    /// Renders an M3 text input field matching StyledTextField.qml.
    pub fn render(
        text: impl Into<SharedString>,
        placeholder: impl Into<SharedString>,
        leading_icon: Option<&str>,
        trailing_icon: Option<&str>,
        field_type: TextFieldType,
        is_focused: bool,
        is_error: bool,
        is_disabled: bool,
        c: &CaelestiaColours,
    ) -> Div {
        let text_str: SharedString = text.into();
        let placeholder_str: SharedString = placeholder.into();

        let border_color = if is_error {
            c.error
        } else if is_focused {
            c.primary
        } else {
            c.outline_variant
        };

        let bg_color = match field_type {
            TextFieldType::Filled => c.surface_container_highest,
            TextFieldType::Outlined => c.surface_container_lowest & 0xffffff00, // transparent
        };

        let content_color = if is_disabled {
            c.disabled_content
        } else {
            c.on_surface
        };

        let mut field = div()
            .relative()
            .w_full()
            .h(px(48.0))
            .rounded(px(ROUNDING_SMALL))
            .bg(rgba(bg_color))
            .border_1()
            .border_color(rgba(border_color))
            .px(px(PADDING_MEDIUM))
            .flex()
            .items_center()
            .gap(px(SPACING_SMALL));

        if !is_disabled {
            field = field.cursor(CursorStyle::IBeam);
        }

        // Leading icon if present
        if let Some(icon) = leading_icon {
            field = field.child(MaterialIcon::with_size(icon, IconSize::Small, c.on_surface_variant));
        }

        // Text content or placeholder
        if text_str.is_empty() {
            field = field.child(
                div()
                    .flex_1()
                    .text_size(px(FONT_SIZE_NORMAL))
                    .text_color(rgba(c.on_surface_variant))
                    .child(placeholder_str),
            );
        } else {
            field = field.child(
                div()
                    .flex_1()
                    .text_size(px(FONT_SIZE_NORMAL))
                    .text_color(rgba(content_color))
                    .child(text_str),
            );
        }

        // Trailing icon if present
        if let Some(icon) = trailing_icon {
            field = field.child(MaterialIcon::with_size(icon, IconSize::Small, c.on_surface_variant));
        }

        field
    }
}
