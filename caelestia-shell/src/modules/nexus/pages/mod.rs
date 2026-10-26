//! Settings pages for Nexus Control Center.

pub mod about;
pub mod appearance;
pub mod desktop;
pub mod network;
pub mod panels;
pub mod services;
pub mod wallpapers;

pub use about::AboutPage;
pub use appearance::AppearancePage;
pub use desktop::DesktopPage;
pub use network::{NetworkSettingsPageModel, VpnConnectionProfile, WifiNetworkItem};
pub use panels::PanelsPage;
pub use services::ServicesSettingsPageModel;
pub use wallpapers::WallpaperSettingsPageModel;

