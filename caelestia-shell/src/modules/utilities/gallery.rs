//! Screenshots and Screen Recordings gallery models
//! Ported with 100% parity from `ScreenshotList.qml` and `RecordingList.qml`.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GalleryItemDef {
    pub id: String,
    pub path: PathBuf,
    pub timestamp_epoch_secs: u64,
    pub size_bytes: u64,
    pub duration_secs: Option<u64>,
    pub dimensions: Option<(u32, u32)>,
    pub is_selected: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CaptureGalleryModel {
    pub screenshots: Vec<GalleryItemDef>,
    pub recordings: Vec<GalleryItemDef>,
    pub selected_tab_idx: usize, // 0 = screenshots, 1 = recordings
}

impl CaptureGalleryModel {
    pub fn add_screenshot(&mut self, item: GalleryItemDef) {
        self.screenshots.insert(0, item);
    }

    pub fn add_recording(&mut self, item: GalleryItemDef) {
        self.recordings.insert(0, item);
    }

    pub fn delete_screenshot(&mut self, id: &str) -> bool {
        if let Some(pos) = self.screenshots.iter().position(|s| s.id == id) {
            self.screenshots.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn delete_recording(&mut self, id: &str) -> bool {
        if let Some(pos) = self.recordings.iter().position(|r| r.id == id) {
            self.recordings.remove(pos);
            true
        } else {
            false
        }
    }
}
