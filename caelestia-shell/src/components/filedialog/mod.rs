//! FileDialog component library
//! 100% parity with `shell/components/filedialog/`.

pub mod current_item;
pub mod dialog_buttons;
pub mod file_dialog;
pub mod folder_contents;
pub mod header_bar;
pub mod sidebar;
pub mod sizes;

pub use current_item::{CurrentItemView, FileEntry};
pub use dialog_buttons::DialogButtonsView;
pub use file_dialog::{FileDialog, FileDialogView};
pub use folder_contents::{scan_directory, FolderContentsView};
pub use header_bar::HeaderBarView;
pub use sidebar::{SidebarView, STANDARD_PLACES};
pub use sizes::*;
