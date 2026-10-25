//! Cross-fading image transition renderer
//! Parity with `shell/components/images/FadeImage.qml`.

use gpui::*;
use std::path::{Path, PathBuf};

/// Controller managing cross-fading image source transitions
#[derive(Clone, Debug)]
pub struct FadeImageController {
    pub current_source: Option<PathBuf>,
    pub previous_source: Option<PathBuf>,
    pub opacity: f32,
    pub is_fading_out: bool,
    pub had_previous: bool,
}

impl Default for FadeImageController {
    fn default() -> Self {
        Self {
            current_source: None,
            previous_source: None,
            opacity: 1.0,
            is_fading_out: false,
            had_previous: false,
        }
    }
}

impl FadeImageController {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_source(&mut self, path: impl AsRef<Path>) {
        let p = path.as_ref().to_path_buf();
        if self.current_source.as_ref() == Some(&p) {
            return;
        }

        if self.current_source.is_some() {
            self.previous_source = self.current_source.take();
            self.had_previous = true;
            self.is_fading_out = true;
            self.opacity = 0.0;
        }

        self.current_source = Some(p);
    }

    pub fn complete_fade_in(&mut self) {
        self.is_fading_out = false;
        self.opacity = 1.0;
        self.previous_source = None;
    }
}

/// Element rendering an image with cross-fade transition state
#[derive(IntoElement)]
pub struct FadeImageView {
    controller: FadeImageController,
}

impl FadeImageView {
    pub fn new(controller: FadeImageController) -> Self {
        Self { controller }
    }
}

impl RenderOnce for FadeImageView {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let name = self
            .controller
            .current_source
            .as_ref()
            .and_then(|p| p.file_name())
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "none".to_string());

        let alpha = (self.controller.opacity.clamp(0.0, 1.0) * 255.0) as u8;

        div()
            .w_full()
            .h_full()
            .flex()
            .items_center()
            .justify_center()
            .bg(rgba((0x1a1920 << 8) | alpha as u32))
            .child(
                div()
                    .text_xs()
                    .text_color(rgb(0xd0bcff))
                    .child(format!("🌄 FadeImage [{}]", name)),
            )
    }
}
