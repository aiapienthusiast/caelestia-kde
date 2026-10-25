//! FileDialog directory contents grid view and entry model
//! Parity with `shell/components/filedialog/FolderContents.qml`.

use super::current_item::FileEntry;
use super::sizes::ITEM_WIDTH;
use gpui::prelude::FluentBuilder;
use gpui::*;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Directory scanner reading and filtering entries in a folder
pub fn scan_directory(dir: &Path, filters: &[String], select_folder: bool) -> Vec<FileEntry> {
    let mut entries = Vec::new();
    let read_dir = match fs::read_dir(dir) {
        Ok(rd) => rd,
        Err(_) => return entries,
    };

    for item in read_dir.flatten() {
        let path = item.path();
        let entry = FileEntry::new(path);

        if entry.is_dir {
            entries.push(entry);
        } else if !select_folder {
            let matches_filter = filters.contains(&"*".to_string())
                || filters.is_empty()
                || filters.iter().any(|f| f.eq_ignore_ascii_case(&entry.suffix));
            if matches_filter {
                entries.push(entry);
            }
        }
    }

    // Sort: directories first, then alphabetical
    entries.sort_by(|a, b| match (a.is_dir, b.is_dir) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
    });

    entries
}

/// Grid view of directory contents
#[derive(IntoElement)]
pub struct FolderContentsView {
    entries: Vec<FileEntry>,
    selected_path: Option<PathBuf>,
    on_entry_clicked: Option<Arc<dyn Fn(FileEntry) + Send + Sync>>,
    on_entry_double_clicked: Option<Arc<dyn Fn(FileEntry) + Send + Sync>>,
}

impl FolderContentsView {
    pub fn new(entries: Vec<FileEntry>, selected_path: Option<PathBuf>) -> Self {
        Self {
            entries,
            selected_path,
            on_entry_clicked: None,
            on_entry_double_clicked: None,
        }
    }

    pub fn on_entry_clicked<F: Fn(FileEntry) + Send + Sync + 'static>(
        mut self,
        cb: F,
    ) -> Self {
        self.on_entry_clicked = Some(Arc::new(cb));
        self
    }

    pub fn on_entry_double_clicked<F: Fn(FileEntry) + Send + Sync + 'static>(
        mut self,
        cb: F,
    ) -> Self {
        self.on_entry_double_clicked = Some(Arc::new(cb));
        self
    }
}

impl RenderOnce for FolderContentsView {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        if self.entries.is_empty() {
            return div()
                .flex_1()
                .flex()
                .flex_col()
                .items_center()
                .justify_center()
                .gap_2()
                .p_6()
                .bg(rgb(0x1a191e))
                .child(
                    div()
                        .text_2xl()
                        .text_color(rgb(0x706c7a))
                        .child("Empty Folder"),
                )
                .child(
                    div()
                        .text_sm()
                        .text_color(rgb(0x504c5a))
                        .child("This folder is empty"),
                );
        }

        let mut grid = div()
            .flex_1()
            .flex()
            .flex_wrap()
            .gap_2()
            .p_3()
            .overflow_hidden()
            .bg(rgb(0x1a191e));

        for entry in self.entries {
            let is_selected = self.selected_path.as_ref() == Some(&entry.path);
            let click_cb = self.on_entry_clicked.clone();
            let dbl_click_cb = self.on_entry_double_clicked.clone();
            let entry_clone = entry.clone();
            let dbl_entry = entry.clone();

            grid = grid.child(
                div()
                    .w(px(ITEM_WIDTH))
                    .h(px(ITEM_WIDTH + 24.0))
                    .flex()
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .p_2()
                    .rounded_md()
                    .cursor_pointer()
                    .when(is_selected, |d| {
                        d.bg(rgb(0x4a4458)).border_1().border_color(rgb(0xd0bcff))
                    })
                    .when(!is_selected, |d| {
                        d.hover(|h| h.bg(rgb(0x25232c)))
                    })
                    .on_mouse_down(MouseButton::Left, move |e, _, _| {
                        if e.click_count == 2 {
                            if let Some(cb) = &dbl_click_cb {
                                cb(dbl_entry.clone());
                            }
                        } else if let Some(cb) = &click_cb {
                            cb(entry_clone.clone());
                        }
                    })
                    .child(
                        div()
                            .text_xl()
                            .child(if entry.is_dir { "📁" } else { "📄" }),
                    )
                    .child(
                        div()
                            .pt_1()
                            .text_xs()
                            .text_color(rgb(0xffffff))
                            .overflow_hidden()
                            .child(entry.name.clone()),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0x8a8494))
                            .child(entry.formatted_size()),
                    ),
            );
        }

        grid
    }
}
