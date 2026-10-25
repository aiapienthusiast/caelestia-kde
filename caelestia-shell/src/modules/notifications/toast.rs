//! Toast Item Model
//! Parity with `shell/modules/notifications/Notification.qml` and `NotifData.qml`.

use crate::services::notifications::{NotificationItem, Urgency};
use std::time::{Duration, Instant};

#[derive(Clone, Debug, PartialEq)]
pub struct ToastItem {
    pub item: NotificationItem,
    pub popup: bool,
    pub closed: bool,
    pub expanded: bool,
    pub hovered: bool,
    pub created_at: Instant,
    pub timeout_ms: u64,
    pub elapsed_active: Duration,
    pub last_tick: Instant,
    pub swipe_offset_x: f32,
}

impl ToastItem {
    pub fn new(item: NotificationItem, default_timeout_ms: u64, open_expanded: bool) -> Self {
        let timeout_ms = match item.urgency {
            Urgency::Critical => 0, // Critical notifications stay until explicitly dismissed
            Urgency::Low => default_timeout_ms.min(3000),
            Urgency::Normal => default_timeout_ms,
        };

        let now = Instant::now();
        Self {
            item,
            popup: true,
            closed: false,
            expanded: open_expanded,
            hovered: false,
            created_at: now,
            timeout_ms,
            elapsed_active: Duration::ZERO,
            last_tick: now,
            swipe_offset_x: 0.0,
        }
    }

    /// Advance active timer if not hovered
    pub fn tick(&mut self, now: Instant) {
        if !self.hovered && self.popup && !self.closed {
            let delta = now.saturating_duration_since(self.last_tick);
            self.elapsed_active += delta;
        }
        self.last_tick = now;
    }

    /// Check if auto-dismiss timeout has expired
    pub fn is_expired(&self) -> bool {
        if self.timeout_ms == 0 || self.hovered || !self.popup || self.closed {
            false
        } else {
            self.elapsed_active.as_millis() >= self.timeout_ms as u128
        }
    }

    /// Set hover state (stops timer when hovered, resumes when unhovered)
    pub fn set_hovered(&mut self, hovered: bool) {
        self.hovered = hovered;
        self.last_tick = Instant::now();
    }

    /// Toggle expanded body text
    pub fn toggle_expanded(&mut self) {
        self.expanded = !self.expanded;
    }

    /// Close popup
    pub fn close(&mut self) {
        self.popup = false;
        self.closed = true;
    }
}
