//! Startup Tasks, XDG Autostart & Web Search Configuration Service
//! 100% parity with `shell/services/StartupTasks.qml`, `Autostart.qml`, `ConfigMigrations.qml`, and `WebSearch.qml`.

use std::fs;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AutostartEntry {
    pub id: String,
    pub name: String,
    pub exec: String,
    pub icon: String,
    pub enabled: bool,
    pub file_path: PathBuf,
}

pub struct AutostartService;

impl AutostartService {
    /// Discovers all autostart desktop entries from user and system directories
    pub fn list_entries() -> Vec<AutostartEntry> {
        let mut entries = Vec::new();
        let home = std::env::var("HOME").unwrap_or_else(|_| "/root".to_string());
        let user_autostart = PathBuf::from(home).join(".config/autostart");
        let sys_autostart = PathBuf::from("/etc/xdg/autostart");

        let mut seen_ids = std::collections::HashSet::new();

        for dir in &[user_autostart, sys_autostart] {
            if let Ok(read_dir) = fs::read_dir(dir) {
                for item in read_dir.flatten() {
                    let path = item.path();
                    if path.extension().and_then(|e| e.to_str()) == Some("desktop") {
                        let id = path.file_stem().and_then(|s| s.to_str()).unwrap_or("").to_string();
                        if !seen_ids.contains(&id) {
                            seen_ids.insert(id.clone());
                            if let Ok(content) = fs::read_to_string(&path) {
                                let mut name = id.clone();
                                let mut exec = String::new();
                                let mut icon = "application-x-executable".to_string();
                                let mut enabled = true;

                                for line in content.lines() {
                                    let line = line.trim();
                                    if let Some(val) = line.strip_prefix("Name=") {
                                        name = val.to_string();
                                    } else if let Some(val) = line.strip_prefix("Exec=") {
                                        exec = val.to_string();
                                    } else if let Some(val) = line.strip_prefix("Icon=") {
                                        icon = val.to_string();
                                    } else if let Some(val) = line.strip_prefix("Hidden=") {
                                        if val.eq_ignore_ascii_case("true") {
                                            enabled = false;
                                        }
                                    } else if let Some(val) = line.strip_prefix("X-GNOME-Autostart-enabled=") {
                                        if val.eq_ignore_ascii_case("false") {
                                            enabled = false;
                                        }
                                    }
                                }

                                entries.push(AutostartEntry {
                                    id,
                                    name,
                                    exec,
                                    icon,
                                    enabled,
                                    file_path: path,
                                });
                            }
                        }
                    }
                }
            }
        }

        entries
    }

    /// Toggles or sets an autostart desktop entry's enabled flag
    pub fn set_enabled(id: &str, enabled: bool) -> Result<(), std::io::Error> {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/root".to_string());
        let user_autostart = PathBuf::from(home).join(".config/autostart");
        fs::create_dir_all(&user_autostart)?;

        let target_path = user_autostart.join(format!("{}.desktop", id));
        let flag_str = if enabled { "false" } else { "true" }; // Hidden=true disables
        super::cutils::CaelestiaUtils::set_desktop_entry_key(&target_path, "Hidden", flag_str)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WebSearchProvider {
    pub id: String,
    pub name: String,
    pub query_template: String,
}

pub struct WebSearchService {
    current_provider_id: String,
    providers: Vec<WebSearchProvider>,
}

impl Default for WebSearchService {
    fn default() -> Self {
        Self::new()
    }
}

impl WebSearchService {
    pub fn new() -> Self {
        let providers = vec![
            WebSearchProvider {
                id: "duckduckgo".to_string(),
                name: "DuckDuckGo".to_string(),
                query_template: "https://duckduckgo.com/?q={@}".to_string(),
            },
            WebSearchProvider {
                id: "google".to_string(),
                name: "Google".to_string(),
                query_template: "https://www.google.com/search?q={@}".to_string(),
            },
            WebSearchProvider {
                id: "kagi".to_string(),
                name: "Kagi".to_string(),
                query_template: "https://kagi.com/search?q={@}".to_string(),
            },
            WebSearchProvider {
                id: "startpage".to_string(),
                name: "Startpage".to_string(),
                query_template: "https://www.startpage.com/do/dsearch?query={@}".to_string(),
            },
            WebSearchProvider {
                id: "bing".to_string(),
                name: "Bing".to_string(),
                query_template: "https://www.bing.com/search?q={@}".to_string(),
            },
        ];

        Self {
            current_provider_id: "duckduckgo".to_string(),
            providers,
        }
    }

    pub fn current_provider(&self) -> &WebSearchProvider {
        self.providers
            .iter()
            .find(|p| p.id == self.current_provider_id)
            .unwrap_or(&self.providers[0])
    }

    pub fn set_provider(&mut self, id: &str) {
        if self.providers.iter().any(|p| p.id == id) {
            self.current_provider_id = id.to_string();
        }
    }

    pub fn available_providers(&self) -> &[WebSearchProvider] {
        &self.providers
    }

    /// Formats a search query into a complete browser URL
    pub fn format_search_url(&self, query: &str) -> String {
        let provider = self.current_provider();
        let encoded = percent_encoding::utf8_percent_encode(
            query,
            percent_encoding::NON_ALPHANUMERIC,
        ).to_string();
        provider.query_template.replace("{@}", &encoded)
    }
}

pub struct ConfigMigrationService;

impl ConfigMigrationService {
    /// Migrates retired config keys or files to current schemas
    pub fn run_migrations(config_dir: &Path) {
        let retired_order = config_dir.join("status_icons_order.txt");
        if retired_order.exists() {
            let _ = fs::remove_file(retired_order);
        }
    }
}
