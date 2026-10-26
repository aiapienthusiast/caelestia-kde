//! Shell In-Memory Toast Notifications Service
//! 100% parity with C++ `Caelestia::Toaster` and `Toast`.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};

static NEXT_TOAST_ID: AtomicUsize = AtomicUsize::new(1);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToastType {
    Info = 0,
    Success,
    Warning,
    Error,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Toast {
    pub id: usize,
    pub title: String,
    pub message: String,
    pub icon: String,
    pub toast_type: ToastType,
    pub timeout_ms: u64,
    pub closed: bool,
    #[serde(skip)]
    pub created_at: Option<Instant>,
    pub lock_count: usize,
}

impl Toast {
    pub fn new(
        title: impl Into<String>,
        message: impl Into<String>,
        icon: impl Into<String>,
        toast_type: ToastType,
        timeout_ms: u64,
    ) -> Self {
        Self {
            id: NEXT_TOAST_ID.fetch_add(1, Ordering::SeqCst),
            title: title.into(),
            message: message.into(),
            icon: icon.into(),
            toast_type,
            timeout_ms,
            closed: false,
            created_at: Some(Instant::now()),
            lock_count: 0,
        }
    }

    pub fn lock(&mut self) {
        self.lock_count += 1;
    }

    pub fn unlock(&mut self) {
        if self.lock_count > 0 {
            self.lock_count -= 1;
        }
    }

    pub fn is_locked(&self) -> bool {
        self.lock_count > 0
    }

    pub fn close(&mut self) {
        self.closed = true;
    }

    pub fn is_expired(&self) -> bool {
        if self.closed || self.is_locked() {
            return false;
        }
        if let Some(created) = self.created_at {
            created.elapsed() >= Duration::from_millis(self.timeout_ms)
        } else {
            false
        }
    }
}

pub struct ToasterService {
    toasts: Mutex<Vec<Toast>>,
}

impl Default for ToasterService {
    fn default() -> Self {
        Self::new()
    }
}

impl ToasterService {
    pub fn new() -> Self {
        Self {
            toasts: Mutex::new(Vec::new()),
        }
    }

    /// Post a new toast notification
    pub fn toast(
        &self,
        title: impl Into<String>,
        message: impl Into<String>,
        icon: impl Into<String>,
        toast_type: ToastType,
        timeout_ms: u64,
    ) -> Toast {
        let item = Toast::new(title, message, icon, toast_type, timeout_ms);
        let cloned = item.clone();
        if let Ok(mut list) = self.toasts.lock() {
            list.push(item);
        }
        cloned
    }

    /// Retrieve active (unclosed) toasts
    pub fn active_toasts(&self) -> Vec<Toast> {
        if let Ok(list) = self.toasts.lock() {
            list.iter().filter(|t| !t.closed).cloned().collect()
        } else {
            Vec::new()
        }
    }

    /// Remove a toast by ID
    pub fn remove(&self, id: usize) {
        if let Ok(mut list) = self.toasts.lock() {
            list.retain(|t| t.id != id);
        }
    }

    /// Close a toast by ID
    pub fn close(&self, id: usize) {
        if let Ok(mut list) = self.toasts.lock() {
            if let Some(t) = list.iter_mut().find(|t| t.id == id) {
                t.close();
            }
        }
    }

    /// Purge expired toasts
    pub fn cleanup_expired(&self) {
        if let Ok(mut list) = self.toasts.lock() {
            list.retain(|t| !t.is_expired() && !t.closed);
        }
    }

    /// Clear all toasts
    pub fn clear(&self) {
        if let Ok(mut list) = self.toasts.lock() {
            list.clear();
        }
    }
}
