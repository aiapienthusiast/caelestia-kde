//! Application Launcher Subsystem
//! Complete parity with `shell/modules/launcher/`.
//!
//! Provides desktop entry scanning, XDG category curation, fuzzy and prefix search,
//! process spawning with Freedesktop field code stripping, and the drawer view.

pub mod categories;
pub mod entry;
pub mod executor;
pub mod searcher;
pub mod view;

pub use categories::{Categories, CategoryDef, CATEGORY_DEFINITIONS};
pub use entry::DesktopEntry;
pub use executor::AppExecutor;
pub use searcher::{ActionCommand, AppSearcher, SearchMode};
pub use view::LauncherView;
