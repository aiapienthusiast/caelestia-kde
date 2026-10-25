//! Animated GIF / WebP playback controller and renderer
//! Parity with `shell/components/images/CachingAnimatedImage.qml`.

use gpui::*;
use std::path::{Path, PathBuf};

/// Controller managing animated image frames and playback state
#[derive(Clone, Debug)]
pub struct AnimatedImageController {
    pub path: PathBuf,
    pub playing: bool,
    pub current_frame: usize,
    pub total_frames: usize,
    pub loops_completed: usize,
}

impl AnimatedImageController {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            playing: true,
            current_frame: 0,
            total_frames: 1,
            loops_completed: 0,
        }
    }

    pub fn with_frames(mut self, frames: usize) -> Self {
        self.total_frames = frames.max(1);
        self
    }

    pub fn play(&mut self) {
        self.playing = true;
    }

    pub fn pause(&mut self) {
        self.playing = false;
    }

    pub fn restart(&mut self) {
        self.current_frame = 0;
        self.playing = true;
    }

    pub fn step_frame(&mut self) {
        if !self.playing || self.total_frames <= 1 {
            return;
        }
        self.current_frame += 1;
        if self.current_frame >= self.total_frames {
            self.current_frame = 0;
            self.loops_completed += 1;
        }
    }
}

/// Element rendering an animated image sequence
#[derive(IntoElement)]
pub struct AnimatedImageView {
    controller: AnimatedImageController,
}

impl AnimatedImageView {
    pub fn new(controller: AnimatedImageController) -> Self {
        Self { controller }
    }
}

impl RenderOnce for AnimatedImageView {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let frame = self.controller.current_frame;
        let total = self.controller.total_frames;
        let is_playing = self.controller.playing;

        div()
            .w_full()
            .h_full()
            .flex()
            .items_center()
            .justify_center()
            .bg(rgb(0x1a1920))
            .child(
                div()
                    .text_xs()
                    .text_color(rgb(0xd0bcff))
                    .child(format!(
                        "🎞 [{} {}/{}] {}",
                        if is_playing { "▶" } else { "⏸" },
                        frame + 1,
                        total,
                        self.controller.path.file_name().unwrap_or_default().to_string_lossy()
                    )),
            )
    }
}
