//! Filled Slider matching `shell/components/controls/FilledSlider.qml`.
//! Continuous filled slider track with secondary accent and elevated inverse-surface handle.

use gpui::*;
use crate::components::icon::MaterialIcon;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;

/// Orientation of the filled slider.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SliderOrientation {
    #[default]
    Vertical,
    Horizontal,
}

/// Filled Slider model and element.
#[derive(Clone, Debug, PartialEq)]
pub struct FilledSlider {
    pub value: f32,
    pub icon: String,
    pub orientation: SliderOrientation,
    pub width: f32,
    pub length: f32,
    pub is_hovered: bool,
    pub is_dragging: bool,
}

impl Default for FilledSlider {
    fn default() -> Self {
        Self {
            value: 0.5,
            icon: "volume_up".to_string(),
            orientation: SliderOrientation::Vertical,
            width: 44.0,
            length: 160.0,
            is_hovered: false,
            is_dragging: false,
        }
    }
}

impl FilledSlider {
    pub fn new(value: f32, icon: impl Into<String>) -> Self {
        Self {
            value: value.clamp(0.0, 1.0),
            icon: icon.into(),
            ..Default::default()
        }
    }

    pub fn with_orientation(mut self, orientation: SliderOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn with_dimensions(mut self, width: f32, length: f32) -> Self {
        self.width = width;
        self.length = length;
        self
    }

    /// Progress percentage from 0 to 100.
    pub fn progress_percent(&self) -> u32 {
        (self.value.clamp(0.0, 1.0) * 100.0).round() as u32
    }
}

impl IntoElement for FilledSlider {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        let colours = CaelestiaColours::default();
        let track_bg = rgba(colours.surface_container_high);
        let fill_color = rgba(colours.secondary);
        let handle_bg = rgba(colours.inverse_surface);
        let handle_fg = rgba(colours.inverse_on_surface);

        let w = self.width;
        let l = self.length;
        let val = self.value.clamp(0.0, 1.0);

        match self.orientation {
            SliderOrientation::Vertical => {
                let fill_h = l * val;
                div()
                    .relative()
                    .w(px(w))
                    .h(px(l))
                    .rounded(px(ROUNDING_FULL))
                    .bg(track_bg)
                    .overflow_hidden()
                    .child(
                        // Fill bar from bottom
                        div()
                            .absolute()
                            .left_0()
                            .right_0()
                            .bottom_0()
                            .h(px(fill_h))
                            .rounded(px(ROUNDING_FULL))
                            .bg(fill_color),
                    )
                    .child(
                        // Handle icon at current position
                        div()
                            .absolute()
                            .left_0()
                            .right_0()
                            .bottom(px((l - w) * val))
                            .h(px(w))
                            .rounded_full()
                            .bg(handle_bg)
                            .text_color(handle_fg)
                            .shadow_md()
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(MaterialIcon::new(self.icon.clone()).text_size(px(ICON_SIZE_SMALL))),
                    )
            }
            SliderOrientation::Horizontal => {
                let fill_w = l * val;
                div()
                    .relative()
                    .w(px(l))
                    .h(px(w))
                    .rounded(px(ROUNDING_FULL))
                    .bg(track_bg)
                    .overflow_hidden()
                    .child(
                        // Fill bar from left
                        div()
                            .absolute()
                            .top_0()
                            .bottom_0()
                            .left_0()
                            .w(px(fill_w))
                            .rounded(px(ROUNDING_FULL))
                            .bg(fill_color),
                    )
                    .child(
                        // Handle icon at current position
                        div()
                            .absolute()
                            .top_0()
                            .bottom_0()
                            .left(px((l - w) * val))
                            .w(px(w))
                            .rounded_full()
                            .bg(handle_bg)
                            .text_color(handle_fg)
                            .shadow_md()
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(MaterialIcon::new(self.icon.clone()).text_size(px(ICON_SIZE_SMALL))),
                    )
            }
        }
    }
}
