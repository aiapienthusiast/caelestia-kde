//! Caelestia Utility Functions & Data Structures
//! 100% parity with C++ `Caelestia::CUtils`, `CircularBuffer`, and `AppIdentifiers`.

use std::collections::VecDeque;
use std::fs;
use std::path::{Path, PathBuf};
use sha2::{Digest, Sha256};

/// General utility functions ported from C++ CUtils
pub struct CaelestiaUtils;

impl CaelestiaUtils {
    pub const VERSION: &'static str = "0.1.0";

    pub fn version() -> &'static str {
        Self::VERSION
    }

    /// Computes SHA256 hex digest of in-memory bytes
    pub fn sha256_bytes(bytes: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(bytes);
        format!("{:x}", hasher.finalize())
    }

    /// Computes SHA256 hex digest of file contents
    pub fn sha256_file(path: impl AsRef<Path>) -> Result<String, std::io::Error> {
        let bytes = fs::read(path)?;
        Ok(Self::sha256_bytes(&bytes))
    }

    /// Copies file with optional overwrite protection
    pub fn copy_file(source: impl AsRef<Path>, target: impl AsRef<Path>, overwrite: bool) -> Result<u64, std::io::Error> {
        let target = target.as_ref();
        if target.exists() && !overwrite {
            return Err(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                "Target file already exists and overwrite is false",
            ));
        }
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(source, target)
    }

    /// Removes a file safely
    pub fn delete_file(path: impl AsRef<Path>) -> Result<(), std::io::Error> {
        let path = path.as_ref();
        if path.exists() {
            fs::remove_file(path)
        } else {
            Ok(())
        }
    }

    /// Normalizes URL or path strings into canonical local PathBuf
    pub fn to_local_path(url_or_path: &str) -> PathBuf {
        if let Some(stripped) = url_or_path.strip_prefix("file://") {
            PathBuf::from(stripped)
        } else {
            PathBuf::from(url_or_path)
        }
    }

    /// Updates or inserts a key-value pair in a Freedesktop .desktop file
    pub fn set_desktop_entry_key(
        path: impl AsRef<Path>,
        key: &str,
        value: &str,
    ) -> Result<(), std::io::Error> {
        let path = path.as_ref();
        let content = if path.exists() {
            fs::read_to_string(path)?
        } else {
            "[Desktop Entry]\n".to_string()
        };

        let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        let prefix = format!("{}=", key);
        let mut replaced = false;

        for line in &mut lines {
            if line.starts_with(&prefix) {
                *line = format!("{}={}", key, value);
                replaced = true;
                break;
            }
        }

        if !replaced {
            let mut insert_idx = lines.len();
            for (idx, line) in lines.iter().enumerate() {
                if line.trim() == "[Desktop Entry]" {
                    insert_idx = idx + 1;
                    break;
                }
            }
            lines.insert(insert_idx, format!("{}={}", key, value));
        }

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, lines.join("\n") + "\n")
    }

    /// Clamps value between min and max bounds
    pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
        if value < min {
            min
        } else if value > max {
            max
        } else {
            value
        }
    }
}

/// Modifier keys and lock state tracker
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ModifierState {
    pub caps_lock: bool,
    pub num_lock: bool,
    pub alt_pressed: bool,
    pub meta_pressed: bool,
    pub ctrl_pressed: bool,
    pub shift_pressed: bool,
}

impl ModifierState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_shortcut_modifier_pressed(&self, shortcut_modifier: &str) -> bool {
        match shortcut_modifier.to_ascii_lowercase().as_str() {
            "alt" => self.alt_pressed,
            "meta" | "super" => self.meta_pressed,
            "ctrl" | "control" => self.ctrl_pressed,
            "shift" => self.shift_pressed,
            _ => false,
        }
    }
}

/// Fixed-capacity circular ring buffer
/// Ported from C++ `CircularBuffer<T>`.
#[derive(Clone, Debug)]
pub struct CircularBuffer<T> {
    capacity: usize,
    buffer: VecDeque<T>,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity: capacity.max(1),
            buffer: VecDeque::with_capacity(capacity.max(1)),
        }
    }

    pub fn push(&mut self, item: T) {
        if self.buffer.len() == self.capacity {
            self.buffer.pop_front();
        }
        self.buffer.push_back(item);
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.buffer.iter()
    }

    pub fn to_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        self.buffer.iter().cloned().collect()
    }
}

/// Application identifier canonicalization
/// Ported from C++ `appidentifiers.cpp`.
pub struct AppIdentifiers;

impl AppIdentifiers {
    /// Normalizes raw window class, instance or desktop filename to canonical app ID
    pub fn canonicalize(raw: &str) -> String {
        let trimmed = raw.trim();
        let without_desktop = trimmed.strip_suffix(".desktop").unwrap_or(trimmed);

        // Strip reverse-DNS prefixes (e.g. org.kde.dolphin -> dolphin, com.discordapp.Discord -> discord)
        let name = if let Some(last_dot) = without_desktop.rfind('.') {
            &without_desktop[last_dot + 1..]
        } else {
            without_desktop
        };

        name.to_lowercase()
    }
}
