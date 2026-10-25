//! Visualizer Bars Widget
//! Parity with `shell/plugin/src/Caelestia/Components/visualiserbars.cpp`.
//! Renders audio frequency spectrum bars with smooth rounded caps and dynamic animation.

use gpui::{div, prelude::*, px, rgba, Div, Styled};

pub struct VisualizerBars;

impl VisualizerBars {
    /// Render animated audio visualizer spectrum bars
    pub fn render(
        bar_count: usize,
        anim_phase: f32,
        is_playing: bool,
        color: u32,
        max_height: f32,
    ) -> Div {
        let mut row = div()
            .flex()
            .items_end()
            .justify_center()
            .h(px(max_height))
            .gap(px(3.0));

        for i in 0..bar_count {
            let offset = i as f32 * 0.35;
            let magnitude = if is_playing {
                let wave1 = ((anim_phase * 2.0 + offset).sin() + 1.0) * 0.5;
                let wave2 = ((anim_phase * 3.5 - offset * 1.5).cos() + 1.0) * 0.5;
                let combined = (wave1 * 0.6 + wave2 * 0.4).clamp(0.08, 1.0);
                combined * max_height
            } else {
                3.0 // Idle resting indicator height
            };

            row = row.child(
                div()
                    .w(px(3.5))
                    .h(px(magnitude.max(3.0)))
                    .rounded_full()
                    .bg(rgba(color)),
            );
        }

        row
    }
}
