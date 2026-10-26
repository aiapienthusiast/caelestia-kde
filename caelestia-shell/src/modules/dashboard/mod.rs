//! Dashboard Subsystem
//! 100% parity with `shell/modules/dashboard/` and `shell/modules/drawers/`.

pub mod dash;
pub mod drawer;
pub mod media;
pub mod performance;
pub mod subviews;
pub mod tabs;
pub mod view;
pub mod weather_tab;

pub use dash::DashView;
pub use drawer::DashboardDrawer;
pub use media::MediaView;
pub use performance::PerformanceView;
pub use subviews::{
    BatteryTankModel, DashboardNoteEntry, HeroPerformanceCardModel, LyricsProviderResult,
    LyricsSelectorModel, NotesTabModel, TerminalTabModel, WallhavenTabModel,
    WallhavenWallpaperEntry,
};
pub use tabs::DashboardTab;
pub use view::DashboardView;
pub use weather_tab::WeatherTabView;

