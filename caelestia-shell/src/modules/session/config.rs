//! Session Subsystem Configuration
//! 100% parity with `shell/modules/session/` and `Config.session`.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SessionIcons {
    pub logout: String,
    pub shutdown: String,
    pub hibernate: String,
    pub reboot: String,
}

impl Default for SessionIcons {
    fn default() -> Self {
        Self {
            logout: "logout".to_string(),
            shutdown: "power_settings_new".to_string(),
            hibernate: "bedtime".to_string(),
            reboot: "cached".to_string(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SessionCommands {
    pub logout: Vec<String>,
    pub shutdown: Vec<String>,
    pub hibernate: Vec<String>,
    pub reboot: Vec<String>,
}

impl Default for SessionCommands {
    fn default() -> Self {
        Self {
            logout: vec![
                "qdbus6".to_string(),
                "org.kde.Shutdown".to_string(),
                "/Shutdown".to_string(),
                "org.kde.Shutdown.logout".to_string(),
            ],
            shutdown: vec!["systemctl".to_string(), "poweroff".to_string()],
            hibernate: vec!["systemctl".to_string(), "hibernate".to_string()],
            reboot: vec!["systemctl".to_string(), "reboot".to_string()],
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SessionConfig {
    pub enabled: bool,
    pub vim_keybinds: bool,
    pub session_gif: String,
    pub session_gif_speed: f32,
    pub icons: SessionIcons,
    pub commands: SessionCommands,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            vim_keybinds: true,
            session_gif: "root:/assets/dino.gif".to_string(),
            session_gif_speed: 1.0,
            icons: SessionIcons::default(),
            commands: SessionCommands::default(),
        }
    }
}
