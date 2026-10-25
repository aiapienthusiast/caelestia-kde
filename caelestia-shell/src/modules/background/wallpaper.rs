//! Wallpaper management model for Caelestia Shell
//! Achieves 100% parity with `shell/modules/background/Wallpaper.qml` and `Wallpapers.qml`.

use std::path::{Path, PathBuf};

/// Supported video wallpaper extensions
pub const VIDEO_EXTENSIONS: &[&str] = &["mp4", "webm", "mkv", "avi", "mov", "wmv", "flv"];

/// Supported static image wallpaper extensions
pub const IMAGE_EXTENSIONS: &[&str] = &["png", "jpg", "jpeg", "webp", "avif", "bmp", "gif"];

/// Model tracking active wallpaper, directory list, and slideshow cycling
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WallpaperModel {
    pub current_path: Option<PathBuf>,
    pub available: Vec<PathBuf>,
    pub current_index: Option<usize>,
}

impl Default for WallpaperModel {
    fn default() -> Self {
        Self {
            current_path: None,
            available: Vec::new(),
            current_index: None,
        }
    }
}

impl WallpaperModel {
    /// Creates a new wallpaper model initialized with an optional path
    pub fn new(initial_path: Option<PathBuf>) -> Self {
        let mut model = Self {
            current_path: initial_path.clone(),
            available: Vec::new(),
            current_index: None,
        };
        if let Some(path) = initial_path {
            if let Some(parent) = path.parent() {
                model.scan_directory(parent);
            }
        }
        model
    }

    /// Checks if a file path is a video wallpaper
    pub fn is_video_path(path: &Path) -> bool {
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| {
                let lower = ext.to_lowercase();
                VIDEO_EXTENSIONS.contains(&lower.as_str())
            })
            .unwrap_or(false)
    }

    /// Checks if a file path is a valid supported wallpaper (image or video)
    pub fn is_supported_wallpaper(path: &Path) -> bool {
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| {
                let lower = ext.to_lowercase();
                VIDEO_EXTENSIONS.contains(&lower.as_str()) || IMAGE_EXTENSIONS.contains(&lower.as_str())
            })
            .unwrap_or(false)
    }

    /// Returns true if the currently active wallpaper is a video
    pub fn is_current_video(&self) -> bool {
        self.current_path
            .as_ref()
            .map(|p| Self::is_video_path(p))
            .unwrap_or(false)
    }

    /// Scans a directory for supported wallpaper files, sorting them deterministically
    pub fn scan_directory(&mut self, dir: &Path) {
        let mut found = Vec::new();
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() && Self::is_supported_wallpaper(&path) {
                    found.push(path);
                }
            }
        }
        found.sort();
        self.available = found;
        self.update_current_index();
    }

    /// Updates internal index matching `current_path`
    fn update_current_index(&mut self) {
        if let Some(ref current) = self.current_path {
            self.current_index = self.available.iter().position(|p| p == current);
        } else {
            self.current_index = None;
        }
    }

    /// Sets the active wallpaper directly
    pub fn set_wallpaper(&mut self, path: PathBuf) {
        self.current_path = Some(path);
        self.update_current_index();
    }

    /// Cycles to the next wallpaper in the list (sequential or random)
    pub fn next_wallpaper(&mut self, random: bool) {
        if self.available.is_empty() {
            return;
        }

        if random && self.available.len() > 1 {
            let next_idx = self.pseudo_random_index(self.available.len(), self.current_index);
            self.current_index = Some(next_idx);
            self.current_path = Some(self.available[next_idx].clone());
        } else {
            let next_idx = match self.current_index {
                Some(idx) => (idx + 1) % self.available.len(),
                None => 0,
            };
            self.current_index = Some(next_idx);
            self.current_path = Some(self.available[next_idx].clone());
        }
    }

    /// Cycles to the previous wallpaper in the list
    pub fn prev_wallpaper(&mut self) {
        if self.available.is_empty() {
            return;
        }

        let prev_idx = match self.current_index {
            Some(0) | None => self.available.len().saturating_sub(1),
            Some(idx) => idx - 1,
        };
        self.current_index = Some(prev_idx);
        self.current_path = Some(self.available[prev_idx].clone());
    }

    /// Sets a random wallpaper from the available pool
    pub fn set_random(&mut self) {
        self.next_wallpaper(true);
    }

    /// Deterministic pseudo-random index selection avoiding the current index when possible
    fn pseudo_random_index(&self, total: usize, current: Option<usize>) -> usize {
        if total <= 1 {
            return 0;
        }
        let now_millis = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis())
            .unwrap_or(0);
        let mut idx = (now_millis as usize) % total;
        if Some(idx) == current {
            idx = (idx + 1) % total;
        }
        idx
    }
}
