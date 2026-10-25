use caelestia_shell::modules::notifications::*;
use caelestia_shell::services::notifications::{NotificationItem, NotificationsService, Urgency};
use std::time::{Duration, Instant};

#[test]
fn test_toast_lifecycle_and_urgency_timeouts() {
    let item_normal = NotificationItem {
        id: 1,
        app_name: "Discord".to_string(),
        app_icon: "discord".to_string(),
        summary: "Message".to_string(),
        body: "Hello world".to_string(),
        urgency: Urgency::Normal,
        timestamp_secs: 100,
    };
    let item_low = NotificationItem {
        id: 2,
        app_name: "Update".to_string(),
        app_icon: "system".to_string(),
        summary: "System".to_string(),
        body: "Low priority".to_string(),
        urgency: Urgency::Low,
        timestamp_secs: 101,
    };
    let item_crit = NotificationItem {
        id: 3,
        app_name: "Battery".to_string(),
        app_icon: "battery".to_string(),
        summary: "Low battery".to_string(),
        body: "Plug in charger".to_string(),
        urgency: Urgency::Critical,
        timestamp_secs: 102,
    };

    let toast_normal = ToastItem::new(item_normal, 5000, false);
    let toast_low = ToastItem::new(item_low, 5000, false);
    let toast_crit = ToastItem::new(item_crit, 5000, false);

    assert_eq!(toast_normal.timeout_ms, 5000);
    assert_eq!(toast_low.timeout_ms, 3000); // Low clamped to 3000 max
    assert_eq!(toast_crit.timeout_ms, 0); // Critical has 0 (infinite) timeout
    assert!(!toast_crit.is_expired());
}

#[test]
fn test_toast_hover_pause_and_resume() {
    let item = NotificationItem {
        id: 1,
        app_name: "Test".to_string(),
        app_icon: "test".to_string(),
        summary: "Hover test".to_string(),
        body: "Hover test body".to_string(),
        urgency: Urgency::Normal,
        timestamp_secs: 100,
    };

    let mut toast = ToastItem::new(item, 100, false);
    assert!(!toast.is_expired());

    // Hover stops timer accumulation
    toast.set_hovered(true);
    let now1 = Instant::now() + Duration::from_millis(150);
    toast.tick(now1);
    assert!(!toast.is_expired());
    assert_eq!(toast.elapsed_active, Duration::ZERO);

    // Unhover resumes timer accumulation
    toast.set_hovered(false);
    let now2 = Instant::now() + Duration::from_millis(150);
    toast.tick(now2);
    assert!(toast.is_expired());
}

#[test]
fn test_notification_manager_sync_and_dnd_suppression() {
    let mut service = NotificationsService::new();
    let config = NotificationConfig {
        max_popups: 2,
        default_timeout_ms: 4000,
        ..Default::default()
    };
    let mut manager = NotificationManager::new(config);

    // Post 3 notifications
    service.post("App1", "icon1", "Title1", "Body1", Urgency::Normal);
    service.post("App2", "icon2", "Title2", "Body2", Urgency::Normal);
    service.post("App3", "icon3", "Title3", "Body3", Urgency::Normal);

    manager.sync_with_service(&service);
    assert_eq!(manager.toasts.len(), 3);

    // max_popups limits visible to 2
    let visible = manager.visible_popups(false, false);
    assert_eq!(visible.len(), 2);

    // Test DND mode suppression
    service.set_dnd(true);
    service.post("App4", "icon4", "DND Title", "DND Body", Urgency::Normal);
    manager.sync_with_service(&service);

    // Newly posted notification under DND has popup = false
    let dnd_toast = manager.toasts.iter().find(|t| t.item.summary == "DND Title").unwrap();
    assert!(!dnd_toast.popup);
}

#[test]
fn test_notification_fullscreen_and_overview_suppression() {
    let mut service = NotificationsService::new();
    let config = NotificationConfig {
        fullscreen_policy: FullscreenPolicy::Off,
        ..Default::default()
    };
    let mut manager = NotificationManager::new(config);

    service.post("App", "icon", "Test", "Body", Urgency::Normal);
    manager.sync_with_service(&service);

    // Normal desktop: popups visible
    assert_eq!(manager.visible_popups(false, false).len(), 1);

    // Fullscreen window active: popups suppressed (FullscreenPolicy::Off)
    assert_eq!(manager.visible_popups(true, false).len(), 0);

    // Overview active: popups suppressed
    assert_eq!(manager.visible_popups(false, true).len(), 0);

    // If FullscreenPolicy is On, allowed during fullscreen
    manager.config.fullscreen_policy = FullscreenPolicy::On;
    assert_eq!(manager.visible_popups(true, false).len(), 1);
}

#[test]
fn test_notification_dismiss_and_clear_all() {
    let mut service = NotificationsService::new();
    let mut manager = NotificationManager::new(NotificationConfig::default());

    let id1 = service.post("App1", "icon", "T1", "B1", Urgency::Normal);
    let _id2 = service.post("App2", "icon", "T2", "B2", Urgency::Normal);
    manager.sync_with_service(&service);

    assert_eq!(manager.toasts.len(), 2);
    assert_eq!(service.count(), 2);

    // Dismiss id1
    manager.dismiss(id1, &mut service);
    assert_eq!(service.count(), 1);
    let t1 = manager.toasts.iter().find(|t| t.item.id == id1).unwrap();
    assert!(t1.closed);
    assert!(!t1.popup);

    // Clear all
    manager.clear_all(&mut service);
    assert_eq!(service.count(), 0);
    assert_eq!(manager.toasts.len(), 0);
}
