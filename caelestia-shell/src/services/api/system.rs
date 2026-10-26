//! System API Bridge for Plugins and Shell Subsystems
//! Parity with `shell/services/api/SystemApi.qml`.

use crate::domain::session::SessionActionKind;
use crate::infrastructure::sysinfo::SystemInfo;
use crate::services::cutils::CaelestiaUtils;
use crate::services::session::SessionService;
use crate::services::system_monitor::SystemMonitorService;
use crate::services::time_service::{TimeService, TimeSnapshot};
use crate::services::update_checker::UpdateCheckerService;
use std::sync::{Arc, Mutex, OnceLock};

/// System API providing hardware metrics, session control, time, and system upgrades
#[derive(Clone)]
pub struct SystemApi {
    pub monitor: Arc<Mutex<SystemMonitorService>>,
    pub update: Arc<Mutex<UpdateCheckerService>>,
}

impl Default for SystemApi {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemApi {
    pub fn new() -> Self {
        Self {
            monitor: Arc::new(Mutex::new(SystemMonitorService::new())),
            update: Arc::new(Mutex::new(UpdateCheckerService::new())),
        }
    }

    pub fn global() -> &'static Self {
        static INSTANCE: OnceLock<SystemApi> = OnceLock::new();
        INSTANCE.get_or_init(Self::new)
    }

    /// Retrieve live hardware metrics snapshot
    pub fn get_sys_info(&self) -> SystemInfo {
        SystemInfo::load()
    }

    /// Read current CPU usage percentage
    pub fn cpu_usage(&self) -> f32 {
        self.monitor.lock().map_or(0.0, |m| m.state.cpu_usage)
    }

    /// Read current Memory usage percentage (0.0 to 1.0)
    pub fn memory_usage(&self) -> f32 {
        self.monitor.lock().map_or(0.0, |m| {
            if m.state.ram_total_bytes > 0 {
                m.state.ram_used_bytes as f32 / m.state.ram_total_bytes as f32
            } else {
                0.0
            }
        })
    }

    /// Read current GPU usage percentage
    pub fn gpu_usage(&self) -> f32 {
        self.monitor.lock().map_or(0.0, |m| m.state.gpu_usage)
    }

    /// Read root storage usage percentage (0.0 to 1.0)
    pub fn storage_usage(&self) -> f32 {
        self.monitor.lock().map_or(0.0, |m| {
            if m.state.disk_total_bytes > 0 {
                m.state.disk_used_bytes as f32 / m.state.disk_total_bytes as f32
            } else {
                0.0
            }
        })
    }

    /// Check if system package upgrades are pending
    pub fn pending_updates_count(&self) -> usize {
        self.update.lock().map_or(0, |u| u.pending_count)
    }

    /// Execute desktop power / session action
    pub fn session_action(&self, action: SessionActionKind) {
        SessionService::execute(action);
    }

    /// Compute SHA256 of bytes
    pub fn sha256(&self, bytes: &[u8]) -> String {
        CaelestiaUtils::sha256_bytes(bytes)
    }

    /// Current time snapshot
    pub fn current_time(&self, twelve_hour: bool, show_seconds: bool) -> TimeSnapshot {
        TimeService::snapshot(twelve_hour, show_seconds)
    }
}
