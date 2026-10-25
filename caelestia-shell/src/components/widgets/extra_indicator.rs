//! Badge overflow indicator pill (+N)
//! Parity with `shell/components/widgets/ExtraIndicator.qml`.

use gpui::prelude::FluentBuilder;
use gpui::*;

/// Extra count badge element
#[derive(IntoElement)]
pub struct ExtraIndicatorView {
    extra: i32,
    radius: f32,
}

impl ExtraIndicatorView {
    pub fn new(extra: i32) -> Self {
        Self {
            extra,
            radius: 12.0,
        }
    }

    pub fn with_radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }
}

impl RenderOnce for ExtraIndicatorView {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let is_visible = self.extra > 0;

        div()
            .when(!is_visible, |d| d)
            .when(is_visible, |d| {
                d.px_2()
                    .py_0p5()
                    .rounded(px(self.radius))
                    .bg(rgb(0x7d5260))
                    .text_color(rgb(0xffd8e4))
                    .text_xs()
                    .font_weight(FontWeight::BOLD)
                    .shadow_sm()
                    .child(format!("+{}", self.extra))
            })
    }
}
