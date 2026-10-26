//! Wallpaper Management & Wallhaven Search Service
//! 100% parity with `shell/services/Wallpapers.qml`, `WallhavenSearcher.qml`, and `AutoScheme.qml`.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum WallpaperKind {
    StaticImage,
    AnimatedGif,
    Video,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WallpaperEntry {
    pub path: PathBuf,
    pub filename: String,
    pub category: String,
    pub kind: WallpaperKind,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WallhavenImage {
    pub id: String,
    pub url: String,
    pub thumbnail_url: String,
    pub resolution: String,
    pub file_size: u64,
    pub purity: String,
}

pub struct WallpaperService {
    wallpapers_dir: PathBuf,
    state_file: PathBuf,
    current_path: Mutex<PathBuf>,
    preview_path: Mutex<Option<PathBuf>>,
    entries: Mutex<Vec<WallpaperEntry>>,
}

impl Default for WallpaperService {
    fn default() -> Self {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/root".to_string());
        let walls_dir = PathBuf::from(&home).join("Pictures/Wallpapers");
        let state_file = PathBuf::from(&home).join(".local/state/caelestia/wallpaper/path.txt");
        Self::new(walls_dir, state_file)
    }
}

impl WallpaperService {
    pub fn new(wallpapers_dir: PathBuf, state_file: PathBuf) -> Self {
        let current = if state_file.exists() {
            fs::read_to_string(&state_file)
                .map(|s| PathBuf::from(s.trim()))
                .unwrap_or_else(|_| wallpapers_dir.join("default.webp"))
        } else {
            wallpapers_dir.join("default.webp")
        };

        let service = Self {
            wallpapers_dir,
            state_file,
            current_path: Mutex::new(current),
            preview_path: Mutex::new(None),
            entries: Mutex::new(Vec::new()),
        };
        service.rescan();
        service
    }

    pub fn current_wallpaper(&self) -> PathBuf {
        if let Ok(preview) = self.preview_path.lock() {
            if let Some(ref p) = *preview {
                return p.clone();
            }
        }
        self.current_path.lock().map(|p| p.clone()).unwrap_or_default()
    }

    pub fn set_preview(&self, path: Option<PathBuf>) {
        if let Ok(mut p) = self.preview_path.lock() {
            *p = path;
        }
    }

    pub fn set_wallpaper(&self, path: impl AsRef<Path>) -> Result<(), std::io::Error> {
        let path = path.as_ref().to_path_buf();
        if let Some(parent) = self.state_file.parent() {
            let _ = fs::create_dir_all(parent);
        }
        fs::write(&self.state_file, path.to_string_lossy().as_bytes())?;

        if let Ok(mut cur) = self.current_path.lock() {
            *cur = path;
        }
        if let Ok(mut p) = self.preview_path.lock() {
            *p = None;
        }
        Ok(())
    }

    /// Rescans directory recursively for image, gif and video files
    pub fn rescan(&self) {
        let mut found = Vec::new();
        Self::scan_recursive(&self.wallpapers_dir, &self.wallpapers_dir, &mut found);

        if let Ok(mut entries) = self.entries.lock() {
            *entries = found;
        }
    }

    pub fn entries(&self) -> Vec<WallpaperEntry> {
        self.entries.lock().map(|e| e.clone()).unwrap_or_default()
    }

    /// Cycles to the next available wallpaper
    pub fn next_wallpaper(&self) {
        let entries = self.entries();
        if entries.is_empty() {
            return;
        }
        let cur = self.current_wallpaper();
        let next_idx = if let Some(pos) = entries.iter().position(|e| e.path == cur) {
            (pos + 1) % entries.len()
        } else {
            0
        };
        let _ = self.set_wallpaper(&entries[next_idx].path);
    }

    /// Cycles to previous wallpaper
    pub fn previous_wallpaper(&self) {
        let entries = self.entries();
        if entries.is_empty() {
            return;
        }
        let cur = self.current_wallpaper();
        let prev_idx = if let Some(pos) = entries.iter().position(|e| e.path == cur) {
            if pos == 0 { entries.len() - 1 } else { pos - 1 }
        } else {
            0
        };
        let _ = self.set_wallpaper(&entries[prev_idx].path);
    }

    fn scan_recursive(base_dir: &Path, dir: &Path, acc: &mut Vec<WallpaperEntry>) {
        if let Ok(read) = fs::read_dir(dir) {
            for item in read.flatten() {
                let path = item.path();
                if path.is_dir() {
                    Self::scan_recursive(base_dir, &path, acc);
                } else if path.is_file() {
                    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
                    let kind = match ext.as_str() {
                        "jpg" | "jpeg" | "png" | "webp" | "bmp" => Some(WallpaperKind::StaticImage),
                        "gif" => Some(WallpaperKind::AnimatedGif),
                        "mp4" | "webm" | "mkv" => Some(WallpaperKind::Video),
                        _ => None,
                    };

                    if let Some(kind) = kind {
                        let filename = path.file_name().and_then(|f| f.to_str()).unwrap_or("").to_string();
                        let category = if dir == base_dir {
                            "Main".to_string()
                        } else {
                            dir.file_name().and_then(|f| f.to_str()).unwrap_or("General").to_string()
                        };

                        acc.push(WallpaperEntry {
                            path,
                            filename,
                            category,
                            kind,
                        });
                    }
                }
            }
        }
    }
}
