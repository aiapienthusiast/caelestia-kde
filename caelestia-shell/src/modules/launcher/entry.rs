//! Desktop Entry Model and Parser
//! Parity with `DesktopEntry` in `shell/modules/launcher/services/Apps.qml`.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopEntry {
    pub id: String,
    pub name: String,
    pub generic_name: String,
    pub comment: String,
    pub icon: String,
    pub exec: String,
    pub categories: Vec<String>,
    pub keywords: Vec<String>,
    pub run_in_terminal: bool,
    pub no_display: bool,
    pub path: Option<PathBuf>,
}

impl DesktopEntry {
    /// Parse a Freedesktop .desktop INI file content
    pub fn parse(id: &str, content: &str, path: Option<PathBuf>) -> Option<Self> {
        let mut in_desktop_entry = false;
        let mut entry = DesktopEntry {
            id: id.to_string(),
            path,
            ..Default::default()
        };

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if line.starts_with('[') && line.ends_with(']') {
                in_desktop_entry = line == "[Desktop Entry]";
                continue;
            }

            if !in_desktop_entry {
                continue;
            }

            if let Some((key, val)) = line.split_once('=') {
                let key = key.trim();
                let val = val.trim();

                match key {
                    "Name" if entry.name.is_empty() => entry.name = val.to_string(),
                    "GenericName" => entry.generic_name = val.to_string(),
                    "Comment" => entry.comment = val.to_string(),
                    "Icon" => entry.icon = val.to_string(),
                    "Exec" => entry.exec = val.to_string(),
                    "Terminal" => entry.run_in_terminal = val.eq_ignore_ascii_case("true"),
                    "NoDisplay" => entry.no_display = val.eq_ignore_ascii_case("true"),
                    "Categories" => {
                        entry.categories = val
                            .split(';')
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect();
                    }
                    "Keywords" => {
                        entry.keywords = val
                            .split(';')
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect();
                    }
                    _ => {}
                }
            }
        }

        if entry.name.is_empty() || entry.exec.is_empty() || entry.no_display {
            None
        } else {
            Some(entry)
        }
    }

    /// Load entry from a file on disk
    pub fn load_file(path: &Path) -> Option<Self> {
        let content = fs::read_to_string(path).ok()?;
        let id = path.file_stem()?.to_str()?;
        Self::parse(id, &content, Some(path.to_path_buf()))
    }

    /// Scan standard application search directories
    pub fn scan_system_applications() -> Vec<Self> {
        let mut entries = Vec::new();
        let home_apps = std::env::var("HOME")
            .ok()
            .map(|h| PathBuf::from(h).join(".local/share/applications"));
        let dirs = [
            home_apps,
            Some(PathBuf::from("/usr/local/share/applications")),
            Some(PathBuf::from("/usr/share/applications")),
        ];

        for dir in dirs.into_iter().flatten() {
            if let Ok(read_dir) = fs::read_dir(dir) {
                for entry in read_dir.flatten() {
                    let path = entry.path();
                    if path.extension().and_then(|ext| ext.to_str()) == Some("desktop") {
                        if let Some(app) = Self::load_file(&path) {
                            if !entries.iter().any(|e: &DesktopEntry| e.id == app.id) {
                                entries.push(app);
                            }
                        }
                    }
                }
            }
        }

        entries.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        entries
    }
}
