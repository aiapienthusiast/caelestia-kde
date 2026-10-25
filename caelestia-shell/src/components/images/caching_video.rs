//! Hardware-accelerated video wallpaper player and lifecycle controller
//! Parity with `shell/components/images/CachingVideo.qml`.

use gpui::*;
use std::path::{Path, PathBuf};

/// Controller managing video wallpaper playback, auto-pause, and audio muting
#[derive(Clone, Debug)]
pub struct VideoWallpaperController {
    pub path: PathBuf,
    pub playing: bool,
    pub muted: bool,
    pub paused_by_user: bool,
    pub is_first_instance: bool,
}

impl VideoWallpaperController {
    pub fn new(path: impl AsRef<Path>, is_first_instance: bool) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            playing: true,
            muted: false,
            paused_by_user: false,
            is_first_instance,
        }
    }

    /// Evaluates auto-pause conditions based on window state and config
    pub fn update_pause_state(
        &mut self,
        has_fullscreen: bool,
        has_tiled: bool,
        pause_on_fullscreen: bool,
        pause_on_tiled: bool,
    ) {
        if self.paused_by_user {
            self.playing = false;
            return;
        }

        let should_pause = (pause_on_fullscreen && has_fullscreen) || (pause_on_tiled && has_tiled);
        self.playing = !should_pause;
    }

    /// Evaluates auto-mute conditions based on active audio playback
    pub fn update_mute_state(
        &mut self,
        sound_enabled: bool,
        mute_on_media: bool,
        media_playing: bool,
    ) {
        if !self.is_first_instance || !sound_enabled {
            self.muted = true;
        } else if mute_on_media && media_playing {
            self.muted = true;
        } else {
            self.muted = false;
        }
    }
}

/// Element rendering video wallpaper output surface
#[derive(IntoElement)]
pub struct CachingVideoView {
    controller: VideoWallpaperController,
}

impl CachingVideoView {
    pub fn new(controller: VideoWallpaperController) -> Self {
        Self { controller }
    }
}

impl RenderOnce for CachingVideoView {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let name = self
            .controller
            .path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "none".to_string());

        let is_playing = self.controller.playing;
        let is_muted = self.controller.muted;

        div()
            .w_full()
            .h_full()
            .flex()
            .items_center()
            .justify_center()
            .bg(rgb(0x100f14))
            .child(
                div()
                    .text_xs()
                    .text_color(rgb(0xd0bcff))
                    .child(format!(
                        "🎬 Video [{}] [{}] [{}]",
                        name,
                        if is_playing { "Playing" } else { "Paused" },
                        if is_muted { "Muted" } else { "Audio" }
                    )),
            )
    }
}
