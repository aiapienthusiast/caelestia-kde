//! Launcher views module
//! Ported with 100% parity from `shell/modules/launcher/`.

pub mod browser_compact;
pub mod keybinds;

pub use browser_compact::{AppBrowserModel, BrowserLayoutMode};
pub use keybinds::{
    AnimationPresetDef, KeybindItemDef, KeybindsViewModel, WallpaperItemDef,
    WindowSwitcherItemDef, WindowSwitcherViewModel,
};
