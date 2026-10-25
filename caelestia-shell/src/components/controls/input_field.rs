use gpui::*;

use crate::components::styled_text::StyledText;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum InputTextAlignment {
    Left,
    #[default]
    Center,
    Right,
}

/// Outlined/filled single-line text input field (port of `StyledInputField.qml`).
#[derive(Clone)]
pub struct StyledInputField {
    pub text: String,
    pub placeholder_text: String,
    pub read_only: bool,
    pub enabled: bool,
    pub focused: bool,
    pub hovered: bool,
    pub alignment: InputTextAlignment,
    pub colours: Option<CaelestiaColours>,
}

impl StyledInputField {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            placeholder_text: String::new(),
            read_only: false,
            enabled: true,
            focused: false,
            hovered: false,
            alignment: InputTextAlignment::Center,
            colours: None,
        }
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder_text = placeholder.into();
        self
    }

    pub fn read_only(mut self, read_only: bool) -> Self {
        self.read_only = read_only;
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    pub fn hovered(mut self, hovered: bool) -> Self {
        self.hovered = hovered;
        self
    }

    pub fn alignment(mut self, alignment: InputTextAlignment) -> Self {
        self.alignment = alignment;
        self
    }

    pub fn with_colours(mut self, colours: CaelestiaColours) -> Self {
        self.colours = Some(colours);
        self
    }
}

impl Default for StyledInputField {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoElement for StyledInputField {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        let colours = self.colours.unwrap_or_default();

        let bg_color = rgba(if self.hovered || self.focused {
            colours.surface_container_high
        } else {
            colours.surface_container
        });

        let border_color = if self.focused {
            rgba(colours.primary)
        } else {
            let mut c = rgba(colours.outline);
            c.a *= 0.3;
            c
        };

        let text_color = rgba(if self.text.is_empty() {
            colours.on_surface_variant
        } else {
            colours.on_surface
        });

        let display_text = if self.text.is_empty() {
            self.placeholder_text
        } else {
            self.text
        };

        let radius = ROUNDING_MEDIUM;
        let padding_h = PADDING_MEDIUM;
        let padding_v = PADDING_SMALL;

        let mut container = div()
            .min_w(px(70.0))
            .h(px(32.0))
            .px(px(padding_h))
            .py(px(padding_v))
            .bg(bg_color)
            .rounded(px(radius))
            .border_1()
            .border_color(border_color)
            .opacity(if self.enabled { 1.0 } else { 0.5 })
            .flex()
            .flex_row()
            .items_center();

        container = match self.alignment {
            InputTextAlignment::Left => container.justify_start(),
            InputTextAlignment::Center => container.justify_center(),
            InputTextAlignment::Right => container.justify_end(),
        };

        container.child(
            StyledText::new(display_text)
                .text_color(text_color)
                .font_weight(FontWeight::NORMAL),
        )
    }
}
