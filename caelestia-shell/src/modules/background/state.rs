//! Background surface state coordinator
//! Unifies wallpaper display, desktop clock, icons toggle, and context menu

use super::clock::DesktopClockModel;
use super::config::BackgroundConfig;
use super::context_menu::{DesktopContextMenuModel, DesktopMenuAction};
use super::wallpaper::WallpaperModel;

#[derive(Clone, Debug, Default)]
pub struct BackgroundState {
    pub config: BackgroundConfig,
    pub wallpaper: WallpaperModel,
    pub clock: DesktopClockModel,
    pub context_menu: DesktopContextMenuModel,
    pub icons_visible: bool,
}

impl BackgroundState {
    pub fn new(config: BackgroundConfig) -> Self {
        let wallpaper = WallpaperModel::new(config.wallpaper_path.clone());
        let clock = DesktopClockModel::new(config.desktop_clock_layout);
        let icons_visible = config.desktop_icons_enabled;

        Self {
            config,
            wallpaper,
            clock,
            context_menu: DesktopContextMenuModel::default(),
            icons_visible,
        }
    }

    /// Toggles desktop icons visibility (middle-click or context menu)
    pub fn toggle_icons(&mut self) {
        self.icons_visible = !self.icons_visible;
    }

    /// Handles right click on empty desktop area
    pub fn on_empty_desktop_right_click(&mut self, x: f32, y: f32, screen_name: impl Into<String>) {
        if !self.context_menu.edit_mode {
            self.context_menu.open_at(x, y, screen_name);
        }
    }

    /// Handles middle click on empty desktop area
    pub fn on_empty_desktop_middle_click(&mut self) {
        if !self.context_menu.edit_mode {
            self.toggle_icons();
        }
    }

    /// Dispatches an action chosen from the context menu
    pub fn execute_context_menu_action(&mut self, action: DesktopMenuAction) {
        match action {
            DesktopMenuAction::NextWallpaper => {
                self.wallpaper.next_wallpaper(self.config.slideshow_random);
                self.context_menu.close();
            }
            DesktopMenuAction::ToggleDesktopIcons => {
                self.toggle_icons();
                self.context_menu.close();
            }
            other => {
                self.context_menu.execute(other);
            }
        }
    }
}
