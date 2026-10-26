//! Windows and Compositor API Bridge
//! Parity with `shell/services/api/WindowsApi.qml`.

use crate::services::kwin::{KWinService, VirtualWorkspace, WindowItem};
use std::sync::{Arc, Mutex, OnceLock};

/// Windows API providing active window focus, workspace tracking, and window controls
#[derive(Clone)]
pub struct WindowsApi {
    pub kwin: Arc<Mutex<KWinService>>,
}

impl Default for WindowsApi {
    fn default() -> Self {
        Self::new()
    }
}

impl WindowsApi {
    pub fn new() -> Self {
        Self {
            kwin: Arc::new(Mutex::new(KWinService::new())),
        }
    }

    pub fn global() -> &'static Self {
        static INSTANCE: OnceLock<WindowsApi> = OnceLock::new();
        INSTANCE.get_or_init(Self::new)
    }

    /// Retrieve the currently active / focused window
    pub fn active_window(&self) -> Option<WindowItem> {
        self.kwin.lock().ok()?.active_window().cloned()
    }

    /// Retrieve all open client windows
    pub fn windows(&self) -> Vec<WindowItem> {
        self.kwin.lock().map_or(Vec::new(), |k| k.window_list().to_vec())
    }

    /// Retrieve virtual desktop workspace state
    pub fn workspaces(&self) -> Vec<VirtualWorkspace> {
        self.kwin.lock().map_or(Vec::new(), |k| k.state.workspaces.clone())
    }

    /// Active output name
    pub fn active_output(&self) -> String {
        self.kwin.lock().map_or_else(|_| "eDP-1".to_string(), |k| k.cursor_output_name())
    }

    /// Switch active virtual desktop workspace by UUID
    pub fn switch_workspace(&self, uuid: &str) {
        if let Ok(mut k) = self.kwin.lock() {
            k.switch_to(uuid, None);
        }
    }
}
