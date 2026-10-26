//! Background and desktop surface subsystem
//! Ported with 100% parity from `shell/modules/background/`.

pub mod clock;
pub mod config;
pub mod context_menu;
pub mod desktopicons;
pub mod state;
pub mod view;
pub mod wallpaper;
pub mod widgets;

pub use clock::{DesktopClockFormatted, DesktopClockModel};
pub use config::{BackgroundConfig, DesktopClockLayout, WallpaperFitMode};
pub use context_menu::{DesktopContextMenuModel, DesktopMenuAction};
pub use desktopicons::{
    BigFolderItem, DesktopEntryIcon, DesktopFileEntry, DesktopGroupPopup, DesktopItemMeta,
    DesktopWidgetCatalog, DesktopWidgetDef, DragPreviewState, DropAction, DropMenuModel, FileKind,
    GridPos, LayoutEngine, NavDirection, PixelRect, Span,
};
pub use state::BackgroundState;
pub use view::BackgroundView;
pub use wallpaper::{WallpaperModel, IMAGE_EXTENSIONS, VIDEO_EXTENSIONS};
pub use widgets::{
    CalendarEventItem, ClockStyle, DesktopCalendarWidgetModel, DesktopClockWidgetModel,
    DesktopLyricLine, DesktopLyricsWidgetModel, DesktopMediaWidgetModel, DesktopNoteWidgetModel,
    DesktopShapesWidgetModel, DesktopSystemWidgetModel, ShapeBlobDef,
};

