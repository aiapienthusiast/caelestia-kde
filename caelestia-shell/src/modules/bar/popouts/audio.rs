//! Audio Popout
//! 100% Parity with `shell/modules/bar/popouts/AudioPopout.qml`.

use super::wrapper::PopoutWrapper;
use crate::components::controls::button_base::{ButtonStyle, ButtonType};
use crate::components::controls::icon_text_button::IconTextButton;
use crate::components::controls::radio_button::StyledRadioButton;
use crate::components::controls::slider::StyledSlider;
use crate::components::styled_text::StyledText;
use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgba, Div, FontWeight, Styled};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AudioDevice {
    pub id: u32,
    pub name: String,
    pub is_active: bool,
}

impl AudioDevice {
    pub fn new(id: u32, name: impl Into<String>, is_active: bool) -> Self {
        Self {
            id,
            name: name.into(),
            is_active,
        }
    }
}

/// State of audio devices and master volume (port of `shell/services/Audio.qml`).
#[derive(Clone, Debug, PartialEq)]
pub struct AudioPopoutState {
    pub sinks: Vec<AudioDevice>,
    pub sources: Vec<AudioDevice>,
    pub volume: f32, // 0.0 to 1.0
    pub muted: bool,
}

impl Default for AudioPopoutState {
    fn default() -> Self {
        Self {
            sinks: vec![AudioDevice::new(1, "Built-in Audio Analog Stereo (Speakers)", true)],
            sources: vec![AudioDevice::new(2, "Built-in Microphone", true)],
            volume: 0.75,
            muted: false,
        }
    }
}

pub struct AudioPopout;

impl AudioPopout {
    pub fn render(center_x: f32, c: &CaelestiaColours) -> Div {
        Self::render_with_state(center_x, &AudioPopoutState::default(), c)
    }

    /// Renders the audio popout with full parity to AudioPopout.qml.
    pub fn render_with_state(center_x: f32, state: &AudioPopoutState, c: &CaelestiaColours) -> Div {
        let width_px = 300.0;

        let mut content = div()
            .flex()
            .flex_col()
            .w(px(width_px))
            .gap(px(SPACING_MEDIUM));

        // 1. Output device section
        content = content.child(
            StyledText::new("Output device")
                .text_size(px(FONT_SIZE_NORMAL))
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgba(c.on_surface)),
        );

        for sink in &state.sinks {
            content = content.child(StyledRadioButton::labeled(
                sink.name.clone(),
                sink.is_active,
                false,
                false,
                false,
                c,
            ));
        }

        // 2. Input device section
        content = content.child(
            div()
                .mt(px(SPACING_SMALL))
                .child(
                    StyledText::new("Input device")
                        .text_size(px(FONT_SIZE_NORMAL))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(rgba(c.on_surface)),
                ),
        );

        for source in &state.sources {
            content = content.child(StyledRadioButton::labeled(
                source.name.clone(),
                source.is_active,
                false,
                false,
                false,
                c,
            ));
        }

        // 3. Volume section with readout: Volume (Muted / 75%)
        let volume_label = if state.muted {
            "Volume (Muted)".to_string()
        } else {
            format!("Volume ({}%)", (state.volume * 100.0).round() as i32)
        };

        content = content.child(
            div()
                .mt(px(SPACING_SMALL))
                .child(
                    StyledText::new(volume_label)
                        .text_size(px(FONT_SIZE_NORMAL))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(rgba(c.on_surface)),
                ),
        );

        // 4. Volume continuous slider
        content = content.child(
            div()
                .w_full()
                .h(px(PADDING_MEDIUM * 3.0))
                .flex()
                .items_center()
                .child(StyledSlider::render(
                    state.volume,
                    width_px,
                    false,
                    false,
                    state.muted,
                    c,
                )),
        );

        // 5. Open settings button
        let mut btn_style = ButtonStyle::default();
        btn_style.button_type = ButtonType::Tonal;
        btn_style.is_disabled = false;

        content = content.child(
            div()
                .mt(px(SPACING_SMALL))
                .child(IconTextButton::render("settings", "Open settings", &btn_style, c)),
        );

        PopoutWrapper::render("Audio", "volume_up", center_x, content, c)
    }
}
