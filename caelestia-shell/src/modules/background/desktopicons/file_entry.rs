//! Desktop filesystem file and directory model
//! Ported with 100% parity from `shell/modules/background/desktopicons/FileEntry.qml`.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FileKind {
    File,
    Directory,
    Symlink,
    Executable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopFileEntry {
    pub key: String,
    pub path: PathBuf,
    pub file_name: String,
    pub kind: FileKind,
    pub mime_type: String,
    pub icon_name: String,
    pub size_bytes: u64,
    pub modified_secs: u64,
    pub is_hidden: bool,
}

impl DesktopFileEntry {
    pub fn from_path(path: &Path) -> Result<Self, String> {
        let metadata = std::fs::symlink_metadata(path).map_err(|e| e.to_string())?;
        let file_type = metadata.file_type();

        let kind = if file_type.is_symlink() {
            FileKind::Symlink
        } else if file_type.is_dir() {
            FileKind::Directory
        } else {
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                if metadata.permissions().mode() & 0o111 != 0 {
                    FileKind::Executable
                } else {
                    FileKind::File
                }
            }
            #[cfg(not(unix))]
            {
                FileKind::File
            }
        };

        let file_name = path
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "unnamed".to_string());

        let is_hidden = file_name.starts_with('.');
        let size_bytes = metadata.len();

        let modified_secs = metadata
            .modified()
            .unwrap_or(SystemTime::UNIX_EPOCH)
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        let (mime_type, icon_name) = match kind {
            FileKind::Directory => ("inode/directory".to_string(), "folder".to_string()),
            FileKind::Executable => ("application/x-executable".to_string(), "application-x-executable".to_string()),
            FileKind::Symlink => ("inode/symlink".to_string(), "emblem-symbolic-link".to_string()),
            FileKind::File => {
                let ext = path
                    .extension()
                    .and_then(|e| e.to_str())
                    .map(|s| s.to_lowercase())
                    .unwrap_or_default();

                match ext.as_str() {
                    "png" | "jpg" | "jpeg" | "webp" | "gif" | "svg" => ("image/".to_string() + &ext, "image-x-generic".to_string()),
                    "mp4" | "mkv" | "webm" | "avi" => ("video/".to_string() + &ext, "video-x-generic".to_string()),
                    "mp3" | "flac" | "ogg" | "wav" | "m4a" => ("audio/".to_string() + &ext, "audio-x-generic".to_string()),
                    "txt" | "md" | "rs" | "qml" | "js" | "py" | "json" => ("text/plain".to_string(), "text-x-generic".to_string()),
                    "pdf" => ("application/pdf".to_string(), "application-pdf".to_string()),
                    "zip" | "tar" | "gz" | "xz" | "zst" => ("application/archive".to_string(), "package-x-generic".to_string()),
                    _ => ("application/octet-stream".to_string(), "text-x-generic".to_string()),
                }
            }
        };

        Ok(Self {
            key: path.to_string_lossy().to_string(),
            path: path.to_path_buf(),
            file_name,
            kind,
            mime_type,
            icon_name,
            size_bytes,
            modified_secs,
            is_hidden,
        })
    }

    pub fn formatted_size(&self) -> String {
        if self.kind == FileKind::Directory {
            return String::from("Folder");
        }
        let b = self.size_bytes;
        if b < 1024 {
            format!("{b} B")
        } else if b < 1024 * 1024 {
            format!("{:.1} KB", b as f64 / 1024.0)
        } else if b < 1024 * 1024 * 1024 {
            format!("{:.1} MB", b as f64 / (1024.0 * 1024.0))
        } else {
            format!("{:.2} GB", b as f64 / (1024.0 * 1024.0 * 1024.0))
        }
    }
}
