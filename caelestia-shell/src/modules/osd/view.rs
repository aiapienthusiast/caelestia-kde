//! OSD View Component
//! Parity with `shell/modules/osd/Content.qml` and `shell/modules/osd/Wrapper.qml`.
//!
//! Renders Material 3 filled vertical sliders for volume, microphone, and brightness
//! with real-time value progress, icons, and smooth interactive feedback.

use super::config::OsdConfig;
use super::state::OsdState;
use crate::components::styled_text::StyledTextExt;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{
    div, prelude::*, px, rgba, CursorStyle, IntoElement, Render, Styled, Window,
};

pub struct OsdView {
    pub config: OsdConfig,
    pub state: OsdState,
    pub colours: CaelestiaColours,
}

impl OsdView {
    pub fn new(config: OsdConfig, state: OsdState, colours: CaelestiaColours) -> Self {
        Self {
            config,
            state,
            colours,
        }
    }

    /// Render an individual filled slider matching FilledSlider.qml
    fn render_slider(&self, icon: &str, value: f32, max: f32) -> impl IntoElement {
        let width = self.config.slider_width;
        let height = self.config.slider_height;
        let percent = (value / max).clamp(0.0, 1.0);
        let fill_height = height * percent;

        let bg_track = rgba(self.colours.surface_container_highest);
        let fill_color = rgba(self.colours.primary);
        let icon_color = rgba(self.colours.on_primary);

        div()
            .w(px(width))
            .h(px(height))
            .rounded(px(width / 2.0))
            .bg(bg_track)
            .border_1()
            .border_color(rgba(self.colours.outline_variant))
            .overflow_hidden()
            .flex()
            .flex_col_reverse()
            .cursor(CursorStyle::PointingHand)
            .child(
                div()
                    .w_full()
                    .h(px(fill_height))
                    .bg(fill_color)
                    .rounded_b(px(width / 2.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .pb_2()
                    .child(
                        div()
                            .text_color(icon_color)
                            .child(StyledTextExt::icon_size(div().child(icon.to_string()), 18.0)),
                    ),
            )
    }
}

impl Render for OsdView {
    fn render(&mut self, _window: &mut Window, _cx: &mut gpui::Context<Self>) -> impl IntoElement {
        if !self.state.is_open || !self.config.enabled {
            return div().into_any_element();
        }

        let opacity = 1.0 - self.state.offset_scale;
        let volume_icon = OsdState::volume_icon(self.state.volume, self.state.muted);
        let mic_icon = OsdState::mic_icon(self.state.source_muted);
        let brightness_icon = OsdState::brightness_icon(self.state.brightness);

        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_3()
            .p_4()
            .rounded_3xl()
            .bg(rgba(self.colours.surface_container_high))
            .border_1()
            .border_color(rgba(self.colours.outline))
            .shadow_lg()
            .opacity(opacity)
            .children(
                vec![
                    if self.config.enable_volume {
                        Some(self.render_slider(volume_icon, self.state.volume, self.config.max_volume).into_any_element())
                    } else {
                        None
                    },
                    if self.config.enable_microphone {
                        Some(self.render_slider(mic_icon, self.state.source_volume, self.config.max_volume).into_any_element())
                    } else {
                        None
                    },
                    if self.config.enable_brightness {
                        Some(self.render_slider(&brightness_icon, self.state.brightness, 1.0).into_any_element())
                    } else {
                        None
                    },
                ]
                .into_iter()
                .flatten(),
            )
            .into_any_element()
    }
}
