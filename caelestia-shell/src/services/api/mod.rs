//! Top-Level Public Caelestia Shell API Gateway
//! Parity with `shell/services/api/CaelestiaApi.qml`.
//!
//! Provides the primary access point for plugins, modules, and external integrations
//! across all shell subsystems.
//!
//! ```rust
//! use caelestia_shell::services::api::CaelestiaApi;
//!
//! let api = CaelestiaApi::global();
//! let cpu = api.system.cpu_usage();
//! let is_online = api.network.is_connected();
//! ```

pub mod media;
pub mod network;
pub mod plugins;
pub mod shortcuts;
pub mod system;
pub mod ui;
pub mod visuals;
pub mod windows;

pub use media::MediaApi;
pub use network::NetworkApi;
pub use plugins::{PluginMeta, PluginsApi};
pub use shortcuts::{DynamicShortcut, ShortcutsApi};
pub use system::SystemApi;
pub use ui::{UiApi, WidgetDescriptor};
pub use visuals::VisualsApi;
pub use windows::WindowsApi;

use std::sync::OnceLock;

/// The central Caelestia Public API Singleton
#[derive(Clone)]
pub struct CaelestiaApi {
    pub system: SystemApi,
    pub windows: WindowsApi,
    pub network: NetworkApi,
    pub media: MediaApi,
    pub visuals: VisualsApi,
    pub ui: UiApi,
    pub shortcuts: ShortcutsApi,
    pub plugins: PluginsApi,
}

impl Default for CaelestiaApi {
    fn default() -> Self {
        Self::new()
    }
}

impl CaelestiaApi {
    pub fn new() -> Self {
        Self {
            system: SystemApi::new(),
            windows: WindowsApi::new(),
            network: NetworkApi::new(),
            media: MediaApi::new(),
            visuals: VisualsApi::new(),
            ui: UiApi::new(),
            shortcuts: ShortcutsApi::new(),
            plugins: PluginsApi::new(),
        }
    }

    /// Retrieve the global shared instance of CaelestiaApi
    pub fn global() -> &'static Self {
        static INSTANCE: OnceLock<CaelestiaApi> = OnceLock::new();
        INSTANCE.get_or_init(Self::new)
    }
}
