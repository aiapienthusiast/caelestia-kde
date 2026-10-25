//! FileDialog current item preview and metadata display
//! Parity with `shell/components/filedialog/CurrentItem.qml`.

use gpui::*;
use std::path::PathBuf;

/// Metadata model for a filesystem entry within FileDialog
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FileEntry {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub size_bytes: u64,
    pub suffix: String,
}

impl FileEntry {
    pub fn new(path: PathBuf) -> Self {
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();
        let suffix = path
            .extension()
            .map(|e| e.to_string_lossy().to_string())
            .unwrap_or_default();
        let metadata = std::fs::metadata(&path).ok();
        let is_dir = metadata.as_ref().map(|m| m.is_dir()).unwrap_or(false);
        let size_bytes = metadata.as_ref().map(|m| m.len()).unwrap_or(0);

        Self {
            name,
            path,
            is_dir,
            size_bytes,
            suffix,
        }
    }

    pub fn formatted_size(&self) -> String {
        if self.is_dir {
            return "Folder".to_string();
        }
        const KB: u64 = 1024;
        const MB: u64 = KB * 1024;
        const GB: u64 = MB * 1024;

        if self.size_bytes >= GB {
            format!("{:.1} GB", self.size_bytes as f64 / GB as f64)
        } else if self.size_bytes >= MB {
            format!("{:.1} MB", self.size_bytes as f64 / MB as f64)
        } else if self.size_bytes >= KB {
            format!("{:.1} KB", self.size_bytes as f64 / KB as f64)
        } else {
            format!("{} B", self.size_bytes)
        }
    }
}

/// Renderable preview card for the currently selected file entry
#[derive(IntoElement)]
pub struct CurrentItemView {
    entry: Option<FileEntry>,
}

impl CurrentItemView {
    pub fn new(entry: Option<FileEntry>) -> Self {
        Self { entry }
    }
}

impl RenderOnce for CurrentItemView {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        if let Some(entry) = self.entry {
            div()
                .flex()
                .flex_col()
                .gap_1()
                .p_2()
                .rounded_md()
                .bg(rgb(0x2a2830))
                .child(
                    div()
                        .text_sm()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(0xffffff))
                        .child(entry.name.clone()),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(rgb(0xa09cb0))
                        .child(entry.formatted_size()),
                )
        } else {
            div()
        }
    }
}
