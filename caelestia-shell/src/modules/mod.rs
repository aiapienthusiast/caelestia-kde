//! Caelestia Shell Modules Subsystem
//! 100% complete parity with `shell/modules/`.

// Root Module Controllers
pub mod battery_monitor;
pub mod bluetooth_reconnect;
pub mod gsf_loader;
pub mod screen_corners;
pub mod service_loader;
pub mod shortcuts;

// Module Subsystems
pub mod background;
pub mod bar;
pub mod dashboard;
pub mod drawers;
pub mod launcher;
pub mod nexus;
pub mod notifications;
pub mod osd;
pub mod overview;
pub mod plugins;
pub mod screenshot;
pub mod session;
pub mod sidebar;
pub mod utilities;
pub mod whatsnew;
pub mod windowinfo;

// Re-exports of root module controllers
pub use battery_monitor::{BatteryMonitor, BatteryToastEvent, BatteryWarnLevel};
pub use bluetooth_reconnect::BluetoothReconnectCoordinator;
pub use gsf_loader::GsfFontLoader;
pub use screen_corners::{ScreenCorner, ScreenCornersConfig, ScreenCornersCoordinator};
pub use service_loader::{ServiceLoader, ServicePhase, ShellServiceId};
pub use shortcuts::{
    GlobalShortcutsModule, KrohnkiteAction, ShellShortcutAction, ShortcutDefinition,
    ShortcutManager,
};

