//! Sparkline Widget
//! Parity with `shell/plugin/src/Caelestia/Components/sparklineitem.cpp`.
//! Renders hardware metrics (CPU, RAM, Network) as a smooth sampled bar/line series.

use gpui::{div, prelude::*, px, rgba, Div, Styled};
use std::collections::VecDeque;

pub struct Sparkline;

impl Sparkline {
    /// Render a sparkline bar-series for given sample deque (values 0.0 to 1.0)
    pub fn render(samples: &VecDeque<f32>, color: u32, height_px: f32) -> Div {
        let mut row = div()
            .flex()
            .items_end()
            .h(px(height_px))
            .gap(px(2.0))
            .overflow_hidden();

        let count = samples.len();
        for (i, &val) in samples.iter().enumerate() {
            // Calculate opacity fade for older samples
            let alpha = 0.35 + 0.65 * (i as f32 / count.max(1) as f32);
            let bar_h = (val * height_px).max(2.0);

            // Extract RGB and apply sample alpha
            let r = (color >> 24) & 0xFF;
            let g = (color >> 16) & 0xFF;
            let b = (color >> 8) & 0xFF;
            let base_a = (color & 0xFF) as f32 / 255.0;
            let final_a = (base_a * alpha).clamp(0.0, 1.0);
            let final_color =
                ((r as u32) << 24) | ((g as u32) << 16) | ((b as u32) << 8) | ((final_a * 255.0) as u32);

            row = row.child(
                div()
                    .w(px(3.0))
                    .h(px(bar_h))
                    .rounded_t(px(1.5))
                    .bg(rgba(final_color)),
            );
        }

        row
    }
}
