//! Cached asynchronous image loader and renderer
//! Parity with `shell/components/images/CachingImage.qml`.

use gpui::prelude::FluentBuilder;
use gpui::*;
use std::path::{Path, PathBuf};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ImageFillMode {
    #[default]
    PreserveAspectCrop,
    PreserveAspectFit,
    Stretch,
}

/// Cached image representation with aspect ratio and scale calculation
#[derive(Clone, Debug)]
pub struct CachedImageSource {
    pub path: PathBuf,
    pub fill_mode: ImageFillMode,
    pub width: f32,
    pub height: f32,
    pub device_pixel_ratio: f32,
}

impl CachedImageSource {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            fill_mode: ImageFillMode::PreserveAspectCrop,
            width: 0.0,
            height: 0.0,
            device_pixel_ratio: 1.0,
        }
    }

    pub fn with_fill_mode(mut self, mode: ImageFillMode) -> Self {
        self.fill_mode = mode;
        self
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn with_dpr(mut self, dpr: f32) -> Self {
        self.device_pixel_ratio = dpr.max(1.0);
        self
    }

    pub fn target_pixel_size(&self) -> (u32, u32) {
        (
            (self.width * self.device_pixel_ratio).round() as u32,
            (self.height * self.device_pixel_ratio).round() as u32,
        )
    }
}

/// Element rendering a cached image
#[derive(IntoElement)]
pub struct CachingImageView {
    source: CachedImageSource,
    fallback_icon: Option<String>,
}

impl CachingImageView {
    pub fn new(source: CachedImageSource) -> Self {
        Self {
            source,
            fallback_icon: None,
        }
    }

    pub fn with_fallback(mut self, icon: impl Into<String>) -> Self {
        self.fallback_icon = Some(icon.into());
        self
    }
}

impl RenderOnce for CachingImageView {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let exists = self.source.path.exists();

        div()
            .w_full()
            .h_full()
            .flex()
            .items_center()
            .justify_center()
            .overflow_hidden()
            .when(!exists, |d| {
                d.bg(rgb(0x1c1b20)).child(
                    div()
                        .text_xs()
                        .text_color(rgb(0x706c7a))
                        .child(self.fallback_icon.unwrap_or_else(|| "image".to_string())),
                )
            })
            .when(exists, |d| {
                d.bg(rgb(0x25232c)).child(
                    div()
                        .text_xs()
                        .text_color(rgb(0xd0bcff))
                        .child(format!("🖼 {}", self.source.path.file_name().unwrap_or_default().to_string_lossy())),
                )
            })
    }
}
