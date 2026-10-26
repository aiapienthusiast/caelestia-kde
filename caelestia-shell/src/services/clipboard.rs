//! System Clipboard History & Pinning Service
//! 100% parity with C++ `Caelestia::ClipboardManager`.

use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::Mutex;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClipboardItem {
    pub id: u64,
    pub text: String,
    pub is_image: bool,
    pub is_pinned: bool,
}

pub struct ClipboardService {
    items: Mutex<Vec<ClipboardItem>>,
    pinned_items: Mutex<Vec<ClipboardItem>>,
    pins_dir: PathBuf,
}

impl Default for ClipboardService {
    fn default() -> Self {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/root".to_string());
        let pins_dir = PathBuf::from(home).join(".local/state/caelestia/clipboard/pins");
        Self::new(pins_dir)
    }
}

impl ClipboardService {
    pub fn new(pins_dir: PathBuf) -> Self {
        let service = Self {
            items: Mutex::new(Vec::new()),
            pinned_items: Mutex::new(Vec::new()),
            pins_dir,
        };
        service.load_pins();
        service.reload();
        service
    }

    pub fn items(&self) -> Vec<ClipboardItem> {
        self.items.lock().map(|i| i.clone()).unwrap_or_default()
    }

    pub fn pinned_items(&self) -> Vec<ClipboardItem> {
        self.pinned_items.lock().map(|p| p.clone()).unwrap_or_default()
    }

    /// Reloads items from system clipboard history (e.g. cliphist)
    pub fn reload(&self) {
        if let Ok(output) = Command::new("cliphist").arg("list").output() {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let mut parsed = Vec::new();

                for line in stdout.lines() {
                    if let Some((id_str, content)) = line.split_once('\t') {
                        if let Ok(id) = id_str.trim().parse::<u64>() {
                            let is_image = content.starts_with("[[ binary data");
                            parsed.push(ClipboardItem {
                                id,
                                text: content.to_string(),
                                is_image,
                                is_pinned: false,
                            });
                        }
                    }
                }

                if let Ok(mut items) = self.items.lock() {
                    *items = parsed;
                }
            }
        }
    }

    /// Copies an entry back to the active Wayland clipboard using `wl-copy`
    pub fn copy_item(&self, item: &ClipboardItem) -> Result<(), std::io::Error> {
        let mut child = Command::new("wl-copy")
            .stdin(Stdio::piped())
            .spawn()?;

        if let Some(mut stdin) = child.stdin.take() {
            use std::io::Write;
            stdin.write_all(item.text.as_bytes())?;
        }
        child.wait()?;
        Ok(())
    }

    /// Pins an item to persistent storage
    pub fn pin(&self, id: u64) {
        let item = {
            let items = self.items.lock().unwrap();
            items.iter().find(|i| i.id == id).cloned()
        };

        if let Some(mut item) = item {
            item.is_pinned = true;
            let _ = fs::create_dir_all(&self.pins_dir);
            let pin_file = self.pins_dir.join(format!("{}.txt", id));
            let _ = fs::write(pin_file, &item.text);

            if let Ok(mut pinned) = self.pinned_items.lock() {
                if !pinned.iter().any(|p| p.id == id) {
                    pinned.push(item);
                }
            }
        }
    }

    /// Unpins an item
    pub fn unpin(&self, id: u64) {
        let pin_file = self.pins_dir.join(format!("{}.txt", id));
        let _ = fs::remove_file(pin_file);

        if let Ok(mut pinned) = self.pinned_items.lock() {
            pinned.retain(|p| p.id != id);
        }
    }

    /// Clears the transient clipboard history
    pub fn clear_history(&self) -> Result<(), std::io::Error> {
        let _ = Command::new("cliphist").arg("wipe").status();
        if let Ok(mut items) = self.items.lock() {
            items.clear();
        }
        Ok(())
    }

    fn load_pins(&self) {
        if let Ok(read) = fs::read_dir(&self.pins_dir) {
            let mut pins = Vec::new();
            for item in read.flatten() {
                let path = item.path();
                if path.extension().and_then(|e| e.to_str()) == Some("txt") {
                    if let Ok(text) = fs::read_to_string(&path) {
                        let id = path
                            .file_stem()
                            .and_then(|s| s.to_str())
                            .and_then(|s| s.parse::<u64>().ok())
                            .unwrap_or(0);
                        pins.push(ClipboardItem {
                            id,
                            text,
                            is_image: false,
                            is_pinned: true,
                        });
                    }
                }
            }
            if let Ok(mut p) = self.pinned_items.lock() {
                *p = pins;
            }
        }
    }
}
