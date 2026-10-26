//! Desktop application shortcut icon model
//! Ported with 100% parity from `shell/modules/background/desktopicons/EntryIcon.qml`.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopEntryIcon {
    pub key: String,
    pub name: String,
    pub icon_name: String,
    pub exec_cmd: String,
    pub desktop_file_path: Option<PathBuf>,
    pub comment: Option<String>,
    pub terminal: bool,
    pub is_pinned: bool,
}

impl DesktopEntryIcon {
    pub fn new(key: impl Into<String>, name: impl Into<String>, icon: impl Into<String>, exec: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            name: name.into(),
            icon_name: icon.into(),
            exec_cmd: exec.into(),
            desktop_file_path: None,
            comment: None,
            terminal: false,
            is_pinned: false,
        }
    }

    /// Parse a freedesktop .desktop entry file
    pub fn from_desktop_file(path: &Path) -> Result<Self, String> {
        let content = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        let mut name = String::new();
        let mut icon = String::from("application-x-executable");
        let mut exec = String::new();
        let mut comment = None;
        let mut terminal = false;
        let mut in_desktop_entry = false;

        for line in content.lines() {
            let line = line.trim();
            if line == "[Desktop Entry]" {
                in_desktop_entry = true;
                continue;
            } else if line.starts_with('[') && line.ends_with(']') {
                in_desktop_entry = false;
                continue;
            }

            if in_desktop_entry {
                if let Some((k, v)) = line.split_once('=') {
                    match k.trim() {
                        "Name" if name.is_empty() => name = v.trim().to_string(),
                        "Icon" => icon = v.trim().to_string(),
                        "Exec" => exec = v.trim().to_string(),
                        "Comment" => comment = Some(v.trim().to_string()),
                        "Terminal" => terminal = v.trim().eq_ignore_ascii_case("true"),
                        _ => {}
                    }
                }
            }
        }

        if name.is_empty() {
            name = path
                .file_stem()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_else(|| "Unknown".to_string());
        }

        let key = path.to_string_lossy().to_string();

        Ok(Self {
            key,
            name,
            icon_name: icon,
            exec_cmd: exec,
            desktop_file_path: Some(path.to_path_buf()),
            comment,
            terminal,
            is_pinned: false,
        })
    }

    /// Build command string to launch the app, stripping field codes (%u, %f, etc.)
    pub fn build_launch_command(&self) -> String {
        let mut parts = Vec::new();
        for token in self.exec_cmd.split_whitespace() {
            if token.starts_with('%') {
                continue;
            }
            parts.push(token);
        }

        if self.terminal {
            format!("konsole -e {}", parts.join(" "))
        } else {
            parts.join(" ")
        }
    }
}
