use gpui::*;

use crate::components::styled_text::StyledText;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TextEchoMode {
    #[default]
    Normal,
    Password,
    NoEcho,
}

/// Base text field primitives with cursor management and selection tokens (port of `TextFieldBase.qml`).
#[derive(Clone)]
pub struct TextFieldBase {
    pub text: String,
    pub placeholder_text: String,
    pub echo_mode: TextEchoMode,
    pub read_only: bool,
    pub cursor_visible: bool,
    pub cursor_position: usize,
    pub cursor_blink: bool,
    pub active_focus: bool,
    pub colours: Option<CaelestiaColours>,
}

impl TextFieldBase {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            placeholder_text: String::new(),
            echo_mode: TextEchoMode::Normal,
            read_only: false,
            cursor_visible: true,
            cursor_position: 0,
            cursor_blink: true,
            active_focus: false,
            colours: None,
        }
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        let t = text.into();
        self.cursor_position = t.len();
        self.text = t;
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder_text = placeholder.into();
        self
    }

    pub fn echo_mode(mut self, mode: TextEchoMode) -> Self {
        self.echo_mode = mode;
        self
    }

    pub fn read_only(mut self, read_only: bool) -> Self {
        self.read_only = read_only;
        self.cursor_visible = !read_only;
        self
    }

    pub fn active_focus(mut self, focus: bool) -> Self {
        self.active_focus = focus;
        self
    }

    pub fn with_colours(mut self, colours: CaelestiaColours) -> Self {
        self.colours = Some(colours);
        self
    }

    pub fn display_text(&self) -> String {
        match self.echo_mode {
            TextEchoMode::Normal => self.text.clone(),
            TextEchoMode::Password => "•".repeat(self.text.chars().count()),
            TextEchoMode::NoEcho => String::new(),
        }
    }
}

impl Default for TextFieldBase {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoElement for TextFieldBase {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        let display = self.display_text();
        let is_empty = self.text.is_empty();
        let colours = self.colours.unwrap_or_default();

        let text_color = rgba(colours.on_surface);
        let placeholder_color = rgba(colours.on_surface_variant);
        let cursor_color = rgba(colours.primary);

        let mut row = div()
            .flex()
            .flex_row()
            .items_center()
            .text_size(px(FONT_SIZE_SMALL));

        if is_empty {
            row = row.child(
                StyledText::new(self.placeholder_text)
                    .text_color(placeholder_color),
            );
        } else {
            row = row.child(
                StyledText::new(display)
                    .text_color(text_color),
            );
        }

        // Render animated cursor if focused and cursor visible
        if self.active_focus && self.cursor_visible && self.cursor_blink {
            row = row.child(
                div()
                    .w(px(1.5))
                    .h(px(16.0))
                    .bg(cursor_color)
                    .rounded(px(ROUNDING_LARGE))
                    .ml(px(1.0)),
            );
        }

        row
    }
}
