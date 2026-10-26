//! Dynamic Shortcuts Registration API Bridge
//! Parity with `shell/services/api/ShortcutsApi.qml`.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

pub type ShortcutCallback = Arc<dyn Fn() + Send + Sync + 'static>;

/// Dynamic Shortcut Handle returned to callers upon registration
#[derive(Clone)]
pub struct DynamicShortcut {
    pub name: String,
    pub description: String,
    pub key: String,
    callback: Option<ShortcutCallback>,
}

impl DynamicShortcut {
    pub fn trigger(&self) {
        if let Some(cb) = &self.callback {
            cb();
        }
    }
}

/// Dynamic shortcut registration manager
#[derive(Clone, Default)]
pub struct ShortcutsApi {
    registered: Arc<Mutex<HashMap<String, DynamicShortcut>>>,
}

impl ShortcutsApi {
    pub fn new() -> Self {
        Self {
            registered: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn global() -> &'static Self {
        static INSTANCE: OnceLock<ShortcutsApi> = OnceLock::new();
        INSTANCE.get_or_init(Self::new)
    }

    /// Dynamically register a shortcut with name, description, key combination, and optional callback
    pub fn register<F>(&self, name: &str, description: &str, key: &str, callback: Option<F>) -> DynamicShortcut
    where
        F: Fn() + Send + Sync + 'static,
    {
        let cb: Option<ShortcutCallback> = callback.map(|f| Arc::new(f) as ShortcutCallback);
        let shortcut = DynamicShortcut {
            name: name.to_string(),
            description: description.to_string(),
            key: key.to_string(),
            callback: cb,
        };

        if let Ok(mut map) = self.registered.lock() {
            map.insert(name.to_string(), shortcut.clone());
        }

        shortcut
    }

    /// Unregister a dynamically registered shortcut by name
    pub fn unregister(&self, name: &str) -> bool {
        self.registered.lock().map_or(false, |mut m| m.remove(name).is_some())
    }

    /// Retrieve all registered dynamic shortcuts
    pub fn all(&self) -> Vec<DynamicShortcut> {
        self.registered.lock().map_or(Vec::new(), |m| m.values().cloned().collect())
    }

    /// Dispatch shortcut trigger by name
    pub fn dispatch(&self, name: &str) -> bool {
        if let Ok(map) = self.registered.lock() {
            if let Some(sc) = map.get(name) {
                sc.trigger();
                return true;
            }
        }
        false
    }
}
