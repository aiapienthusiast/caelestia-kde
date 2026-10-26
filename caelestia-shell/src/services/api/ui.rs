//! UI State and Overlay API Bridge
//! Parity with `shell/services/api/UiApi.qml`.

use crate::services::clipboard::{ClipboardItem, ClipboardService};
use crate::services::emojidb::{EmojiDbService, EmojiEntry};
use crate::services::global_shortcut::{GlobalShortcutDispatcher, RegisteredShortcut};
use crate::services::toaster::{Toast, ToastType, ToasterService};
use std::sync::{Arc, Mutex, OnceLock};

/// Panel or launcher widget descriptor registered by a plugin
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WidgetDescriptor {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub category: String,
}

/// UI API exposing Toasts, Clipboard, Emojis, Shortcuts, and Plugin Widget models
#[derive(Clone)]
pub struct UiApi {
    pub toaster: Arc<ToasterService>,
    pub clipboard: Arc<ClipboardService>,
    pub emojidb: Arc<EmojiDbService>,
    pub shortcuts: Arc<Mutex<GlobalShortcutDispatcher>>,
    pub top_bar_left_widgets: Arc<Mutex<Vec<WidgetDescriptor>>>,
    pub top_bar_middle_widgets: Arc<Mutex<Vec<WidgetDescriptor>>>,
    pub top_bar_right_widgets: Arc<Mutex<Vec<WidgetDescriptor>>>,
    pub launcher_widgets: Arc<Mutex<Vec<WidgetDescriptor>>>,
}

impl Default for UiApi {
    fn default() -> Self {
        Self::new()
    }
}

impl UiApi {
    pub fn new() -> Self {
        Self {
            toaster: Arc::new(ToasterService::new()),
            clipboard: Arc::new(ClipboardService::default()),
            emojidb: Arc::new(EmojiDbService::default()),
            shortcuts: Arc::new(Mutex::new(GlobalShortcutDispatcher::default())),
            top_bar_left_widgets: Arc::new(Mutex::new(Vec::new())),
            top_bar_middle_widgets: Arc::new(Mutex::new(Vec::new())),
            top_bar_right_widgets: Arc::new(Mutex::new(Vec::new())),
            launcher_widgets: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn global() -> &'static Self {
        static INSTANCE: OnceLock<UiApi> = OnceLock::new();
        INSTANCE.get_or_init(Self::new)
    }

    /// Dispatch an ephemeral toast notification to the screen
    pub fn show_toast(&self, title: &str, message: &str, icon: &str, toast_type: ToastType, timeout_ms: u64) -> Toast {
        self.toaster.toast(title, message, icon, toast_type, timeout_ms)
    }

    /// Read active toasts
    pub fn active_toasts(&self) -> Vec<Toast> {
        self.toaster.active_toasts()
    }

    /// Read current text content from system clipboard history
    pub fn clipboard_items(&self) -> Vec<ClipboardItem> {
        self.clipboard.items()
    }

    /// Copy item back to system clipboard
    pub fn copy_clipboard_item(&self, item: &ClipboardItem) -> Result<(), std::io::Error> {
        self.clipboard.copy_item(item)
    }

    /// Fuzzy search emoji database by query
    pub fn search_emojis(&self, query: &str) -> Vec<EmojiEntry> {
        self.emojidb.search(query, 20, &[])
    }

    /// Registered global shortcuts
    pub fn shortcuts(&self) -> Vec<RegisteredShortcut> {
        self.shortcuts.lock().map_or(Vec::new(), |s| s.all_shortcuts())
    }

    /// Register a widget into top bar left zone
    pub fn register_top_bar_left(&self, widget: WidgetDescriptor) {
        if let Ok(mut w) = self.top_bar_left_widgets.lock() {
            w.push(widget);
        }
    }

    /// Register a widget into top bar middle zone
    pub fn register_top_bar_middle(&self, widget: WidgetDescriptor) {
        if let Ok(mut w) = self.top_bar_middle_widgets.lock() {
            w.push(widget);
        }
    }

    /// Register a widget into top bar right zone
    pub fn register_top_bar_right(&self, widget: WidgetDescriptor) {
        if let Ok(mut w) = self.top_bar_right_widgets.lock() {
            w.push(widget);
        }
    }

    /// Register a widget into the app launcher catalog
    pub fn register_launcher_widget(&self, widget: WidgetDescriptor) {
        if let Ok(mut w) = self.launcher_widgets.lock() {
            w.push(widget);
        }
    }
}
