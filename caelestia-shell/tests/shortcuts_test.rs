use caelestia_shell::modules::drawers::ContentWindowCoordinator;
use caelestia_shell::shortcuts::{KrohnkiteAction, ShellShortcutAction, ShortcutManager};

#[test]
fn test_default_shortcuts_registration_and_lookup() {
    let manager = ShortcutManager::new();

    // Verify key mappings
    assert_eq!(manager.lookup("Meta+N"), Some(ShellShortcutAction::Nexus));
    assert_eq!(
        manager.lookup("Meta+D"),
        Some(ShellShortcutAction::Dashboard)
    );
    assert_eq!(
        manager.lookup("Meta+Tab"),
        Some(ShellShortcutAction::Overview)
    );
    assert_eq!(
        manager.lookup("Super_L"),
        Some(ShellShortcutAction::Launcher)
    );
    assert_eq!(manager.lookup("Meta+X"), Some(ShellShortcutAction::Session));
    assert_eq!(manager.lookup("Meta+B"), Some(ShellShortcutAction::Sidebar));
    assert_eq!(
        manager.lookup("Meta+U"),
        Some(ShellShortcutAction::Utilities)
    );
    assert_eq!(
        manager.lookup("Print"),
        Some(ShellShortcutAction::Screenshot)
    );
    assert_eq!(
        manager.lookup("Meta+Q"),
        Some(ShellShortcutAction::Krohnkite(KrohnkiteAction::CloseWindow))
    );
    assert_eq!(
        manager.lookup("Meta+Up"),
        Some(ShellShortcutAction::Krohnkite(KrohnkiteAction::FocusUp))
    );
}

#[test]
fn test_shortcut_dispatching_to_coordinator() {
    let mut manager = ShortcutManager::new();
    let mut coordinator = ContentWindowCoordinator::default();

    assert!(!coordinator.visibilities.dashboard);
    assert!(!coordinator.visibilities.overview);

    // Dispatch Dashboard toggle
    manager.dispatch(ShellShortcutAction::Dashboard, &mut coordinator);
    assert!(coordinator.visibilities.dashboard);

    // Dispatch Overview toggle
    manager.dispatch(ShellShortcutAction::Overview, &mut coordinator);
    assert!(coordinator.visibilities.overview);
    // Opening overview closes dashboard per drawer exclusive rules
    assert!(!coordinator.visibilities.dashboard);

    // Dispatch Overview toggle off
    manager.dispatch(ShellShortcutAction::Overview, &mut coordinator);
    assert!(!coordinator.visibilities.overview);

    // ShowAll action
    manager.dispatch(ShellShortcutAction::ShowAll, &mut coordinator);
    assert!(coordinator.visibilities.launcher);
    assert!(coordinator.visibilities.dashboard);
    assert!(coordinator.visibilities.utilities);
}

#[test]
fn test_custom_shortcut_component_lifecycle() {
    use caelestia_shell::components::misc::CustomShortcut;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    let pressed_count = Arc::new(AtomicUsize::new(0));
    let released_count = Arc::new(AtomicUsize::new(0));

    let p_clone = pressed_count.clone();
    let r_clone = released_count.clone();

    let shortcut = CustomShortcut::new("test_action", "Test Description")
        .with_key("Meta+Shift+T")
        .on_pressed(move || {
            p_clone.fetch_add(1, Ordering::SeqCst);
        })
        .on_released(move || {
            r_clone.fetch_add(1, Ordering::SeqCst);
        });

    assert_eq!(shortcut.name, "test_action");
    assert_eq!(shortcut.description, "Test Description");
    assert_eq!(shortcut.key, Some("Meta+Shift+T".to_string()));
    assert!(shortcut.enabled);

    // Test press and release
    shortcut.press();
    assert_eq!(pressed_count.load(Ordering::SeqCst), 1);
    assert_eq!(released_count.load(Ordering::SeqCst), 0);

    shortcut.release();
    assert_eq!(released_count.load(Ordering::SeqCst), 1);

    // Test activate (calls press then release)
    shortcut.activate();
    assert_eq!(pressed_count.load(Ordering::SeqCst), 2);
    assert_eq!(released_count.load(Ordering::SeqCst), 2);

    // Test disabled state
    let disabled = shortcut.with_enabled(false);
    disabled.activate();
    // Counts should remain unchanged
    assert_eq!(pressed_count.load(Ordering::SeqCst), 2);
    assert_eq!(released_count.load(Ordering::SeqCst), 2);
}

#[test]
fn test_global_shortcut_dispatcher_and_collisions() {
    use caelestia_shell::services::global_shortcut::{
        GlobalShortcutDispatcher, RegisteredShortcut, StolenShortcut,
    };

    let dispatcher = GlobalShortcutDispatcher::global();
    let mut lock = dispatcher.lock().unwrap();

    // Register a shortcut
    lock.register(RegisteredShortcut {
        name: "test_nexus".to_string(),
        description: "Open nexus".to_string(),
        key: "Meta+N".to_string(),
        enabled: true,
        stolen_shortcuts: vec![StolenShortcut {
            component: "kwin".to_string(),
            action: "Window Quick Tile Top".to_string(),
            component_friendly_name: "KWin".to_string(),
            action_friendly_name: "Quick Tile Top".to_string(),
            keys: vec!["Meta+N".to_string()],
            trigger_key: "Meta+N".to_string(),
        }],
    });

    // Verify collision detection
    let collision = lock.collision_for_key("Meta+N");
    assert!(collision.is_some());
    assert_eq!(collision.unwrap(), "KWin - Quick Tile Top");

    // Clean up
    lock.unregister("test_nexus");
    assert!(lock.collision_for_key("Meta+N").is_none());
}

#[test]
fn test_shortcut_manager_custom_shortcuts_integration() {
    let manager = ShortcutManager::new();
    let custom_list = manager.custom_shortcuts();
    assert!(!custom_list.is_empty());

    // Verify nexus custom shortcut is present
    let nexus_sc = custom_list.iter().find(|s| s.name == "nexus");
    assert!(nexus_sc.is_some());
    let nexus = nexus_sc.unwrap();
    assert_eq!(nexus.description, "Open nexus");
    assert_eq!(nexus.key, Some("Meta+N".to_string()));
}
