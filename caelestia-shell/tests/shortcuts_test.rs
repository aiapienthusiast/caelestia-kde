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
