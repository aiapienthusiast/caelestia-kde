//! Desktop icons subsystem
//! Ported with 100% parity from `shell/modules/background/desktopicons/`.

pub mod big_folder;
pub mod drag_preview;
pub mod entry_icon;
pub mod file_entry;
pub mod layout_engine;
pub mod widget_catalog;

pub use big_folder::{BigFolderItem, DesktopGroupPopup};
pub use drag_preview::{DragPreviewState, DropAction, DropMenuModel};
pub use entry_icon::DesktopEntryIcon;
pub use file_entry::{DesktopFileEntry, FileKind};
pub use layout_engine::{
    DesktopItemMeta, GridPos, LayoutEngine, NavDirection, PixelRect, Span,
};
pub use widget_catalog::{DesktopWidgetCatalog, DesktopWidgetDef};
