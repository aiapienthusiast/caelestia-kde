//! Test Suite for Session Subsystem
//! Validates SessionConfig, SessionState keyboard navigation (Tab, Arrow keys, Vim bindings),
//! drawer opening/closing, and action dispatching.

use caelestia_shell::domain::session::SessionActionKind;
use caelestia_shell::modules::session::{SessionConfig, SessionState};
use caelestia_shell::services::session::SessionService;

#[test]
fn test_session_config_and_actions_parity() {
    let config = SessionConfig::default();
    assert!(config.enabled);
    assert!(config.vim_keybinds);
    assert_eq!(config.icons.logout, "system-log-out");
    assert_eq!(config.icons.shutdown, "system-shutdown");
    assert_eq!(config.icons.hibernate, "system-suspend-hibernate");
    assert_eq!(config.icons.reboot, "system-reboot");

    let state = SessionState::new(&config);
    assert_eq!(state.actions.len(), 4);
    assert_eq!(state.actions[0].kind, SessionActionKind::Logout);
    assert_eq!(state.actions[1].kind, SessionActionKind::Shutdown);
    assert_eq!(state.actions[2].kind, SessionActionKind::Hibernate);
    assert_eq!(state.actions[3].kind, SessionActionKind::Reboot);
}

#[test]
fn test_session_state_focus_and_navigation() {
    let config = SessionConfig::default();
    let mut state = SessionState::new(&config);

    assert!(!state.is_open);
    state.open();
    assert!(state.is_open);
    assert_eq!(state.focused_index, 0);
    assert_eq!(state.focused_action(), Some(SessionActionKind::Logout));

    // Down arrow -> Shutdown
    let action = state.handle_key("down", false, false);
    assert_eq!(action, None);
    assert_eq!(state.focused_index, 1);
    assert_eq!(state.focused_action(), Some(SessionActionKind::Shutdown));

    // Tab -> Hibernate
    let action = state.handle_key("tab", false, false);
    assert_eq!(action, None);
    assert_eq!(state.focused_index, 2);
    assert_eq!(state.focused_action(), Some(SessionActionKind::Hibernate));

    // Vim Ctrl+J -> Reboot
    let action = state.handle_key("j", true, false);
    assert_eq!(action, None);
    assert_eq!(state.focused_index, 3);
    assert_eq!(state.focused_action(), Some(SessionActionKind::Reboot));

    // Down wrap-around -> Logout
    let action = state.handle_key("down", false, false);
    assert_eq!(action, None);
    assert_eq!(state.focused_index, 0);

    // Vim Ctrl+K -> Reboot (wrap backwards)
    let action = state.handle_key("k", true, false);
    assert_eq!(action, None);
    assert_eq!(state.focused_index, 3);

    // Shift+Tab -> Hibernate
    let action = state.handle_key("tab", false, true);
    assert_eq!(action, None);
    assert_eq!(state.focused_index, 2);

    // Enter executes focused action
    let action = state.handle_key("enter", false, false);
    assert_eq!(action, Some(SessionActionKind::Hibernate));

    // Escape closes drawer
    let action = state.handle_key("escape", false, false);
    assert_eq!(action, None);
    assert!(!state.is_open);
}

#[test]
fn test_session_service_command_mapping() {
    assert_eq!(
        SessionService::command_for_action(SessionActionKind::Shutdown),
        ("systemctl", &["poweroff"][..])
    );
    assert_eq!(
        SessionService::command_for_action(SessionActionKind::Reboot),
        ("systemctl", &["reboot"][..])
    );
    assert_eq!(
        SessionService::command_for_action(SessionActionKind::Hibernate),
        ("systemctl", &["hibernate"][..])
    );
    assert_eq!(
        SessionService::command_for_action(SessionActionKind::Suspend),
        ("systemctl", &["suspend"][..])
    );
    assert_eq!(
        SessionService::command_for_action(SessionActionKind::Logout),
        (
            "qdbus6",
            &["org.kde.Shutdown", "/Shutdown", "org.kde.Shutdown.logout"][..]
        )
    );
    assert_eq!(
        SessionService::command_for_action(SessionActionKind::Lock),
        (
            "qdbus6",
            &[
                "org.freedesktop.ScreenSaver",
                "/ScreenSaver",
                "org.freedesktop.ScreenSaver.Lock"
            ][..]
        )
    );
}
