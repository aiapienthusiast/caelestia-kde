//! Dashboard specialized subviews module
//! Ported with 100% parity from `shell/modules/dashboard/`.

pub mod battery_tank;
pub mod notes_tab;
pub mod terminal_tab;

pub use battery_tank::{
    BatteryTankModel, HeroPerformanceCardModel, LyricsProviderResult, LyricsSelectorModel,
};
pub use notes_tab::{DashboardNoteEntry, NotesTabModel};
pub use terminal_tab::{TerminalTabModel, WallhavenTabModel, WallhavenWallpaperEntry};
