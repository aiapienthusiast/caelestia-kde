//! Subsystem and Resource Loader Diagnostics System.
//! Provides comprehensive observability into loaded and failed components,
//! font availability, theme validity, configuration fallbacks, system services,
//! community plugins, and layer-shell surfaces.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{LazyLock, RwLock};
use tracing::{debug, error, info, warn};

/// Status of a loaded component or resource.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagnosticStatus {
    /// Component loaded cleanly and is functioning normally.
    Loaded,
    /// Component missing or non-critical failure; functioning using a fallback.
    Fallback,
    /// Critical failure or component could not be initialized.
    Failed,
    /// Component was not requested or enabled in current session mode.
    Skipped,
}

impl DiagnosticStatus {
    pub fn badge(&self) -> &'static str {
        match self {
            DiagnosticStatus::Loaded => "[✓ LOADED]  ",
            DiagnosticStatus::Fallback => "[! FALLBACK]",
            DiagnosticStatus::Failed => "[✗ FAILED]  ",
            DiagnosticStatus::Skipped => "[- SKIPPED] ",
        }
    }

    pub fn ansi_badge(&self) -> &'static str {
        match self {
            DiagnosticStatus::Loaded => "\x1b[32m[✓ LOADED]\x1b[0m  ",
            DiagnosticStatus::Fallback => "\x1b[33m[! FALLBACK]\x1b[0m",
            DiagnosticStatus::Failed => "\x1b[31m[✗ FAILED]\x1b[0m  ",
            DiagnosticStatus::Skipped => "\x1b[90m[- SKIPPED]\x1b[0m ",
        }
    }
}

/// Category of subsystem or resource being audited.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DiagnosticCategory {
    Font,
    Theme,
    Config,
    Display,
    Service,
    Plugin,
    Surface,
}

impl fmt::Display for DiagnosticCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DiagnosticCategory::Font => write!(f, "Font"),
            DiagnosticCategory::Theme => write!(f, "Theme"),
            DiagnosticCategory::Config => write!(f, "Config"),
            DiagnosticCategory::Display => write!(f, "Display"),
            DiagnosticCategory::Service => write!(f, "Service"),
            DiagnosticCategory::Plugin => write!(f, "Plugin"),
            DiagnosticCategory::Surface => write!(f, "Surface"),
        }
    }
}

/// A single diagnostic record capturing subsystem status, targets, and reasons.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticEntry {
    pub category: DiagnosticCategory,
    pub name: String,
    pub status: DiagnosticStatus,
    pub path_or_target: Option<String>,
    pub details: String,
    pub error_or_reason: Option<String>,
}

impl DiagnosticEntry {
    pub fn new(
        category: DiagnosticCategory,
        name: impl Into<String>,
        status: DiagnosticStatus,
        path_or_target: Option<String>,
        details: impl Into<String>,
        error_or_reason: Option<String>,
    ) -> Self {
        Self {
            category,
            name: name.into(),
            status,
            path_or_target,
            details: details.into(),
            error_or_reason,
        }
    }
}

/// Global registry tracking all loaded, fallback, failed, and skipped components.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LoaderDiagnostics {
    pub entries: Vec<DiagnosticEntry>,
}

static GLOBAL_DIAGNOSTICS: LazyLock<RwLock<LoaderDiagnostics>> =
    LazyLock::new(|| RwLock::new(LoaderDiagnostics::new()));

impl LoaderDiagnostics {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Add an entry to the diagnostics list.
    pub fn add(&mut self, entry: DiagnosticEntry) {
        self.entries.push(entry);
    }

    /// Record a successfully loaded component.
    pub fn record_loaded(
        &mut self,
        category: DiagnosticCategory,
        name: impl Into<String>,
        path_or_target: Option<&str>,
        details: impl Into<String>,
    ) {
        let entry = DiagnosticEntry::new(
            category,
            name,
            DiagnosticStatus::Loaded,
            path_or_target.map(|p| p.to_string()),
            details,
            None,
        );
        self.add(entry);
    }

    /// Record a component that fell back to a default or alternative implementation.
    pub fn record_fallback(
        &mut self,
        category: DiagnosticCategory,
        name: impl Into<String>,
        path_or_target: Option<&str>,
        details: impl Into<String>,
        reason: impl Into<String>,
    ) {
        let entry = DiagnosticEntry::new(
            category,
            name,
            DiagnosticStatus::Fallback,
            path_or_target.map(|p| p.to_string()),
            details,
            Some(reason.into()),
        );
        self.add(entry);
    }

    /// Record a failed component with error details.
    pub fn record_failed(
        &mut self,
        category: DiagnosticCategory,
        name: impl Into<String>,
        path_or_target: Option<&str>,
        details: impl Into<String>,
        error: impl Into<String>,
    ) {
        let entry = DiagnosticEntry::new(
            category,
            name,
            DiagnosticStatus::Failed,
            path_or_target.map(|p| p.to_string()),
            details,
            Some(error.into()),
        );
        self.add(entry);
    }

    /// Record a skipped component.
    pub fn record_skipped(
        &mut self,
        category: DiagnosticCategory,
        name: impl Into<String>,
        reason: impl Into<String>,
    ) {
        let entry = DiagnosticEntry::new(
            category,
            name,
            DiagnosticStatus::Skipped,
            None,
            "Subsystem omitted in this execution mode",
            Some(reason.into()),
        );
        self.add(entry);
    }

    /// Audit system font availability, specifically Material Symbols Rounded and UI fonts.
    pub fn audit_fonts(&mut self) {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let search_dirs = [
            PathBuf::from("/usr/share/fonts"),
            PathBuf::from("/usr/local/share/fonts"),
            PathBuf::from(format!("{}/.local/share/fonts", home)),
            PathBuf::from(format!("{}/.fonts", home)),
        ];

        // 1. Material Symbols Rounded (Vital for icon glyphs)
        let mut material_found = None;
        for dir in &search_dirs {
            if let Ok(entries) = walkdir_search(dir, "MaterialSymbols") {
                if let Some(font_file) = entries.into_iter().next() {
                    material_found = Some(font_file);
                    break;
                }
            }
        }

        if let Some(path) = material_found {
            self.record_loaded(
                DiagnosticCategory::Font,
                "Material Symbols Rounded",
                path.to_str(),
                "OpenType icon font glyphs & ligatures available",
            );
        } else {
            self.record_failed(
                DiagnosticCategory::Font,
                "Material Symbols Rounded",
                None,
                "Icon ligatures will render as literal ASCII text",
                "MaterialSymbols font files not found in /usr/share/fonts or ~/.local/share/fonts",
            );
        }

        // 2. Google Sans Flex / System UI Font
        let mut google_sans_found = None;
        for dir in &search_dirs {
            if let Ok(entries) = walkdir_search(dir, "GoogleSans") {
                if let Some(font_file) = entries.into_iter().next() {
                    google_sans_found = Some(font_file);
                    break;
                }
            }
        }

        if let Some(path) = google_sans_found {
            self.record_loaded(
                DiagnosticCategory::Font,
                "Google Sans Flex",
                path.to_str(),
                "Primary UI font family available",
            );
        } else {
            self.record_fallback(
                DiagnosticCategory::Font,
                "Google Sans Flex",
                None,
                "Using system generic sans-serif fallback font",
                "GoogleSans font files not present locally",
            );
        }

        // 3. Monospace font
        let mut mono_found = None;
        for pattern in &["JetBrains", "FiraCode", "Hack", "NotoSansMono"] {
            for dir in &search_dirs {
                if let Ok(entries) = walkdir_search(dir, pattern) {
                    if let Some(font_file) = entries.into_iter().next() {
                        mono_found = Some((pattern.to_string(), font_file));
                        break;
                    }
                }
            }
            if mono_found.is_some() {
                break;
            }
        }

        if let Some((name, path)) = mono_found {
            self.record_loaded(
                DiagnosticCategory::Font,
                format!("Monospace ({})", name),
                path.to_str(),
                "Code and terminal monospace font available",
            );
        } else {
            self.record_fallback(
                DiagnosticCategory::Font,
                "Monospace Font",
                None,
                "Using system generic monospace font",
                "No preferred monospace fonts (JetBrains, FiraCode) found",
            );
        }
    }

    /// Audit shell configuration file (`~/.config/caelestia/shell.json`).
    pub fn audit_config(&mut self) {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let config_path = PathBuf::from(format!("{}/.config/caelestia/shell.json", home));

        if !config_path.exists() {
            self.record_fallback(
                DiagnosticCategory::Config,
                "shell.json",
                config_path.to_str(),
                "Using default built-in ShellConfig parameters",
                "File does not exist",
            );
            return;
        }

        match fs::read_to_string(&config_path) {
            Ok(content) => match serde_json::from_str::<serde_json::Value>(&content) {
                Ok(val) => {
                    let has_appearance = val.get("appearance").is_some();
                    let has_border = val.get("border").is_some();
                    self.record_loaded(
                        DiagnosticCategory::Config,
                        "shell.json",
                        config_path.to_str(),
                        format!(
                            "Valid JSON configuration loaded (appearance: {}, border: {})",
                            has_appearance, has_border
                        ),
                    );
                }
                Err(err) => {
                    self.record_fallback(
                        DiagnosticCategory::Config,
                        "shell.json",
                        config_path.to_str(),
                        "Using default built-in ShellConfig parameters",
                        format!("JSON syntax error at line {}:{}: {}", err.line(), err.column(), err),
                    );
                }
            },
            Err(err) => {
                self.record_fallback(
                    DiagnosticCategory::Config,
                    "shell.json",
                    config_path.to_str(),
                    "Using default built-in ShellConfig parameters",
                    format!("Failed to read file: {}", err),
                );
            }
        }
    }

    /// Audit Material 3 theme scheme (`~/.local/state/caelestia/scheme.json`).
    pub fn audit_theme(&mut self) {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let scheme_path = PathBuf::from(format!("{}/.local/state/caelestia/scheme.json", home));

        if !scheme_path.exists() {
            self.record_fallback(
                DiagnosticCategory::Theme,
                "scheme.json",
                scheme_path.to_str(),
                "Using hardcoded default Material 3 Dark theme",
                "File does not exist",
            );
            return;
        }

        match fs::read_to_string(&scheme_path) {
            Ok(content) => match serde_json::from_str::<serde_json::Value>(&content) {
                Ok(val) => {
                    let colors = val.get("colours").or_else(|| val.get("colors"));
                    if let Some(c) = colors {
                        let has_primary = c.get("primary").is_some();
                        let has_surface = c.get("surface").is_some();
                        if has_primary && has_surface {
                            self.record_loaded(
                                DiagnosticCategory::Theme,
                                "scheme.json",
                                scheme_path.to_str(),
                                "Material 3 color palette validated and loaded",
                            );
                        } else {
                            self.record_fallback(
                                DiagnosticCategory::Theme,
                                "scheme.json",
                                scheme_path.to_str(),
                                "Using partial defaults for missing color tokens",
                                "Missing primary or surface token definitions",
                            );
                        }
                    } else {
                        self.record_fallback(
                            DiagnosticCategory::Theme,
                            "scheme.json",
                            scheme_path.to_str(),
                            "Using default Material 3 Dark theme",
                            "Missing 'colours' or 'colors' root object",
                        );
                    }
                }
                Err(err) => {
                    self.record_fallback(
                        DiagnosticCategory::Theme,
                        "scheme.json",
                        scheme_path.to_str(),
                        "Using default Material 3 Dark theme",
                        format!("JSON syntax error at line {}:{}: {}", err.line(), err.column(), err),
                    );
                }
            },
            Err(err) => {
                self.record_fallback(
                    DiagnosticCategory::Theme,
                    "scheme.json",
                    scheme_path.to_str(),
                    "Using default Material 3 Dark theme",
                    format!("Failed to read theme file: {}", err),
                );
            }
        }
    }

    /// Audit display server, compositor connection, and Wayland environment.
    pub fn audit_display(&mut self) {
        let wayland_display = std::env::var("WAYLAND_DISPLAY");
        let xdg_runtime = std::env::var("XDG_RUNTIME_DIR");

        match (wayland_display, xdg_runtime) {
            (Ok(disp), Ok(runtime)) => {
                let socket_path = PathBuf::from(&runtime).join(&disp);
                if socket_path.exists() {
                    self.record_loaded(
                        DiagnosticCategory::Display,
                        format!("Wayland Socket ({})", disp),
                        socket_path.to_str(),
                        "Wayland display socket accessible for layer-shell surface creation",
                    );
                } else {
                    self.record_failed(
                        DiagnosticCategory::Display,
                        format!("Wayland Socket ({})", disp),
                        socket_path.to_str(),
                        "Layer-shell client connection will fail",
                        "Wayland socket file does not exist at path",
                    );
                }
            }
            (Err(_), _) => {
                self.record_failed(
                    DiagnosticCategory::Display,
                    "Wayland Display",
                    None,
                    "Shell cannot attach to Wayland compositor",
                    "WAYLAND_DISPLAY environment variable is unset",
                );
            }
            (_, Err(_)) => {
                self.record_fallback(
                    DiagnosticCategory::Display,
                    "XDG_RUNTIME_DIR",
                    None,
                    "Guessing runtime path /tmp/caelestia-*",
                    "XDG_RUNTIME_DIR environment variable is unset",
                );
            }
        }

        if let Ok(desktop) = std::env::var("XDG_CURRENT_DESKTOP") {
            self.record_loaded(
                DiagnosticCategory::Display,
                "Desktop Environment",
                None,
                format!("Compositor environment: {}", desktop),
            );
        }
    }

    /// Audit system services and hardware endpoints (Audio, Network, Battery, D-Bus).
    pub fn audit_services(&mut self) {
        let uid = std::env::var("UID").unwrap_or_else(|_| "1000".to_string());
        let runtime = std::env::var("XDG_RUNTIME_DIR")
            .unwrap_or_else(|_| format!("/run/user/{}", uid));

        // 1. PipeWire Audio Socket
        let pw_socket = PathBuf::from(&runtime).join("pipewire-0");
        let pulse_socket = PathBuf::from(&runtime).join("pulse/native");
        if pw_socket.exists() {
            self.record_loaded(
                DiagnosticCategory::Service,
                "PipeWire Audio",
                pw_socket.to_str(),
                "Native PipeWire daemon socket active",
            );
        } else if pulse_socket.exists() {
            self.record_loaded(
                DiagnosticCategory::Service,
                "PulseAudio Emulation",
                pulse_socket.to_str(),
                "PulseAudio compatibility socket active",
            );
        } else {
            self.record_fallback(
                DiagnosticCategory::Service,
                "Audio Server",
                None,
                "Volume sliders and audio spectrum visualizers will remain inert",
                "Neither pipewire-0 nor pulse/native socket found in runtime directory",
            );
        }

        // 2. D-Bus Session Bus
        let session_bus = PathBuf::from(&runtime).join("bus");
        if session_bus.exists() || std::env::var("DBUS_SESSION_BUS_ADDRESS").is_ok() {
            self.record_loaded(
                DiagnosticCategory::Service,
                "D-Bus Session Bus",
                session_bus.to_str(),
                "IPC bus active for MPRIS, KWin, and Notifications",
            );
        } else {
            self.record_failed(
                DiagnosticCategory::Service,
                "D-Bus Session Bus",
                None,
                "Desktop services relying on IPC will be disabled",
                "D-Bus session bus socket unreachable",
            );
        }

        // 3. Network Subsystem (/sys/class/net)
        let net_dir = Path::new("/sys/class/net");
        if net_dir.exists() {
            if let Ok(entries) = fs::read_dir(net_dir) {
                let ifaces: Vec<String> = entries
                    .filter_map(|e| e.ok())
                    .map(|e| e.file_name().to_string_lossy().to_string())
                    .filter(|n| n != "lo")
                    .collect();
                if !ifaces.is_empty() {
                    self.record_loaded(
                        DiagnosticCategory::Service,
                        "Network Interfaces",
                        net_dir.to_str(),
                        format!("Active interfaces detected: {}", ifaces.join(", ")),
                    );
                } else {
                    self.record_fallback(
                        DiagnosticCategory::Service,
                        "Network Interfaces",
                        net_dir.to_str(),
                        "Network indicators will report disconnected state",
                        "No non-loopback network interfaces present",
                    );
                }
            }
        } else {
            self.record_fallback(
                DiagnosticCategory::Service,
                "Network Interfaces",
                None,
                "Network monitoring unavailable",
                "/sys/class/net directory does not exist",
            );
        }

        // 4. Power & Battery Subsystem (/sys/class/power_supply)
        let power_dir = Path::new("/sys/class/power_supply");
        if power_dir.exists() {
            if let Ok(entries) = fs::read_dir(power_dir) {
                let supplies: Vec<String> = entries
                    .filter_map(|e| e.ok())
                    .map(|e| e.file_name().to_string_lossy().to_string())
                    .collect();
                if supplies.iter().any(|s| s.starts_with("BAT")) {
                    self.record_loaded(
                        DiagnosticCategory::Service,
                        "Battery Subsystem",
                        power_dir.to_str(),
                        format!("Battery devices discovered: {}", supplies.join(", ")),
                    );
                } else if !supplies.is_empty() {
                    self.record_loaded(
                        DiagnosticCategory::Service,
                        "Power Supply (AC/Desktop)",
                        power_dir.to_str(),
                        format!("Mains power supplies detected: {}", supplies.join(", ")),
                    );
                } else {
                    self.record_fallback(
                        DiagnosticCategory::Service,
                        "Power Subsystem",
                        power_dir.to_str(),
                        "Battery popout will display stationary wall-power icon",
                        "No power supplies registered in sysfs",
                    );
                }
            }
        } else {
            self.record_fallback(
                DiagnosticCategory::Service,
                "Power Subsystem",
                None,
                "Battery monitoring unavailable",
                "/sys/class/power_supply sysfs node missing",
            );
        }

        // 5. Backlight & Brightness (/sys/class/backlight)
        let backlight_dir = Path::new("/sys/class/backlight");
        if backlight_dir.exists() {
            if let Ok(entries) = fs::read_dir(backlight_dir) {
                let devices: Vec<String> = entries
                    .filter_map(|e| e.ok())
                    .map(|e| e.file_name().to_string_lossy().to_string())
                    .collect();
                if !devices.is_empty() {
                    self.record_loaded(
                        DiagnosticCategory::Service,
                        "Backlight Controller",
                        backlight_dir.to_str(),
                        format!("Hardware brightness devices: {}", devices.join(", ")),
                    );
                } else {
                    self.record_fallback(
                        DiagnosticCategory::Service,
                        "Backlight Controller",
                        backlight_dir.to_str(),
                        "Software brightness or DDC/CI monitor control fallback",
                        "No hardware backlight devices registered in sysfs",
                    );
                }
            }
        } else {
            self.record_fallback(
                DiagnosticCategory::Service,
                "Backlight Controller",
                None,
                "Brightness controls using fallback software dimming",
                "/sys/class/backlight not present",
            );
        }
    }

    /// Audit community plugins and validate their manifests (`~/.config/caelestia/plugins`).
    pub fn audit_plugins(&mut self) {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let plugin_dirs = [
            PathBuf::from(format!("{}/.config/caelestia/plugins", home)),
            PathBuf::from(format!("{}/.local/share/caelestia/plugins", home)),
            PathBuf::from("/usr/share/caelestia/plugins"),
        ];

        let mut scanned_count = 0;
        let mut loaded_count = 0;

        for dir in &plugin_dirs {
            if !dir.exists() {
                continue;
            }
            if let Ok(entries) = fs::read_dir(dir) {
                for entry in entries.filter_map(|e| e.ok()) {
                    let path = entry.path();
                    if !path.is_dir() {
                        continue;
                    }
                    scanned_count += 1;
                    let plugin_name = entry.file_name().to_string_lossy().to_string();
                    let metadata_file = path.join("metadata.json");
                    let manifest_file = path.join("plugin.json");

                    let target_manifest = if metadata_file.exists() {
                        Some(metadata_file)
                    } else if manifest_file.exists() {
                        Some(manifest_file)
                    } else {
                        None
                    };

                    match target_manifest {
                        Some(m_path) => {
                            match fs::read_to_string(&m_path) {
                                Ok(content) => {
                                    match serde_json::from_str::<serde_json::Value>(&content) {
                                        Ok(val) => {
                                            let name = val
                                                .get("name")
                                                .and_then(|v| v.as_str())
                                                .unwrap_or(&plugin_name);
                                            let ver = val
                                                .get("version")
                                                .and_then(|v| v.as_str())
                                                .unwrap_or("0.1.0");
                                            let desc = val
                                                .get("description")
                                                .and_then(|v| v.as_str())
                                                .unwrap_or("Community shell plugin");
                                            self.record_loaded(
                                                DiagnosticCategory::Plugin,
                                                format!("{} (v{})", name, ver),
                                                path.to_str(),
                                                desc.to_string(),
                                            );
                                            loaded_count += 1;
                                        }
                                        Err(err) => {
                                            self.record_failed(
                                                DiagnosticCategory::Plugin,
                                                plugin_name,
                                                m_path.to_str(),
                                                "Plugin disabled due to invalid manifest",
                                                format!(
                                                    "JSON syntax error at line {}:{}: {}",
                                                    err.line(),
                                                    err.column(),
                                                    err
                                                ),
                                            );
                                        }
                                    }
                                }
                                Err(err) => {
                                    self.record_failed(
                                        DiagnosticCategory::Plugin,
                                        plugin_name,
                                        m_path.to_str(),
                                        "Could not read plugin manifest file",
                                        format!("{}", err),
                                    );
                                }
                            }
                        }
                        None => {
                            self.record_fallback(
                                DiagnosticCategory::Plugin,
                                plugin_name,
                                path.to_str(),
                                "Plugin loaded in unmanaged mode",
                                "Missing metadata.json or plugin.json descriptor",
                            );
                        }
                    }
                }
            }
        }

        if scanned_count == 0 {
            self.record_loaded(
                DiagnosticCategory::Plugin,
                "Community Plugins",
                None,
                "0 plugins configured; running vanilla core desktop shell",
            );
        } else {
            debug!(
                scanned = scanned_count,
                loaded = loaded_count,
                "Plugin scan completed"
            );
        }
    }

    /// Run full system diagnostics probe across all subsystems.
    pub fn audit_all(&mut self) {
        self.audit_fonts();
        self.audit_config();
        self.audit_theme();
        self.audit_display();
        self.audit_services();
        self.audit_plugins();
    }

    /// Counts by status: (loaded, fallback, failed, skipped).
    pub fn counts(&self) -> (usize, usize, usize, usize) {
        let mut loaded = 0;
        let mut fallback = 0;
        let mut failed = 0;
        let mut skipped = 0;
        for entry in &self.entries {
            match entry.status {
                DiagnosticStatus::Loaded => loaded += 1,
                DiagnosticStatus::Fallback => fallback += 1,
                DiagnosticStatus::Failed => failed += 1,
                DiagnosticStatus::Skipped => skipped += 1,
            }
        }
        (loaded, fallback, failed, skipped)
    }

    /// Output diagnostics to tracing logs at appropriate log levels.
    pub fn log_diagnostics(&self) {
        for entry in &self.entries {
            match entry.status {
                DiagnosticStatus::Loaded => {
                    info!(
                        category = %entry.category,
                        name = %entry.name,
                        target = ?entry.path_or_target,
                        details = %entry.details,
                        "Resource LOADED successfully"
                    );
                }
                DiagnosticStatus::Fallback => {
                    warn!(
                        category = %entry.category,
                        name = %entry.name,
                        target = ?entry.path_or_target,
                        reason = ?entry.error_or_reason,
                        details = %entry.details,
                        "Resource FALLBACK active"
                    );
                }
                DiagnosticStatus::Failed => {
                    error!(
                        category = %entry.category,
                        name = %entry.name,
                        target = ?entry.path_or_target,
                        error = ?entry.error_or_reason,
                        details = %entry.details,
                        "Resource FAILED to load"
                    );
                }
                DiagnosticStatus::Skipped => {
                    debug!(
                        category = %entry.category,
                        name = %entry.name,
                        reason = ?entry.error_or_reason,
                        "Resource SKIPPED"
                    );
                }
            }
        }
    }

    /// Render a formatted ASCII table report for terminal diagnostics.
    pub fn format_report(&self, use_ansi: bool) -> String {
        let (loaded, fallback, failed, skipped) = self.counts();
        let mut out = String::new();

        out.push_str("====================================================================================================\n");
        out.push_str("                   CAELESTIA SHELL SUBSYSTEM & RESOURCE LOADER DIAGNOSTICS\n");
        out.push_str("====================================================================================================\n");
        out.push_str(&format!(
            "{:<10} {:<14} {:<24} {}\n",
            "CATEGORY", "STATUS", "SUBSYSTEM / ASSET", "TARGET PATH & DETAILS / REASON"
        ));
        out.push_str("----------------------------------------------------------------------------------------------------\n");

        for entry in &self.entries {
            let badge = if use_ansi {
                entry.status.ansi_badge()
            } else {
                entry.status.badge()
            };

            let path_info = match (&entry.path_or_target, &entry.error_or_reason) {
                (Some(path), Some(reason)) => format!("{} (Reason: {})", path, reason),
                (Some(path), None) => format!("{} - {}", path, entry.details),
                (None, Some(reason)) => format!("{} (Reason: {})", entry.details, reason),
                (None, None) => entry.details.clone(),
            };

            out.push_str(&format!(
                "{:<10} {} {:<24} {}\n",
                entry.category.to_string(),
                badge,
                entry.name,
                path_info
            ));
        }

        out.push_str("====================================================================================================\n");
        if use_ansi {
            out.push_str(&format!(
                " SUMMARY: \x1b[32m{} Loaded\x1b[0m, \x1b[33m{} Fallback\x1b[0m, \x1b[31m{} Failed\x1b[0m, \x1b[90m{} Skipped\x1b[0m (Total: {})\n",
                loaded, fallback, failed, skipped, self.entries.len()
            ));
        } else {
            out.push_str(&format!(
                " SUMMARY: {} Loaded, {} Fallback, {} Failed, {} Skipped (Total: {})\n",
                loaded, fallback, failed, skipped, self.entries.len()
            ));
        }
        out.push_str("====================================================================================================\n");

        out
    }

    /// Serialize report entries as structured JSON.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

// Global accessor functions
pub fn global_diagnostics() -> &'static RwLock<LoaderDiagnostics> {
    &GLOBAL_DIAGNOSTICS
}

pub fn global_record_loaded(
    category: DiagnosticCategory,
    name: impl Into<String>,
    path_or_target: Option<&str>,
    details: impl Into<String>,
) {
    if let Ok(mut lock) = GLOBAL_DIAGNOSTICS.write() {
        lock.record_loaded(category, name, path_or_target, details);
    }
}

pub fn global_record_fallback(
    category: DiagnosticCategory,
    name: impl Into<String>,
    path_or_target: Option<&str>,
    details: impl Into<String>,
    reason: impl Into<String>,
) {
    if let Ok(mut lock) = GLOBAL_DIAGNOSTICS.write() {
        lock.record_fallback(category, name, path_or_target, details, reason);
    }
}

pub fn global_record_failed(
    category: DiagnosticCategory,
    name: impl Into<String>,
    path_or_target: Option<&str>,
    details: impl Into<String>,
    error: impl Into<String>,
) {
    if let Ok(mut lock) = GLOBAL_DIAGNOSTICS.write() {
        lock.record_failed(category, name, path_or_target, details, error);
    }
}

pub fn global_record_skipped(
    category: DiagnosticCategory,
    name: impl Into<String>,
    reason: impl Into<String>,
) {
    if let Ok(mut lock) = GLOBAL_DIAGNOSTICS.write() {
        lock.record_skipped(category, name, reason);
    }
}

/// Helper function to recursively search for files matching substring.
fn walkdir_search(dir: &Path, pattern: &str) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut matches = Vec::new();
    if !dir.exists() {
        return Ok(matches);
    }
    let mut stack = vec![dir.to_path_buf()];
    while let Some(current) = stack.pop() {
        if let Ok(entries) = fs::read_dir(&current) {
            for entry in entries.filter_map(|e| e.ok()) {
                let path = entry.path();
                if path.is_dir() {
                    stack.push(path);
                } else if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                    if file_name.contains(pattern) {
                        matches.push(path);
                    }
                }
            }
        }
    }
    Ok(matches)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagnostics_recording_and_counts() {
        let mut diag = LoaderDiagnostics::new();
        diag.record_loaded(DiagnosticCategory::Font, "TestFont", Some("/path/to/font.ttf"), "Loaded OK");
        diag.record_fallback(
            DiagnosticCategory::Config,
            "shell.json",
            None,
            "Using default",
            "File missing",
        );
        diag.record_failed(
            DiagnosticCategory::Display,
            "Wayland",
            None,
            "Cannot connect",
            "Socket disconnected",
        );
        diag.record_skipped(DiagnosticCategory::Surface, "Launcher", "Flag disabled");

        let (loaded, fallback, failed, skipped) = diag.counts();
        assert_eq!(loaded, 1);
        assert_eq!(fallback, 1);
        assert_eq!(failed, 1);
        assert_eq!(skipped, 1);

        let report = diag.format_report(false);
        assert!(report.contains("CAELESTIA SHELL SUBSYSTEM & RESOURCE LOADER DIAGNOSTICS"));
        assert!(report.contains("TestFont"));
        assert!(report.contains("shell.json"));
        assert!(report.contains("Wayland"));
        assert!(report.contains("Launcher"));
        assert!(report.contains("File missing"));
    }

    #[test]
    fn test_audit_all_generates_entries() {
        let mut diag = LoaderDiagnostics::new();
        diag.audit_all();
        assert!(!diag.entries.is_empty(), "audit_all should discover system assets");

        let json = diag.to_json().expect("Serialization to JSON must succeed");
        assert!(json.contains("entries"));
    }
}
