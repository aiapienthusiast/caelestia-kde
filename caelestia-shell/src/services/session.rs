//! Session service executing desktop power / lock / logout commands via D-Bus and systemctl.

use crate::domain::session::SessionActionKind;
use std::process::Command;
use tracing::{error, info};

pub struct SessionService;

impl SessionService {
    pub fn execute(kind: SessionActionKind) {
        info!("Executing session action: {:?}", kind);
        let res = match kind {
            SessionActionKind::Lock => Command::new("qdbus6")
                .args(["org.freedesktop.ScreenSaver", "/ScreenSaver", "org.freedesktop.ScreenSaver.Lock"])
                .spawn(),
            SessionActionKind::Suspend => Command::new("systemctl")
                .arg("suspend")
                .spawn(),
            SessionActionKind::Hibernate => Command::new("systemctl")
                .arg("hibernate")
                .spawn(),
            SessionActionKind::Reboot => Command::new("systemctl")
                .arg("reboot")
                .spawn(),
            SessionActionKind::Shutdown => Command::new("systemctl")
                .arg("poweroff")
                .spawn(),
            SessionActionKind::Logout => Command::new("qdbus6")
                .args(["org.kde.Shutdown", "/Shutdown", "org.kde.Shutdown.logout"])
                .spawn(),
        };

        if let Err(e) = res {
            error!("Failed to execute session action {:?}: {}", kind, e);
        }
    }
}
