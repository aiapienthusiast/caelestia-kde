//! Desktop Notifications Module
//! Complete parity with `shell/modules/notifications/` and `shell/services/Notifs.qml`.
//!
//! Provides toast cards, floating popup stack, urgency styling, auto-dismiss timeouts,
//! hover pause, and fullscreen suppression policies.

pub mod card;
pub mod config;
pub mod manager;
pub mod toast;
pub mod view;

pub use card::NotificationCard;
pub use config::{FullscreenPolicy, NotificationConfig};
pub use manager::NotificationManager;
pub use toast::ToastItem;
pub use view::NotificationsView;
