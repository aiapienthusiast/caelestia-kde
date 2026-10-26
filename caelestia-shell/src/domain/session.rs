//! Domain entities for session actions and controls.

#![allow(dead_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionActionKind {
    Lock,
    Suspend,
    Hibernate,
    Reboot,
    Shutdown,
    Logout,
}

#[derive(Debug, Clone)]
pub struct SessionAction {
    pub kind: SessionActionKind,
    pub title: &'static str,
    pub description: &'static str,
    pub icon_name: &'static str,
}

impl SessionAction {
    pub const ALL: [SessionAction; 4] = [
        SessionAction {
            kind: SessionActionKind::Logout,
            title: "Log out",
            description: "End current KDE session",
            icon_name: "logout",
        },
        SessionAction {
            kind: SessionActionKind::Shutdown,
            title: "Shut down",
            description: "Power off the computer",
            icon_name: "power_settings_new",
        },
        SessionAction {
            kind: SessionActionKind::Hibernate,
            title: "Hibernate",
            description: "Suspend to disk",
            icon_name: "bedtime",
        },
        SessionAction {
            kind: SessionActionKind::Reboot,
            title: "Restart",
            description: "Reboot the computer",
            icon_name: "cached",
        },
    ];
}
