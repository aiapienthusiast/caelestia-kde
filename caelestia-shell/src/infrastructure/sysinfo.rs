//! System Information provider for Caelestia KDE Shell
//! Parity with `shell/utils/SysInfo.qml`.

use std::fs;

#[derive(Debug, Clone)]
pub struct SystemInfo {
    pub os_name: String,
    pub os_pretty_name: String,
    pub os_id: String,
    pub kernel: String,
    pub hostname: String,
    pub user: String,
    pub desktop_session: String,
    pub shell: String,
    pub board_vendor: String,
    pub board_name: String,
}

impl Default for SystemInfo {
    fn default() -> Self {
        Self::load()
    }
}

impl SystemInfo {
    pub fn load() -> Self {
        let mut os_name = "Linux".to_string();
        let mut os_pretty_name = "Linux".to_string();
        let mut os_id = "linux".to_string();

        if let Ok(content) = fs::read_to_string("/etc/os-release") {
            for line in content.lines() {
                if let Some(val) = line.strip_prefix("NAME=") {
                    os_name = val.trim_matches('"').to_string();
                } else if let Some(val) = line.strip_prefix("PRETTY_NAME=") {
                    os_pretty_name = val.trim_matches('"').to_string();
                } else if let Some(val) = line.strip_prefix("ID=") {
                    os_id = val.trim_matches('"').to_string();
                }
            }
        }

        let kernel = fs::read_to_string("/proc/sys/kernel/osrelease")
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|_| "unknown".to_string());

        let hostname = fs::read_to_string("/proc/sys/kernel/hostname")
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|_| "localhost".to_string());

        let user = std::env::var("USER").unwrap_or_else(|_| "user".to_string());
        let desktop_session = std::env::var("XDG_CURRENT_DESKTOP")
            .or_else(|_| std::env::var("XDG_SESSION_DESKTOP"))
            .unwrap_or_else(|_| "KDE".to_string());

        let shell = std::env::var("SHELL")
            .map(|s| s.rsplit('/').next().unwrap_or("bash").to_string())
            .unwrap_or_else(|_| "bash".to_string());

        let board_vendor = fs::read_to_string("/sys/class/dmi/id/sys_vendor")
            .map(|s| Self::sanitise_dmi(&s))
            .unwrap_or_default();

        let board_name = fs::read_to_string("/sys/class/dmi/id/product_name")
            .map(|s| Self::sanitise_dmi(&s))
            .unwrap_or_default();

        Self {
            os_name,
            os_pretty_name,
            os_id,
            kernel,
            hostname,
            user,
            desktop_session,
            shell,
            board_vendor,
            board_name,
        }
    }

    /// Read current uptime formatted as "Xh Ym"
    pub fn uptime_formatted() -> String {
        if let Ok(content) = fs::read_to_string("/proc/uptime") {
            if let Some(first) = content.split_whitespace().next() {
                if let Ok(secs) = first.parse::<f64>() {
                    let total_secs = secs as u64;
                    let hours = total_secs / 3600;
                    let mins = (total_secs % 3600) / 60;
                    if hours > 0 {
                        return format!("{}h {}m", hours, mins);
                    } else {
                        return format!("{}m", mins);
                    }
                }
            }
        }
        "0m".to_string()
    }

    /// Device display name combined from vendor and product name
    pub fn device_name(&self) -> String {
        if self.board_name.is_empty() {
            return self.board_vendor.clone();
        }
        if self.board_vendor.is_empty()
            || self
                .board_name
                .to_lowercase()
                .starts_with(&self.board_vendor.to_lowercase())
        {
            return self.board_name.clone();
        }
        format!("{} {}", self.board_vendor, self.board_name)
    }

    pub fn sanitise_dmi(s: &str) -> String {
        let t = s.trim();
        let junk = [
            "to be filled by o.e.m.",
            "system product name",
            "system manufacturer",
            "system version",
            "default string",
            "o.e.m.",
            "not specified",
            "not applicable",
            "unknown",
            "none",
            "",
        ];
        if junk.contains(&t.to_lowercase().as_str()) {
            String::new()
        } else {
            t.to_string()
        }
    }
}
