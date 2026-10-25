//! KWin Compositor Service
//! Parity with `shell/services/Kwin.qml`, `kwinworkspacestate.cpp`, and `kwinactivewindowbridge.cpp`.
//! Manages KDE KWin virtual desktops, workspaces, and window manager actions via D-Bus.

use std::process::Command;
use tracing::info;

#[derive(Clone, Debug, PartialEq)]
pub struct VirtualWorkspace {
    pub id: u32,
    pub uuid: String,
    pub name: String,
    pub active: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct KWinState {
    pub active_uuid: String,
    pub active_index: u32,
    pub count: u32,
    pub workspaces: Vec<VirtualWorkspace>,
    pub showing_desktop: bool,
}

impl Default for KWinState {
    fn default() -> Self {
        Self {
            active_uuid: String::new(),
            active_index: 0,
            count: 1,
            workspaces: Vec::new(),
            showing_desktop: false,
        }
    }
}

pub struct KWinService {
    pub state: KWinState,
}

impl KWinService {
    pub fn new() -> Self {
        let mut service = Self {
            state: KWinState::default(),
        };
        service.poll();
        service
    }

    /// Refresh virtual desktops and active workspace
    pub fn poll(&mut self) {
        // Read current desktop UUID
        if let Ok(output) = Command::new("qdbus6")
            .args(["org.kde.KWin", "/VirtualDesktopManager", "org.kde.KWin.VirtualDesktopManager.current"])
            .output()
        {
            if output.status.success() {
                self.state.active_uuid = String::from_utf8_lossy(&output.stdout).trim().to_string();
            }
        }

        // Read all virtual desktops
        if let Ok(output) = Command::new("qdbus6")
            .args(["--literal", "org.kde.KWin", "/VirtualDesktopManager", "org.kde.KWin.VirtualDesktopManager.desktops"])
            .output()
        {
            if output.status.success() {
                let text = String::from_utf8_lossy(&output.stdout);
                self.state.workspaces = Self::parse_desktops_literal(&text, &self.state.active_uuid);
                self.state.count = self.state.workspaces.len() as u32;

                if let Some(pos) = self.state.workspaces.iter().position(|ws| ws.active) {
                    self.state.active_index = pos as u32;
                }
            }
        }
    }

    /// Parse `--literal` output from `qdbus6 --literal org.kde.KWin /VirtualDesktopManager org.kde.KWin.VirtualDesktopManager.desktops`
    /// e.g. `[Variant: [Argument: a(uss) {[Argument: (uss) 0, "8b0caf0f-...", "undefined"], ...}]]`
    pub fn parse_desktops_literal(output: &str, active_uuid: &str) -> Vec<VirtualWorkspace> {
        let mut workspaces = Vec::new();

        for chunk in output.split("[Argument: (uss)") {
            if let Some(end) = chunk.find(']') {
                let inner = &chunk[..end].trim();
                let parts: Vec<&str> = inner.split(',').collect();
                if parts.len() >= 3 {
                    let id_str = parts[0].trim();
                    let uuid_str = parts[1].trim().trim_matches('"');
                    let name_str = parts[2].trim().trim_matches('"');

                    if let Ok(id) = id_str.parse::<u32>() {
                        let active = uuid_str == active_uuid;
                        let display_name = if name_str == "undefined" || name_str.is_empty() {
                            format!("Workspace {}", id + 1)
                        } else {
                            name_str.to_string()
                        };

                        workspaces.push(VirtualWorkspace {
                            id,
                            uuid: uuid_str.to_string(),
                            name: display_name,
                            active,
                        });
                    }
                }
            }
        }

        // Sort by numeric workspace index
        workspaces.sort_by_key(|w| w.id);
        workspaces
    }

    /// Switch active workspace by UUID
    pub fn switch_to_uuid(&mut self, uuid: &str) {
        info!("Switching to KWin workspace UUID: {}", uuid);
        self.state.active_uuid = uuid.to_string();
        for ws in &mut self.state.workspaces {
            ws.active = ws.uuid == uuid;
        }

        let _ = Command::new("qdbus6")
            .args(["org.kde.KWin", "/VirtualDesktopManager", "org.kde.KWin.VirtualDesktopManager.current", uuid])
            .spawn();
    }

    /// Switch active workspace by index (0-indexed)
    pub fn switch_to_index(&mut self, index: usize) {
        if let Some(ws) = self.state.workspaces.get(index) {
            let uuid = ws.uuid.clone();
            self.switch_to_uuid(&uuid);
        }
    }

    /// Toggle KWin Overview effect
    pub fn toggle_overview() {
        let _ = Command::new("qdbus6")
            .args(["org.kde.kglobalaccel", "/component/kwin", "invokeShortcut", "Overview"])
            .spawn();
    }

    /// Toggle Show Desktop
    pub fn toggle_showing_desktop(&mut self) {
        self.state.showing_desktop = !self.state.showing_desktop;
        let _ = Command::new("qdbus6")
            .args(["org.kde.kglobalaccel", "/component/kwin", "invokeShortcut", "ShowDesktopGrid"])
            .spawn();
    }
}
