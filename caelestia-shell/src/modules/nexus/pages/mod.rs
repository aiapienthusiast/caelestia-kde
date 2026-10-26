//! Settings pages for Nexus Control Center.

pub mod about;
pub mod appearance;
pub mod desktop;
pub mod hardware;
pub mod network;
pub mod panels;
pub mod services;
pub mod wallpapers;

pub use about::AboutPage;
pub use appearance::AppearancePage;
pub use desktop::DesktopPage;
pub use hardware::{AppsPage, AudioPage, BluetoothPage, DisplayPage, PowerPage};
pub use network::{NetworkPage, NetworkSettingsPageModel, VpnConnectionProfile, WifiNetworkItem};
pub use panels::PanelsPage;
pub use services::{ServicesPage, ServicesSettingsPageModel};
pub use wallpapers::WallpaperSettingsPageModel;


