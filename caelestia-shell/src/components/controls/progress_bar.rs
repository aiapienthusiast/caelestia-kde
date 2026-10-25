//! StyledProgressBar
//! Parity with `shell/components/controls/StyledProgressBar.qml`.

use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, Div, Styled};

pub struct StyledProgressBar;

impl StyledProgressBar {
    /// Renders a determinate M3 ProgressBar.
    pub fn determinate(
        progress: f32, // 0.0 to 1.0
        width_px: f32,
        height_px: Option<f32>,
        c: &CaelestiaColours,
    ) -> Div {
        let h = height_px.unwrap_or(PROGRESS_BAR_HEIGHT);
        let filled_w = (progress.clamp(0.0, 1.0) * width_px).max(0.0);

        div()
            .relative()
            .w(px(width_px))
            .h(px(h))
            .rounded(px(ROUNDING_FULL))
            .bg(rgba(c.secondary_container))
            .overflow_hidden()
            .child(
                div()
                    .absolute()
                    .top_0()
                    .bottom_0()
                    .left_0()
                    .w(px(filled_w))
                    .rounded(px(ROUNDING_FULL))
                    .bg(rgba(c.primary)),
            )
    }

    /// Renders an indeterminate M3 ProgressBar with an animated indicator segment.
    pub fn indeterminate(
        animation_phase: f32, // 0.0 to 1.0 cyclic
        width_px: f32,
        height_px: Option<f32>,
        c: &CaelestiaColours,
    ) -> Div {
        let h = height_px.unwrap_or(PROGRESS_BAR_HEIGHT);
        let segment_width = width_px * 0.35;
        let left_offset = (animation_phase.fract() * (width_px + segment_width)) - segment_width;

        div()
            .relative()
            .w(px(width_px))
            .h(px(h))
            .rounded(px(ROUNDING_FULL))
            .bg(rgba(c.secondary_container))
            .overflow_hidden()
            .child(
                div()
                    .absolute()
                    .top_0()
                    .bottom_0()
                    .left(px(left_offset))
                    .w(px(segment_width))
                    .rounded(px(ROUNDING_FULL))
                    .bg(rgba(c.primary)),
            )
    }
}
