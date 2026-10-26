//! Circular determinate progress arc matching `shell/components/controls/CircularProgress.qml`.

use gpui::*;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;

#[derive(Default)]
pub struct CircularProgress {
    pub value: f32,
    pub start_angle: i32,
    pub sweep_angle: i32,
    pub stroke_width: f32,
    pub size: f32,
    pub has_end_indicator: bool,
    pub wavy: bool,
    pub fg_color: Option<u32>,
    pub bg_color: Option<u32>,
    pub child: Option<AnyElement>,
}

impl std::fmt::Debug for CircularProgress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CircularProgress")
            .field("value", &self.value)
            .field("start_angle", &self.start_angle)
            .field("sweep_angle", &self.sweep_angle)
            .field("stroke_width", &self.stroke_width)
            .field("size", &self.size)
            .field("has_end_indicator", &self.has_end_indicator)
            .field("wavy", &self.wavy)
            .field("fg_color", &self.fg_color)
            .field("bg_color", &self.bg_color)
            .finish()
    }
}

impl PartialEq for CircularProgress {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
            && self.start_angle == other.start_angle
            && self.sweep_angle == other.sweep_angle
            && self.stroke_width == other.stroke_width
            && self.size == other.size
            && self.has_end_indicator == other.has_end_indicator
            && self.wavy == other.wavy
            && self.fg_color == other.fg_color
            && self.bg_color == other.bg_color
    }
}

impl CircularProgress {
    pub fn new(value: f32) -> Self {
        Self {
            value: value.clamp(0.0, 1.0),
            start_angle: -90,
            sweep_angle: 360,
            stroke_width: PADDING_SMALL,
            size: 64.0,
            has_end_indicator: false,
            wavy: false,
            fg_color: None,
            bg_color: None,
            child: None,
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

    pub fn with_colors(mut self, fg: u32, bg: u32) -> Self {
        self.fg_color = Some(fg);
        self.bg_color = Some(bg);
        self
    }

    pub fn with_child(mut self, child: impl IntoElement) -> Self {
        self.child = Some(child.into_any_element());
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
        let fg = rgba(self.fg_color.unwrap_or(colours.primary));
        let bg = rgba(self.bg_color.unwrap_or(colours.secondary_container));

        let size = self.size;
        let stroke = self.stroke_width;
        let pct = (self.clamped_value() * 100.0).round() as u32;

        let mut root = div()
            .w(px(size))
            .h(px(size))
            .rounded_full()
            .border(px(stroke))
            .border_color(bg)
            .flex()
            .items_center()
            .justify_center();

        if let Some(child) = self.child {
            root = root.child(child);
        } else {
            root = root.child(
                div()
                    .text_xs()
                    .font_weight(FontWeight::BOLD)
                    .text_color(fg)
                    .child(format!("{}%", pct)),
            );
        }

        root
    }
}
