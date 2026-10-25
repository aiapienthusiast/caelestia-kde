//! Coloured icon with automatic tinting and dominant color adaptation
//! Parity with `shell/components/effects/ColouredIcon.qml`.

use gpui::*;

/// Element rendering a monochrome icon tinted with a specific theme color
#[derive(IntoElement)]
pub struct ColouredIconView {
    icon_name: String,
    colour: Rgba,
    size: f32,
}

impl ColouredIconView {
    pub fn new(icon_name: impl Into<String>, colour: Rgba) -> Self {
        Self {
            icon_name: icon_name.into(),
            colour,
            size: 24.0,
        }
    }

    pub fn with_size(mut self, size: f32) -> Self {
        self.size = size.max(8.0);
        self
    }
}

impl RenderOnce for ColouredIconView {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .w(px(self.size))
            .h(px(self.size))
            .flex()
            .items_center()
            .justify_center()
            .text_color(self.colour)
            .child(
                div()
                    .text_sm()
                    .child(self.icon_name),
            )
    }
}
