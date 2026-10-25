//! Cover art visualizer widget for media playback
//! Parity with `shell/components/widgets/CoverArt.qml`.

use gpui::prelude::FluentBuilder;
use gpui::*;
use std::path::PathBuf;

/// State for media player album art display
#[derive(Clone, Debug)]
pub struct CoverArtState {
    pub art_url: Option<String>,
    pub local_path: Option<PathBuf>,
    pub is_playing: bool,
    pub is_loading: bool,
    pub has_error: bool,
    pub rotation_deg: f32,
    pub size: f32,
}

impl Default for CoverArtState {
    fn default() -> Self {
        Self {
            art_url: None,
            local_path: None,
            is_playing: false,
            is_loading: false,
            has_error: false,
            rotation_deg: 0.0,
            size: 80.0,
        }
    }
}

impl CoverArtState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_art(mut self, url: impl Into<String>) -> Self {
        self.art_url = Some(url.into());
        self
    }

    pub fn with_playing(mut self, playing: bool) -> Self {
        self.is_playing = playing;
        self
    }

    pub fn with_size(mut self, size: f32) -> Self {
        self.size = size.max(24.0);
        self
    }

    /// Advances the vinyl / cookie shape rotation when playing (23.5s per 360 deg)
    pub fn step_rotation(&mut self, delta_seconds: f32) {
        if self.is_playing {
            let deg_per_sec = 360.0 / 23.5;
            self.rotation_deg = (self.rotation_deg + deg_per_sec * delta_seconds) % 360.0;
        }
    }
}

/// Renderable CoverArt widget element
#[derive(IntoElement)]
pub struct CoverArtView {
    state: CoverArtState,
}

impl CoverArtView {
    pub fn new(state: CoverArtState) -> Self {
        Self { state }
    }
}

impl RenderOnce for CoverArtView {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let size = self.state.size;
        let is_loading = self.state.is_loading;
        let has_art = self.state.art_url.is_some() || self.state.local_path.is_some();
        let has_err = self.state.has_error;

        div()
            .w(px(size))
            .h(px(size))
            .flex()
            .items_center()
            .justify_center()
            .rounded_2xl()
            .overflow_hidden()
            .bg(rgb(0x2b2930))
            .shadow_md()
            .when(is_loading, |d| {
                d.child(
                    div()
                        .text_xs()
                        .text_color(rgb(0xd0bcff))
                        .child("⏳ Loading..."),
                )
            })
            .when(!is_loading && has_art && !has_err, |d| {
                d.child(
                    div()
                        .w_full()
                        .h_full()
                        .flex()
                        .items_center()
                        .justify_center()
                        .bg(rgb(0x381e72))
                        .child(
                            div()
                                .text_base()
                                .text_color(rgb(0xffffff))
                                .child("🎵 Album Art"),
                        ),
                )
            })
            .when(!is_loading && (!has_art || has_err), |d| {
                d.child(
                    div()
                        .text_xs()
                        .text_color(rgb(0x938f99))
                        .child(if has_err { "broken_image" } else { "art_track" }),
                )
            })
    }
}
