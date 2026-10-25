//! Notification Manager Subsystem
//! Complete parity with `shell/services/Notifs.qml` and `shell/modules/notifications/Wrapper.qml`.

use super::config::{FullscreenPolicy, NotificationConfig};
use super::toast::ToastItem;
use crate::services::notifications::NotificationsService;
use std::time::Instant;

pub struct NotificationManager {
    pub config: NotificationConfig,
    pub toasts: Vec<ToastItem>,
    pub dnd: bool,
}

impl NotificationManager {
    pub fn new(config: NotificationConfig) -> Self {
        Self {
            config,
            toasts: Vec::new(),
            dnd: false,
        }
    }

    /// Sync active toasts from backend NotificationsService
    pub fn sync_with_service(&mut self, service: &NotificationsService) {
        self.dnd = service.state.dnd;

        for item in &service.state.items {
            if !self.toasts.iter().any(|t| t.item.id == item.id) {
                // If DND is on, don't show popup toast
                let mut toast = ToastItem::new(
                    item.clone(),
                    self.config.default_timeout_ms,
                    self.config.open_expanded,
                );
                if self.dnd {
                    toast.popup = false;
                }
                self.toasts.insert(0, toast);
            }
        }

        // Keep toasts array bounded
        if self.toasts.len() > service.state.max_items {
            self.toasts.truncate(service.state.max_items);
        }
    }

    /// Advance timers and dismiss expired popups
    pub fn tick(&mut self) {
        let now = Instant::now();
        for toast in &mut self.toasts {
            toast.tick(now);
            if toast.is_expired() {
                toast.popup = false;
                toast.closed = true;
            }
        }
    }

    /// Dismiss a specific toast and sync back to backend service
    pub fn dismiss(&mut self, id: u64, service: &mut NotificationsService) {
        if let Some(toast) = self.toasts.iter_mut().find(|t| t.item.id == id) {
            toast.close();
        }
        service.dismiss(id);
    }

    /// Clear all popups and backend history
    pub fn clear_all(&mut self, service: &mut NotificationsService) {
        for toast in &mut self.toasts {
            toast.close();
        }
        self.toasts.clear();
        service.clear_all();
    }

    /// Get visible active popups matching Content.qml filtering
    pub fn visible_popups(&self, has_fullscreen: bool, overview_open: bool) -> Vec<&ToastItem> {
        if overview_open {
            return Vec::new();
        }

        if has_fullscreen && self.config.fullscreen_policy == FullscreenPolicy::Off {
            return Vec::new();
        }

        self.toasts
            .iter()
            .filter(|t| t.popup && !t.closed)
            .take(self.config.max_popups)
            .collect()
    }
}
