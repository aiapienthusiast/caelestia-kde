//! Circular busy indicator matching `shell/components/controls/CircularIndicator.qml`.

use gpui::*;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;

/// Animation mode matching CircularIndicator.qml.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum IndicatorAnimType {
    #[default]
    Advance,
    Retreat,
}

/// Animation lifecycle state.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum IndicatorAnimState {
    Stopped,
    #[default]
    Running,
    Completing,
}

/// Indeterminate radial busy / loading indicator.
#[derive(Clone, Debug, PartialEq)]
pub struct CircularIndicator {
    pub running: bool,
    pub size: f32,
    pub stroke_width: f32,
    pub anim_type: IndicatorAnimType,
    pub anim_state: IndicatorAnimState,
    pub rotation_deg: f32,
}

impl Default for CircularIndicator {
    fn default() -> Self {
        Self {
            running: true,
            size: 48.0,
            stroke_width: PADDING_EXTRA_SMALL,
            anim_type: IndicatorAnimType::Advance,
            anim_state: IndicatorAnimState::Running,
            rotation_deg: 0.0,
        }
    }
}

impl CircularIndicator {
    pub fn new(running: bool) -> Self {
        Self {
            running,
            anim_state: if running {
                IndicatorAnimState::Running
            } else {
                IndicatorAnimState::Stopped
            },
            ..Default::default()
        }
    }

    pub fn with_size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn with_stroke_width(mut self, width: f32) -> Self {
        self.stroke_width = width;
        self
    }

    pub fn step(&mut self, delta_seconds: f32) {
        if self.running {
            self.rotation_deg = (self.rotation_deg + 360.0 * delta_seconds) % 360.0;
        }
    }
}

impl IntoElement for CircularIndicator {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        let colours = CaelestiaColours::default();
        let primary = rgba(colours.primary);
        let track = rgba(colours.secondary_container);

        let size = self.size;
        let stroke = self.stroke_width;

        div()
            .w(px(size))
            .h(px(size))
            .rounded_full()
            .border(px(stroke))
            .border_color(track)
            .flex()
            .items_center()
            .justify_center()
            .child(
                div()
                    .w(px(size - stroke * 2.0))
                    .h(px(size - stroke * 2.0))
                    .rounded_full()
                    .border_t(px(stroke))
                    .border_color(primary),
            )
    }
}
