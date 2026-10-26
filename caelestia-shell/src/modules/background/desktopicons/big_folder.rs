//! Expanded folder and grouping models for desktop icons
//! Ported with 100% parity from `BigFolder.qml`, `GroupPopup.qml`, and `GroupTitle.qml`.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BigFolderItem {
    pub key: String,
    pub title: String,
    pub icon_name: String,
    pub path: PathBuf,
    pub children_keys: Vec<String>,
    pub is_expanded: bool,
    pub page_index: usize,
    pub items_per_page: usize,
}

impl BigFolderItem {
    pub fn new(key: impl Into<String>, title: impl Into<String>, path: PathBuf) -> Self {
        Self {
            key: key.into(),
            title: title.into(),
            icon_name: "folder".to_string(),
            path,
            children_keys: Vec::new(),
            is_expanded: false,
            page_index: 0,
            items_per_page: 9,
        }
    }

    pub fn total_pages(&self) -> usize {
        if self.children_keys.is_empty() {
            1
        } else {
            (self.children_keys.len() + self.items_per_page - 1) / self.items_per_page
        }
    }

    pub fn current_page_items(&self) -> &[String] {
        let start = self.page_index * self.items_per_page;
        if start >= self.children_keys.len() {
            &[]
        } else {
            let end = (start + self.items_per_page).min(self.children_keys.len());
            &self.children_keys[start..end]
        }
    }

    pub fn add_child(&mut self, key: impl Into<String>) {
        let k = key.into();
        if !self.children_keys.contains(&k) {
            self.children_keys.push(k);
        }
    }

    pub fn remove_child(&mut self, key: &str) -> bool {
        if let Some(pos) = self.children_keys.iter().position(|k| k == key) {
            self.children_keys.remove(pos);
            if self.page_index >= self.total_pages() && self.page_index > 0 {
                self.page_index -= 1;
            }
            true
        } else {
            false
        }
    }
}

/// Desktop item group popup model
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopGroupPopup {
    pub group_key: String,
    pub title: String,
    pub is_open: bool,
    pub items: Vec<String>,
}

impl DesktopGroupPopup {
    pub fn new(group_key: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            group_key: group_key.into(),
            title: title.into(),
            is_open: false,
            items: Vec::new(),
        }
    }

    pub fn open(&mut self) {
        self.is_open = true;
    }

    pub fn close(&mut self) {
        self.is_open = false;
    }
}
