//! OSD Controller Subsystem
//! Complete parity with signal handlers and timer triggers in `shell/modules/osd/Wrapper.qml`.

use super::config::OsdConfig;
use super::state::OsdState;
use crate::services::audio::AudioService;
use crate::services::brightness::BrightnessService;
use std::time::Instant;

pub struct OsdController {
    pub config: OsdConfig,
    pub state: OsdState,
}

impl OsdController {
    pub fn new(config: OsdConfig) -> Self {
        Self {
            config,
            state: OsdState::default(),
        }
    }

    /// Trigger OSD appearance and restart hide timer
    pub fn show(&mut self) {
        self.state.is_open = true;
        self.state.offset_scale = 0.0;
        self.state.last_shown_at = Some(Instant::now());
    }

    /// Hide OSD
    pub fn hide(&mut self) {
        if !self.state.hovered {
            self.state.is_open = false;
            self.state.offset_scale = 1.0;
        }
    }

    /// Update hover state
    pub fn set_hovered(&mut self, hovered: bool) {
        self.state.hovered = hovered;
        if !hovered && self.state.is_open {
            self.state.last_shown_at = Some(Instant::now());
        }
    }

    /// Periodic tick to evaluate auto-hide timer
    pub fn tick(&mut self) {
        if self.state.is_open && self.state.is_expired(self.config.hide_delay_ms) {
            self.hide();
        }
    }

    /// Synchronize audio values and trigger OSD if output volume or mute changed
    pub fn sync_audio(&mut self, audio: &AudioService) {
        let vol = audio.state.sink_volume;
        let muted = audio.state.sink_muted;
        let src_vol = audio.state.source_volume;
        let src_muted = audio.state.source_muted;

        let changed = (vol - self.state.volume).abs() > 0.001
            || muted != self.state.muted
            || (src_vol - self.state.source_volume).abs() > 0.001
            || src_muted != self.state.source_muted;

        if changed {
            self.state.volume = vol;
            self.state.muted = muted;
            self.state.source_volume = src_vol;
            self.state.source_muted = src_muted;
            self.show();
        }
    }

    /// Synchronize brightness values and trigger OSD if brightness changed
    pub fn sync_brightness(&mut self, brightness: &BrightnessService) {
        let b = brightness.state.current_pct as f32 / 100.0;
        if (b - self.state.brightness).abs() > 0.005 {
            self.state.brightness = b;
            self.show();
        }
    }

    /// Step output volume up
    pub fn increment_volume(&mut self, audio: &mut AudioService) {
        audio.step_volume(self.config.volume_step);
        self.state.volume = audio.state.sink_volume;
        self.state.muted = audio.state.sink_muted;
        self.show();
    }

    /// Step output volume down
    pub fn decrement_volume(&mut self, audio: &mut AudioService) {
        audio.step_volume(-self.config.volume_step);
        self.state.volume = audio.state.sink_volume;
        self.state.muted = audio.state.sink_muted;
        self.show();
    }

    /// Set output volume to absolute percentage
    pub fn set_volume(&mut self, audio: &mut AudioService, value: f32) {
        audio.set_volume(value.clamp(0.0, self.config.max_volume));
        self.state.volume = audio.state.sink_volume;
        self.show();
    }

    /// Step input (mic) volume up
    pub fn increment_source_volume(&mut self, audio: &mut AudioService) {
        let new_vol = audio.state.source_volume + self.config.volume_step;
        audio.set_source_volume(new_vol);
        self.state.source_volume = audio.state.source_volume;
        self.state.source_muted = audio.state.source_muted;
        self.show();
    }

    /// Step input (mic) volume down
    pub fn decrement_source_volume(&mut self, audio: &mut AudioService) {
        let new_vol = audio.state.source_volume - self.config.volume_step;
        audio.set_source_volume(new_vol);
        self.state.source_volume = audio.state.source_volume;
        self.state.source_muted = audio.state.source_muted;
        self.show();
    }

    /// Set input (mic) volume to absolute percentage
    pub fn set_source_volume(&mut self, audio: &mut AudioService, value: f32) {
        audio.set_source_volume(value.clamp(0.0, self.config.max_volume));
        self.state.source_volume = audio.state.source_volume;
        self.show();
    }

    /// Step display backlight brightness
    pub fn step_brightness(&mut self, brightness: &mut BrightnessService, delta: f32) {
        let delta_pct = (delta * 100.0).round() as i32;
        brightness.step(delta_pct);
        self.state.brightness = brightness.state.current_pct as f32 / 100.0;
        self.show();
    }

    /// Set display backlight brightness
    pub fn set_brightness(&mut self, brightness: &mut BrightnessService, value: f32) {
        let pct = (value * 100.0).round() as u32;
        brightness.set_percent(pct);
        self.state.brightness = brightness.state.current_pct as f32 / 100.0;
        self.show();
    }
}
