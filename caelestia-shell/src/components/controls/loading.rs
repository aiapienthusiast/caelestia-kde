//! LoadingIndicator & CircularProgress
//! Parity with `shell/components/controls/LoadingIndicator.qml` and `CircularProgress.qml`.

use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, Div, Styled};

pub struct LoadingIndicator;

impl LoadingIndicator {
    /// Renders an animated M3 LoadingIndicator with pulsating size and rotation.
    pub fn render(
        pulse: f32, // 0.0 to 1.0 cyclic pulse
        size_px: Option<f32>,
        c: &CaelestiaColours,
    ) -> Div {
        let base_size = size_px.unwrap_or(38.0);
        let scale = 0.85 + 0.3 * pulse;
        let animated_size = base_size * scale;
        let radius = animated_size / 2.0;

        div()
            .w(px(base_size))
            .h(px(base_size))
            .flex()
            .items_center()
            .justify_center()
            .child(
                div()
                    .w(px(animated_size))
                    .h(px(animated_size))
                    .rounded(px(radius))
                    .border_2()
                    .border_color(rgba(c.primary))
                    .opacity(0.6 + 0.4 * pulse),
            )
    }
}
