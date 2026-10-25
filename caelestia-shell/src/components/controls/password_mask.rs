use gpui::*;

use crate::components::styled_text::StyledText;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;

/// Animated password/PIN mask indicator with shape morphing or bullet dots (port of `AnimatedPasswordMask.qml`).
#[derive(Clone)]
pub struct AnimatedPasswordMask {
    pub buffer: String,
    pub show_password: bool,
    pub dot_size: f32,
    pub spacing: f32,
    pub colours: Option<CaelestiaColours>,
}

impl AnimatedPasswordMask {
    pub fn new(buffer: impl Into<String>) -> Self {
        Self {
            buffer: buffer.into(),
            show_password: false,
            dot_size: 10.0,
            spacing: SPACING_EXTRA_SMALL,
            colours: None,
        }
    }

    pub fn buffer(mut self, buffer: impl Into<String>) -> Self {
        self.buffer = buffer.into();
        self
    }

    pub fn show_password(mut self, show: bool) -> Self {
        self.show_password = show;
        self
    }

    pub fn dot_size(mut self, size: f32) -> Self {
        self.dot_size = size;
        self
    }

    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn with_colours(mut self, colours: CaelestiaColours) -> Self {
        self.colours = Some(colours);
        self
    }

    pub fn char_count(&self) -> usize {
        self.buffer.chars().count()
    }

    pub fn full_width(&self) -> f32 {
        let count = self.char_count();
        if count == 0 {
            0.0
        } else {
            (count as f32) * self.dot_size + ((count.saturating_sub(1)) as f32) * self.spacing
        }
    }
}

impl IntoElement for AnimatedPasswordMask {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        let colours = self.colours.unwrap_or_default();
        let dot_color = rgba(colours.on_surface);
        let dot_size = self.dot_size;
        let spacing = self.spacing;
        let show = self.show_password;

        let mut row = div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(spacing))
            .h(px(dot_size * 1.5));

        for ch in self.buffer.chars() {
            if show {
                row = row.child(
                    StyledText::new(ch.to_string())
                        .text_color(dot_color)
                        .font_weight(FontWeight::SEMIBOLD),
                );
            } else {
                row = row.child(
                    div()
                        .size(px(dot_size))
                        .rounded_full()
                        .bg(dot_color)
                        .flex_shrink_0(),
                );
            }
        }

        row
    }
}
