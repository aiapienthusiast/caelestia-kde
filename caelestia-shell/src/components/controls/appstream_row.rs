use gpui::*;

use crate::components::controls::slider_row::SliderRow;
use crate::infrastructure::theme::CaelestiaColours;

/// Application PipeWire audio stream volume row (port of `AppStreamRow.qml`).
#[derive(Clone)]
pub struct AppStreamRow {
    pub stream_id: u32,
    pub stream_name: String,
    pub volume: f32, // 0.0 to 1.0
    pub muted: bool,
    pub mute_on_icon_click: bool,
    pub is_hovered: bool,
    pub is_dragging: bool,
    pub colours: Option<CaelestiaColours>,
}

impl AppStreamRow {
    pub fn new(stream_id: u32, stream_name: impl Into<String>) -> Self {
        Self {
            stream_id,
            stream_name: stream_name.into(),
            volume: 1.0,
            muted: false,
            mute_on_icon_click: true,
            is_hovered: false,
            is_dragging: false,
            colours: None,
        }
    }

    pub fn volume(mut self, volume: f32) -> Self {
        self.volume = volume.clamp(0.0, 1.0);
        self
    }

    pub fn muted(mut self, muted: bool) -> Self {
        self.muted = muted;
        self
    }

    pub fn mute_on_icon_click(mut self, allow: bool) -> Self {
        self.mute_on_icon_click = allow;
        self
    }

    pub fn with_colours(mut self, colours: CaelestiaColours) -> Self {
        self.colours = Some(colours);
        self
    }

    pub fn volume_icon(&self) -> &'static str {
        if self.muted || self.volume <= 0.001 {
            "volume_off"
        } else if self.volume < 0.33 {
            "volume_mute"
        } else if self.volume < 0.66 {
            "volume_down"
        } else {
            "volume_up"
        }
    }

    pub fn value_label(&self) -> String {
        if self.muted {
            "Muted".to_string()
        } else {
            format!("{}%", (self.volume * 100.0).round() as i32)
        }
    }
}

impl IntoElement for AppStreamRow {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        let icon = self.volume_icon();
        let value_label = self.value_label();
        let name = self.stream_name;
        let volume = self.volume;
        let is_disabled = self.muted;
        let colours = self.colours.unwrap_or_default();

        SliderRow::render(
            name,
            value_label,
            volume,
            200.0,
            Some(icon),
            self.is_hovered,
            self.is_dragging,
            is_disabled,
            &colours,
        )
    }
}
