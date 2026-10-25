//! Media Widget
//! Parity with `shell/modules/bar/components/Media.qml`.

use crate::components::styled_text::StyledTextExt;
use crate::components::widgets::visualizer_bars::VisualizerBars;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, CursorStyle, Div, Styled};

pub struct MediaWidget;

impl MediaWidget {
    /// Renders the compact media pill with audio visualizer spectrum bars.
    pub fn render(
        title: &str,
        is_playing: bool,
        anim_phase: f32,
        c: &CaelestiaColours,
    ) -> Div {
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap_2()
            .px_3()
            .py_1()
            .rounded_full()
            .bg(rgba(c.surface_container_low))
            .cursor(CursorStyle::PointingHand)
            .child(
                div()
                    .text_color(rgba(c.primary))
                    .child(StyledTextExt::icon_size(div().child("music_note"), 14.0)),
            )
            .child(
                div()
                    .text_color(rgba(c.on_surface))
                    .text_xs()
                    .max_w(px(120.0))
                    .overflow_hidden()
                    .child(title.to_string()),
            )
            .child(VisualizerBars::render(
                8,
                anim_phase,
                is_playing,
                c.primary,
                14.0,
            ))
    }
}
