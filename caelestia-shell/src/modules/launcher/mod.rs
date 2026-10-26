//! Application Launcher Subsystem
//! Complete parity with `shell/modules/launcher/`.
//!
//! Provides desktop entry scanning, XDG category curation, fuzzy and prefix search,
//! process spawning with Freedesktop field code stripping, and the drawer view.

pub mod categories;
pub mod entry;
pub mod executor;
pub mod items;
pub mod searcher;
pub mod view;
pub mod views;

pub use categories::{Categories, CategoryDef, CATEGORY_DEFINITIONS};
pub use entry::DesktopEntry;
pub use executor::AppExecutor;
pub use items::{
    ActionLauncherItem, CalcResultItem, ClipboardResultItem, EmojiResultItem, LauncherSearchResult,
    SchemeResultItem, VariantResultItem, WindowResultItem,
};
pub use searcher::{ActionCommand, AppSearcher, SearchMode};
pub use view::LauncherView;
pub use views::{
    AnimationPresetDef, AppBrowserModel, BrowserLayoutMode, KeybindItemDef, KeybindsViewModel,
    WallpaperItemDef, WindowSwitcherItemDef, WindowSwitcherViewModel,
};

