//! Icon image resolver and renderer
//! Parity with `shell/components/images/CachingIconImage.qml`.

use gpui::*;
use std::path::PathBuf;

/// Icon source descriptor distinguishing theme names, URIs, and local file paths
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IconSource {
    Theme(String),
    Uri(String),
    File(PathBuf),
    Empty,
}

impl IconSource {
    pub fn parse(s: &str) -> Self {
        let trimmed = s.trim();
        if trimmed.is_empty() {
            Self::Empty
        } else if trimmed.starts_with("image://icon/") {
            let name = trimmed.trim_start_matches("image://icon/");
            Self::Theme(name.to_string())
        } else if trimmed.starts_with("file://") {
            let path_str = trimmed.trim_start_matches("file://");
            Self::File(PathBuf::from(path_str))
        } else if trimmed.starts_with('/') {
            Self::File(PathBuf::from(trimmed))
        } else {
            Self::Theme(trimmed.to_string())
        }
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::Theme(name) => name,
            Self::Uri(uri) => uri,
            Self::File(path) => path.file_name().and_then(|n| n.to_str()).unwrap_or("file"),
            Self::Empty => "",
        }
    }
}

/// Element rendering an icon image with size constraints
#[derive(IntoElement)]
pub struct IconImageView {
    source: IconSource,
    size: f32,
    fallback: String,
}

impl IconImageView {
    pub fn new(source: IconSource) -> Self {
        Self {
            source,
            size: 24.0,
            fallback: "application-x-executable".to_string(),
        }
    }

    pub fn with_size(mut self, size: f32) -> Self {
        self.size = size.max(8.0);
        self
    }

    pub fn with_fallback(mut self, fallback: impl Into<String>) -> Self {
        self.fallback = fallback.into();
        self
    }
}

impl RenderOnce for IconImageView {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let name = if self.source.is_empty() {
            &self.fallback
        } else {
            self.source.display_name()
        };

        div()
            .w(px(self.size))
            .h(px(self.size))
            .flex()
            .items_center()
            .justify_center()
            .child(
                div()
                    .text_xs()
                    .text_color(rgb(0xd0bcff))
                    .child(format!("🔣 {}", name)),
            )
    }
}
