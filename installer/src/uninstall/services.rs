use super::*;

impl Uninstaller {
    pub(super) fn stop_services(&self) {
        section("Step 1 - Stop and Disable Services");

        for service in [
            "qs-kwin-bridge",
            "cliphist",
            "ydotoold",
            "kde-material-you-colors",
        ] {
            let unit = format!("{service}.service");
            if unit_is_enabled_or_active(&unit) {
                let _ =
                    process::succeeds("systemctl", &["--user", "disable", "--now", &unit], None);
                output::ok(&format!("Disabled user service: {service}"));
            } else {
                output::skip(&format!("User service not active: {service}"));
            }
        }

        if unit_is_enabled_or_active("caelestia-update-checker.timer") {
            let _ = process::succeeds(
                "systemctl",
                &[
                    "--user",
                    "disable",
                    "--now",
                    "caelestia-update-checker.timer",
                ],
                None,
            );
            let _ = process::succeeds(
                "systemctl",
                &[
                    "--user",
                    "disable",
                    "--now",
                    "caelestia-update-checker.service",
                ],
                None,
            );
            output::ok("Disabled user timer: caelestia-update-checker");
        }

        if system_unit_is_enabled_or_active("keyd") {
            let _ = privileges::elevated("systemctl", &["disable", "--now", "keyd"], None);
            output::ok("Disabled system service: keyd");
        } else {
            output::skip("keyd not active");
        }

        let _ = process::succeeds("pkill", &["-f", "caelestia shell"], None);
        let _ = process::succeeds("pkill", &["-f", "quickshell.*caelestia"], None);
        output::ok("Stopped any running shell processes");
    }
    pub(super) fn remove_service_files(&self) {
        section("Step 2 - Remove Service and Autostart Files");

        let systemd = self.path(".config/systemd/user");
        for service in [
            "qs-kwin-bridge.service",
            "cliphist.service",
            "ydotoold.service",
            "kde-material-you-colors.service",
            "caelestia-update-checker.service",
            "caelestia-update-checker.timer",
        ] {
            let path = systemd.join(service);
            if path.is_file() && fs::remove_file(&path).is_ok() {
                output::ok(&format!("Removed: {}", path.display()));
            }
        }

        let _ = process::succeeds(
            "systemctl",
            &["--user", "disable", "--now", "caelestia-shell.service"],
            None,
        );
        let shell_unit = systemd.join("caelestia-shell.service");
        if shell_unit.is_file() && fs::remove_file(&shell_unit).is_ok() {
            output::ok("Removed: caelestia-shell.service");
        }

        if let Ok(entries) = fs::read_dir(&systemd) {
            for wants in entries.flatten().filter(|entry| {
                entry.path().is_dir() && entry.file_name().to_string_lossy().ends_with(".wants")
            }) {
                let link = wants.path().join("caelestia-shell.service");
                if fs::symlink_metadata(&link).is_ok()
                    && !link.exists()
                    && fs::remove_file(&link).is_ok()
                {
                    output::ok(&format!(
                        "Removed an enable link whose unit is gone: {}",
                        link.display()
                    ));
                }
            }
        }

        let autostart = self.path(".config/autostart/caelestiashell.desktop");
        if autostart.is_file() {
            let _ = process::succeeds(
                "systemctl",
                &["--user", "disable", "app-caelestiashell@autostart.service"],
                None,
            );
            let _ = fs::remove_file(&autostart);
            output::ok("Removed the retired autostart entry: caelestiashell.desktop");
        }

        let _ = process::succeeds("systemctl", &["--user", "daemon-reload"], None);
    }
    pub(super) fn remove_shell_installation(&self) {
        section("Step 3 - Remove Shell Installation");

        // An uninstall is about the layout the installer put down, which is the source one unless
        // this very binary runs from a package: the packaged tree belongs to the package manager.
        for relative in [
            ".config/quickshell/caelestia",
            ".local/lib/caelestia",
            ".local/share/caelestia-shell",
            ".local/share/caelestia",
        ] {
            let path = self.path(relative);
            if path.is_dir() && remove_tree(&path) {
                output::ok(&format!("Removed {}", display_home(&path, &self.home)));
            }
        }

        for name in ["quickshell.desktop", "org.quickshell.desktop"] {
            let path = self.path(".local/share/applications").join(name);
            if path.is_file() && file_mentions(&path, "X-KDE-Wayland-Interfaces") {
                let _ = fs::remove_file(&path);
                output::ok(&format!("Removed ~/.local/share/applications/{name}"));
            }
        }

        for module in ["Caelestia", "M3Shapes"] {
            let path = self.path(".local/lib/qt6/qml").join(module);
            if path.is_dir() && remove_tree(&path) {
                output::ok(&format!("Removed QML module: {module}"));
            }
        }
    }
    pub(super) fn remove_shell_state(&self) {
        section("Step 3b - Remove Shell State");

        for name in ["stolen-shortcuts.json", "stolen-screen-edges.json"] {
            let path = self.path(".config/caelestia").join(name);
            if path.is_file() && fs::remove_file(&path).is_ok() {
                output::ok(&format!(
                    "Removed stranded recovery file: ~/.config/caelestia/{name}"
                ));
            }
        }
        let _ = fs::remove_file(self.path(".config/caelestia/status_icons_order.txt"));

        for relative in [".cache/Caelestia", ".cache/caelestia-shell"] {
            let path = self.path(relative);
            if path.is_dir() && remove_tree(&path) {
                output::ok(&format!("Removed {}", path.display()));
            }
        }

        for (relative, message) in [
            (".config/Caelestia", "Removed legacy ~/.config/Caelestia"),
            (
                ".local/share/plasma/shells/caelestia.desktop",
                "Removed ~/.local/share/plasma/shells/caelestia.desktop",
            ),
        ] {
            let path = self.path(relative);
            if path.is_dir() && remove_tree(&path) {
                output::ok(message);
            }
        }

        if process::available("kpackagetool6") {
            let _ = process::succeeds(
                "kpackagetool6",
                &[
                    "-t",
                    "Plasma/Wallpaper",
                    "-r",
                    "net.dosowisko.PlasmaApplicationWallpaper",
                ],
                None,
            );
        }

        let wallpaper = self
            .path(".local/share/plasma/wallpapers")
            .join("net.dosowisko.PlasmaApplicationWallpaper");
        if wallpaper.is_dir() && remove_tree(&wallpaper) {
            output::ok("Removed Plasma wallpaper plugin: net.dosowisko.PlasmaApplicationWallpaper");
        }

        let _ = fs::remove_file(
            paths::cache_dir()
                .join("caelestia-kde")
                .join("wallpaper-plugin-installed"),
        );
    }
    pub(super) fn remove_bridge_scripts(&self) {
        section("Step 4 - Remove Bridge Scripts");

        let bin = self.path(".local/bin");
        if let Ok(entries) = fs::read_dir(self.bundle.join("src").join("bin")) {
            for entry in entries.flatten() {
                let target = bin.join(entry.file_name());
                if target.is_file() && fs::remove_file(&target).is_ok() {
                    output::ok(&format!("Removed: {}", target.display()));
                }
            }
        }

        for name in [
            "kcolorpicker",
            "qs-kwin-bridge.py",
            "caelestia-shortcuts",
            "caelestia-keyd-run",
            "ydotoold-wrapper",
            "caelestia-autostart.sh",
            "caelestia-cliphist",
        ] {
            let target = bin.join(name);
            if target.is_file() && fs::remove_file(&target).is_ok() {
                output::ok(&format!("Removed: {}", target.display()));
            }
        }

        let script = self.path(".local/share/kwin/scripts/quickshell-kde-bridge");
        if script.is_dir() && remove_tree(&script) {
            output::ok("Removed KWin script: quickshell-kde-bridge");
        }
    }
}
