//! Notifications Service
//! Parity with `shell/services/Notifs.qml` and `NotifData.qml`.
//! Manages desktop notification history, Do Not Disturb (DND) modes, and notification actions.

use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::info;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Urgency {
    Low,
    Normal,
    Critical,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationItem {
    pub id: u64,
    pub app_name: String,
    pub app_icon: String,
    pub summary: String,
    pub body: String,
    pub urgency: Urgency,
    pub timestamp_secs: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationsState {
    pub dnd: bool,
    pub items: Vec<NotificationItem>,
    pub max_items: usize,
}

impl Default for NotificationsState {
    fn default() -> Self {
        Self {
            dnd: false,
            items: Vec::new(),
            max_items: 50,
        }
    }
}

pub struct NotificationsService {
    pub state: NotificationsState,
    next_id: u64,
}

impl NotificationsService {
    pub fn new() -> Self {
        Self {
            state: NotificationsState::default(),
            next_id: 1,
        }
    }

    /// Toggle Do Not Disturb (DND)
    pub fn toggle_dnd(&mut self) {
        self.state.dnd = !self.state.dnd;
        info!("Toggled DND mode: {}", self.state.dnd);
    }

    /// Set DND explicitly
    pub fn set_dnd(&mut self, dnd: bool) {
        self.state.dnd = dnd;
    }

    /// Post a new notification to the service
    pub fn post(
        &mut self,
        app_name: impl Into<String>,
        app_icon: impl Into<String>,
        summary: impl Into<String>,
        body: impl Into<String>,
        urgency: Urgency,
    ) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        let timestamp_secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        let item = NotificationItem {
            id,
            app_name: app_name.into(),
            app_icon: app_icon.into(),
            summary: summary.into(),
            body: body.into(),
            urgency,
            timestamp_secs,
        };

        self.state.items.insert(0, item);

        // Enforce max notifications limit
        if self.state.items.len() > self.state.max_items {
            self.state.items.truncate(self.state.max_items);
        }

        id
    }

    /// Dismiss a notification by ID
    pub fn dismiss(&mut self, id: u64) {
        self.state.items.retain(|item| item.id != id);
    }

    /// Clear all notifications
    pub fn clear_all(&mut self) {
        self.state.items.clear();
    }

    /// Number of active notifications
    pub fn count(&self) -> usize {
        self.state.items.len()
    }
}
