//! Declarative CustomShortcut component
//! 100% parity with `shell/components/misc/CustomShortcut.qml`.
//!
//! Provides a reactive shortcut wrapper that registers with `GlobalShortcutDispatcher`
//! on KDE Plasma (or Hyprland global shortcuts when under Hyprland), handling
//! pressed/released signals, activation events, and collision lookup.

use crate::services::global_shortcut::{GlobalShortcutDispatcher, RegisteredShortcut};
use std::sync::Arc;

/// A declarative custom shortcut component matching `CustomShortcut.qml`
#[derive(Clone)]
pub struct CustomShortcut {
    pub name: String,
    pub description: String,
    pub key: Option<String>,
    pub enabled: bool,
    on_pressed: Option<Arc<dyn Fn() + Send + Sync>>,
    on_released: Option<Arc<dyn Fn() + Send + Sync>>,
}

impl std::fmt::Debug for CustomShortcut {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CustomShortcut")
            .field("name", &self.name)
            .field("description", &self.description)
            .field("key", &self.key)
            .field("enabled", &self.enabled)
            .field("has_on_pressed", &self.on_pressed.is_some())
            .field("has_on_released", &self.on_released.is_some())
            .finish()
    }
}

impl CustomShortcut {
    /// Creates a new `CustomShortcut` with given name and description.
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            key: None,
            enabled: true,
            on_pressed: None,
            on_released: None,
        }
    }

    /// Sets the keybinding sequence (e.g. "Meta+Space", "Super_L", "Meta+Shift+S").
    pub fn with_key(mut self, key: impl Into<String>) -> Self {
        self.key = Some(key.into());
        self
    }

    /// Sets whether the shortcut is actively listening.
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Attaches an `on_pressed` callback.
    pub fn on_pressed<F>(mut self, callback: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.on_pressed = Some(Arc::new(callback));
        self
    }

    /// Attaches an `on_released` callback.
    pub fn on_released<F>(mut self, callback: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.on_released = Some(Arc::new(callback));
        self
    }

    /// Invokes the `on_pressed` callback if the shortcut is enabled.
    pub fn press(&self) {
        if self.enabled {
            if let Some(cb) = &self.on_pressed {
                cb();
            }
        }
    }

    /// Invokes the `on_released` callback if the shortcut is enabled.
    pub fn release(&self) {
        if self.enabled {
            if let Some(cb) = &self.on_released {
                cb();
            }
        }
    }

    /// Simulates KDE `onActivated`: triggers `press()` followed by `release()`.
    pub fn activate(&self) {
        if self.enabled {
            self.press();
            self.release();
        }
    }

    /// Returns true if running under Hyprland compositor.
    pub fn is_hyprland() -> bool {
        std::env::var("HYPRLAND_INSTANCE_SIGNATURE").is_ok()
    }

    /// Returns the collision name if the configured key collides with an existing KDE or Caelestia binding.
    pub fn collision_name(&self) -> Option<String> {
        let key = self.key.as_deref()?;
        let dispatcher = GlobalShortcutDispatcher::global();
        let lock = dispatcher.lock().ok()?;
        lock.collision_for_key(key)
    }

    /// Registers this shortcut with the global shortcut dispatcher.
    pub fn register_global(&self) {
        let dispatcher = GlobalShortcutDispatcher::global();
        if let Ok(mut lock) = dispatcher.lock() {
            lock.register(RegisteredShortcut {
                name: self.name.clone(),
                description: self.description.clone(),
                key: self.key.clone().unwrap_or_default(),
                enabled: self.enabled,
                stolen_shortcuts: Vec::new(),
            });
        }
    }

    /// Unregisters this shortcut from the global shortcut dispatcher.
    pub fn unregister_global(&self) {
        let dispatcher = GlobalShortcutDispatcher::global();
        if let Ok(mut lock) = dispatcher.lock() {
            lock.unregister(&self.name);
        }
    }
}
