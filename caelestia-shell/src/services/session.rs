//! Session service executing desktop power / lock / logout commands via D-Bus and systemctl.

use crate::domain::session::SessionActionKind;
use std::process::Command;
use tracing::{error, info};

pub struct SessionService;

impl SessionService {
    /// Pure command and arguments mapping for session actions (used for verification and dispatch)
    pub fn command_for_action(kind: SessionActionKind) -> (&'static str, &'static [&'static str]) {
        match kind {
            SessionActionKind::Lock => (
                "qdbus6",
                &[
                    "org.freedesktop.ScreenSaver",
                    "/ScreenSaver",
                    "org.freedesktop.ScreenSaver.Lock",
                ],
            ),
            SessionActionKind::Suspend => ("systemctl", &["suspend"]),
            SessionActionKind::Hibernate => ("systemctl", &["hibernate"]),
            SessionActionKind::Reboot => ("systemctl", &["reboot"]),
            SessionActionKind::Shutdown => ("systemctl", &["poweroff"]),
            SessionActionKind::Logout => (
                "qdbus6",
                &["org.kde.Shutdown", "/Shutdown", "org.kde.Shutdown.logout"],
            ),
        }
    }

    pub fn execute(kind: SessionActionKind) {
        info!("Executing session action: {:?}", kind);
        if crate::infrastructure::is_test_environment() {
            return;
        }
        let (prog, args) = Self::command_for_action(kind);
        let res = Command::new(prog).args(args).spawn();
        if let Err(e) = res {
            error!("Failed to execute session action {:?}: {}", kind, e);
        }
    }
}
