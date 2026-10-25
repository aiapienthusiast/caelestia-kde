//! Main FileDialog coordinator and composite view
//! Parity with `shell/components/filedialog/FileDialog.qml`.

use super::current_item::FileEntry;
use super::dialog_buttons::DialogButtonsView;
use super::folder_contents::{scan_directory, FolderContentsView};
use super::header_bar::HeaderBarView;
use super::sidebar::SidebarView;
use super::sizes::{DIALOG_HEIGHT, DIALOG_MIN_HEIGHT, DIALOG_MIN_WIDTH, DIALOG_WIDTH};
use gpui::*;
use std::path::PathBuf;
use std::sync::Arc;

/// FileDialog controller managing state, current working directory, and selection validation
#[derive(Clone, Debug)]
pub struct FileDialog {
    pub cwd: Vec<String>,
    pub filter_label: String,
    pub filters: Vec<String>,
    pub select_folder: bool,
    pub title: String,
    pub is_open: bool,
    pub selected_entry: Option<FileEntry>,
}

impl Default for FileDialog {
    fn default() -> Self {
        Self {
            cwd: vec!["Home".to_string()],
            filter_label: "All files".to_string(),
            filters: vec!["*".to_string()],
            select_folder: false,
            title: "Select a file".to_string(),
            is_open: false,
            selected_entry: None,
        }
    }
}

impl FileDialog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn for_folder() -> Self {
        Self {
            select_folder: true,
            title: "Select a folder".to_string(),
            ..Self::default()
        }
    }

    pub fn with_filters(mut self, label: impl Into<String>, filters: Vec<String>) -> Self {
        self.filter_label = label.into();
        self.filters = filters;
        self
    }

    pub fn open(&mut self) {
        self.is_open = true;
    }

    pub fn close(&mut self) {
        self.is_open = false;
        self.selected_entry = None;
    }

    /// Resolves the current breadcrumb vector to an absolute filesystem PathBuf
    pub fn resolve_cwd_path(&self) -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/home".to_string());
        if self.cwd.is_empty() || self.cwd[0] == "Home" {
            let mut path = PathBuf::from(home);
            for seg in self.cwd.iter().skip(1) {
                path.push(seg);
            }
            path
        } else {
            let mut path = PathBuf::from("/");
            for seg in &self.cwd {
                path.push(seg);
            }
            path
        }
    }

    /// Validates whether the current selection can be accepted
    pub fn is_selection_valid(&self) -> bool {
        if self.select_folder {
            return true;
        }
        if let Some(entry) = &self.selected_entry {
            if entry.is_dir {
                return false;
            }
            if self.filters.contains(&"*".to_string()) || self.filters.is_empty() {
                return true;
            }
            self.filters
                .iter()
                .any(|f| f.eq_ignore_ascii_case(&entry.suffix))
        } else {
            false
        }
    }

    /// Returns the target path to hand off on acceptance
    pub fn accept_selection(&self) -> Option<PathBuf> {
        if !self.select_folder {
            self.selected_entry.as_ref().map(|e| e.path.clone())
        } else if let Some(entry) = &self.selected_entry {
            if entry.is_dir {
                Some(entry.path.clone())
            } else {
                Some(self.resolve_cwd_path())
            }
        } else {
            Some(self.resolve_cwd_path())
        }
    }

    /// Navigates up one folder level
    pub fn go_up(&mut self) {
        if self.cwd.len() > 1 {
            self.cwd.pop();
            self.selected_entry = None;
        }
    }

    /// Sets cwd from standard sidebar place
    pub fn select_place(&mut self, place: &str) {
        if place == "Home" {
            self.cwd = vec!["Home".to_string()];
        } else {
            self.cwd = vec!["Home".to_string(), place.to_string()];
        }
        self.selected_entry = None;
    }
}

/// Composite FileDialog view rendering sidebar, header, contents grid, and action buttons
#[derive(IntoElement)]
pub struct FileDialogView {
    dialog: FileDialog,
    on_accepted: Option<Arc<dyn Fn(PathBuf) + Send + Sync>>,
    on_rejected: Option<Arc<dyn Fn() + Send + Sync>>,
}

impl FileDialogView {
    pub fn new(dialog: FileDialog) -> Self {
        Self {
            dialog,
            on_accepted: None,
            on_rejected: None,
        }
    }

    pub fn on_accepted<F: Fn(PathBuf) + Send + Sync + 'static>(mut self, cb: F) -> Self {
        self.on_accepted = Some(Arc::new(cb));
        self
    }

    pub fn on_rejected<F: Fn() + Send + Sync + 'static>(mut self, cb: F) -> Self {
        self.on_rejected = Some(Arc::new(cb));
        self
    }
}

impl RenderOnce for FileDialogView {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        if !self.dialog.is_open {
            return div();
        }

        let cwd_path = self.dialog.resolve_cwd_path();
        let entries = scan_directory(
            &cwd_path,
            &self.dialog.filters,
            self.dialog.select_folder,
        );

        let selected_path = self.dialog.selected_entry.as_ref().map(|e| e.path.clone());
        let current_place = self
            .dialog
            .cwd
            .last()
            .cloned()
            .unwrap_or_else(|| "Home".to_string());

        let accept_cb = self.on_accepted.clone();
        let reject_cb = self.on_rejected.clone();
        let valid = self.dialog.is_selection_valid();
        let accepted_path = self.dialog.accept_selection();

        div()
            .w(px(DIALOG_WIDTH))
            .h(px(DIALOG_HEIGHT))
            .min_w(px(DIALOG_MIN_WIDTH))
            .min_h(px(DIALOG_MIN_HEIGHT))
            .flex()
            .flex_row()
            .rounded_2xl()
            .overflow_hidden()
            .bg(rgb(0x141218))
            .child(SidebarView::new(current_place))
            .child(
                div()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .child(HeaderBarView::new(self.dialog.cwd.clone()))
                    .child(FolderContentsView::new(entries, selected_path))
                    .child(
                        DialogButtonsView::new(
                            self.dialog.filter_label.clone(),
                            self.dialog.filters.clone(),
                            self.dialog.select_folder,
                            valid,
                        )
                        .on_select(move || {
                            if let (Some(cb), Some(p)) = (&accept_cb, &accepted_path) {
                                cb(p.clone());
                            }
                        })
                        .on_cancel(move || {
                            if let Some(cb) = &reject_cb {
                                cb();
                            }
                        }),
                    ),
            )
    }
}
