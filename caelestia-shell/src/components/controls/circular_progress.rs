//! Circular determinate progress arc matching `shell/components/controls/CircularProgress.qml`.

use gpui::*;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;

/// Determinate circular progress gauge.
#[derive(Clone, Debug, PartialEq)]
pub struct CircularProgress {
    pub value: f32,
    pub start_angle: i32,
    pub sweep_angle: i32,
    pub stroke_width: f32,
    pub size: f32,
    pub has_end_indicator: bool,
    pub wavy: bool,
}

impl Default for CircularProgress {
    fn default() -> Self {
        Self {
            value: 0.0,
            start_angle: -90,
            sweep_angle: 360,
            stroke_width: PADDING_SMALL,
            size: 64.0,
            has_end_indicator: false,
            wavy: false,
        }
    }
}

impl CircularProgress {
    pub fn new(value: f32) -> Self {
        Self {
            value: value.clamp(0.0, 1.0),
            ..Default::default()
        }
    }

    pub fn with_size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn with_stroke_width(mut self, stroke: f32) -> Self {
        self.stroke_width = stroke;
        self
    }

    pub fn with_end_indicator(mut self, has_indicator: bool) -> Self {
        self.has_end_indicator = has_indicator;
        self
    }

    pub fn with_angles(mut self, start: i32, sweep: i32) -> Self {
        self.start_angle = start;
        self.sweep_angle = sweep;
        self
    }

    pub fn clamped_value(&self) -> f32 {
        self.value.clamp(1.0 / 360.0, 1.0)
    }

    pub fn arc_radius(&self) -> f32 {
        (self.size - self.stroke_width) / 2.0
    }
}

impl IntoElement for CircularProgress {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        let colours = CaelestiaColours::default();
        let fg = rgba(colours.primary);
        let bg = rgba(colours.secondary_container);

        let size = self.size;
        let stroke = self.stroke_width;
        let pct = (self.clamped_value() * 100.0).round() as u32;

        div()
            .w(px(size))
            .h(px(size))
            .rounded_full()
            .border(px(stroke))
            .border_color(bg)
            .flex()
            .items_center()
            .justify_center()
            .child(
                div()
                    .text_xs()
                    .font_weight(FontWeight::BOLD)
                    .text_color(fg)
                    .child(format!("{}%", pct)),
            )
    }
}
