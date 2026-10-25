//! Background and desktop surface subsystem
//! Ported with 100% parity from `shell/modules/background/`.

pub mod clock;
pub mod config;
pub mod context_menu;
pub mod state;
pub mod view;
pub mod wallpaper;

pub use clock::{DesktopClockFormatted, DesktopClockModel};
pub use config::{BackgroundConfig, DesktopClockLayout, WallpaperFitMode};
pub use context_menu::{DesktopMenuAction, DesktopContextMenuModel};
pub use state::BackgroundState;
pub use view::BackgroundView;
pub use wallpaper::{WallpaperModel, IMAGE_EXTENSIONS, VIDEO_EXTENSIONS};
