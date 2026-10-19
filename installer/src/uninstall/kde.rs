use super::*;

impl Uninstaller {
    pub(super) fn restore_or_remove_configs(&self) {
        section("Step 5 - Restore or Remove Config Directories");

        for config in [
            "btop",
            "fastfetch",
            "fish",
            "foot",
            "kitty",
            "micro",
            "thunar",
        ] {
            let path = self.path(".config").join(config);
            if path.exists() {
                self.restore_or_remove(config, &path, ".config");
            }
        }

        let starship = self.path(".config/starship.toml");
        if starship.is_file() {
            self.restore_or_remove("starship.toml", &starship, ".config");
        }
    }
    pub(super) fn revert_kde_settings(&mut self) {
        section("Step 6 - Revert KDE Settings");

        for (file, groups, key) in [
            ("plasmarc", &["OSD"][..], "Enabled"),
            ("plasmarc", &["OSD"][..], "ShowOnActiveScreen"),
        ] {
            kwrite(file, groups, key, "true");
        }
        kwrite("kdeglobals", &["KDE"], "OSDEnabled", "true");
        kwrite(
            "plasmanotifyrc",
            &["Notifications"],
            "LoudnessChangedOSD",
            "true",
        );
        kwrite("powerdevilrc", &["BrightnessControl"], "showOSD", "true");
        kwrite("powerdevilrc", &["AC"], "brightnessosd", "true");
        kwrite("kmixrc", &["Global"], "ShowOSD", "true");
        output::ok("Re-enabled KDE OSD notifications");

        if let Some(archive) = self.konsave_archive.clone() {
            if self.ensure_konsave() {
                output::info("Restoring KDE settings from konsave archive...");
                let bin = self.konsave.clone().unwrap_or_default();
                let bin = bin.to_string_lossy().to_string();
                let archive = archive.to_string_lossy().to_string();
                if process::run_visible(&bin, &["-i", &archive], None) == Some(0)
                    && process::run_visible(&bin, &["-a", "caelestia-preinstall"], None) == Some(0)
                {
                    self.theme_restored = true;
                    output::ok("Restored KDE settings from konsave backup.");
                } else {
                    output::warn("konsave restore failed, falling back to manual restore paths.");
                    self.konsave_archive = None;
                }
            } else {
                output::warn("konsave is unavailable, falling back to manual restore paths.");
                self.konsave_archive = None;
            }
        }

        if self.konsave_archive.is_none() {
            self.restore_core_kde_configs();
        }

        if !self.has_a_panel()
            && process::succeeds("pgrep", &["-x", "plasmashell"], None)
            && let Some(qdbus) = process::locate("qdbus6").or_else(|| process::locate("qdbus"))
        {
            let qdbus = qdbus.to_string_lossy().to_string();
            if process::succeeds(
                &qdbus,
                &[
                    "org.kde.plasmashell",
                    "/PlasmaShell",
                    "evaluateScript",
                    STOCK_PANEL,
                ],
                None,
            ) {
                output::ok("Created a default bottom panel (no backup was available)");
            } else {
                output::warn(
                    "Could not create a default panel. Add one via Edit Mode if the desktop is empty.",
                );
            }
        }

        for (key, value) in [
            ("quickshell-kde-bridgeEnabled", "false"),
            ("krohnkiteEnabled", "false"),
            ("kwin_workspace_trackerEnabled", "false"),
        ] {
            kwrite("kwinrc", &["Plugins"], key, value);
        }
        output::ok(
            "Disabled KWin plugins: quickshell-kde-bridge, krohnkite, kwin_workspace_tracker",
        );

        for (group, key) in [
            ("caelestia-opacity", "Description"),
            ("caelestia-opacity", "types"),
            ("caelestia-opacity", "opacityinactive"),
            ("caelestia-opacity", "opacityinactiverule"),
            ("caelestia-dialogs", "Description"),
            ("caelestia-dialogs", "types"),
            ("caelestia-dialogs", "placement"),
            ("caelestia-dialogs", "placementrule"),
            ("caelestia-pip", "Description"),
            ("caelestia-pip", "title"),
            ("caelestia-pip", "titlematch"),
            ("caelestia-pip", "aboverule"),
        ] {
            kwrite_delete("kwinrulesrc", &[group], key);
        }
        self.prune_window_rules();

        kwrite_delete("plasmashellrc", &["Shell"], "ShellPackage");
        kwrite(
            "kscreenlockerrc",
            &["Greeter"],
            "Theme",
            "org.kde.breeze.desktop",
        );
        kwrite(
            "kscreenlockerrc",
            &["Greeter"],
            "WallpaperPlugin",
            "org.kde.image",
        );
        for (groups, key) in [
            (
                [
                    "Greeter",
                    "Wallpaper",
                    "net.dosowisko.PlasmaApplicationWallpaper",
                    "General",
                ],
                "command",
            ),
            (
                [
                    "Greeter",
                    "Wallpaper",
                    "net.dosowisko.PlasmaApplicationWallpaper",
                    "General",
                ],
                "fps",
            ),
        ] {
            kwrite_delete("kscreenlockerrc", &groups, key);
        }
        kwrite_delete(
            "kscreenlockerrc",
            &["Greeter", "LnF", "General"],
            "alwaysShowClock",
        );
        kwrite_delete(
            "kscreenlockerrc",
            &["Greeter", "LnF", "General"],
            "showMediaControls",
        );
        output::ok("Restored stock KDE lock screen configuration.");

        if !self.theme_restored {
            kwrite("kwinrc", &["Desktops"], "Number", "1");
            kwrite("kwinrc", &["Desktops"], "Rows", "1");
            for index in 1..=5 {
                kwrite(
                    "kwinrc",
                    &["Desktops"],
                    &format!("Name_{index}"),
                    &format!("Desktop {index}"),
                );
            }
            output::ok("Restored desktop count to 1");
        }

        for index in 1..=5 {
            kwrite(
                "kglobalshortcutsrc",
                &["kwin"],
                &format!("Switch to Desktop {index}"),
                &format!("none,none,Switch to Desktop {index}"),
            );
            kwrite(
                "kglobalshortcutsrc",
                &["kwin"],
                &format!("Window to Desktop {index}"),
                &format!("none,none,Move Window to Desktop {index}"),
            );
        }
        output::ok("Cleared installer workspace shortcuts from kglobalshortcutsrc");

        if let Some(backup) = &self.backup {
            let shortcuts = backup.join(".config/kglobalshortcutsrc");
            if shortcuts.is_file()
                && copy_file(&shortcuts, &self.path(".config/kglobalshortcutsrc"))
            {
                output::ok("Restored kglobalshortcutsrc from backup");
            }
        }

        for scheme in [
            "MaterialYou.colorscheme",
            "MaterialYouAlt.colorscheme",
            "TempMyou.profile",
        ] {
            let _ = fs::remove_file(self.path(".local/share/konsole").join(scheme));
        }
        for (directory, prefixes) in [(".local/share/color-schemes", ["MaterialYou", "Matugen"])] {
            for prefix in prefixes {
                remove_by_prefix(&self.path(directory), prefix);
            }
        }
        output::ok("Removed Konsole profiles generated by Caelestia");

        self.revert_darkly_gtk();
    }
    pub(super) fn has_a_panel(&self) -> bool {
        let path = self.path(".config/plasma-org.kde.plasma.desktop-appletsrc");
        let Ok(text) = fs::read_to_string(path) else {
            return false;
        };
        panel_in(&text)
    }
    pub(super) fn restore_core_kde_configs(&mut self) {
        let Some(backup) = self.backup.clone() else {
            self.reset_theme_to_breeze();
            return;
        };

        output::info("Restoring core KDE configuration files from backup...");
        let mut restored = 0;
        for config in [
            "kdeglobals",
            "ksplashrc",
            "plasmarc",
            "kwinrc",
            "kwinrulesrc",
            "kcminputrc",
            "plasma-org.kde.plasma.desktop-appletsrc",
        ] {
            let source = backup.join(".config").join(config);
            if source.is_file() && copy_file(&source, &self.path(".config").join(config)) {
                restored += 1;
            }
        }

        if restored > 0 {
            self.theme_restored = true;
            output::ok(&format!(
                "Restored {restored} core KDE configuration file(s) from backup (including wallpaper and splash when present)."
            ));
        } else {
            output::warn(
                "Selected backup did not contain expected core KDE config files. Falling back to Breeze defaults.",
            );
            self.reset_theme_to_breeze();
        }
    }
    pub(super) fn reset_theme_to_breeze(&self) {
        output::info("No theme backup found. Reverting to default Breeze theme...");
        kwrite("plasmarc", &["Theme"], "name", "default");
        kwrite("kdeglobals", &["KDE"], "widgetStyle", "Breeze");
        kwrite("kdeglobals", &["General"], "ColorScheme", "BreezeLight");
        kwrite(
            "kwinrc",
            &["org.kde.kdecoration2"],
            "library",
            "org.kde.breeze",
        );
        kwrite("kwinrc", &["org.kde.kdecoration2"], "theme", "@breeze");
        kwrite("kcminputrc", &["Mouse"], "cursorTheme", "breeze_cursors");
        output::ok("Reset KDE theme settings to Breeze.");
    }
    pub(super) fn prune_window_rules(&self) {
        let ours = ["caelestia-opacity", "caelestia-dialogs", "caelestia-pip"];
        let Some(index) = kde_read("kwinrulesrc", &["General"], "rules") else {
            return;
        };

        let kept: Vec<String> = index
            .split(',')
            .map(|name| name.trim().to_string())
            .filter(|name| !name.is_empty() && !ours.contains(&name.as_str()))
            .collect();

        if !kept.is_empty() {
            kwrite("kwinrulesrc", &["General"], "rules", &kept.join(","));
            kwrite(
                "kwinrulesrc",
                &["General"],
                "count",
                &kept.len().to_string(),
            );
        } else {
            kwrite_delete("kwinrulesrc", &["General"], "rules");
            kwrite_delete("kwinrulesrc", &["General"], "count");
        }
        output::ok("Removed the Caelestia window rules from kwinrulesrc");
    }
    pub(super) fn revert_darkly_gtk(&self) {
        let theme = paths::data_home().join("themes").join("Darkly");
        if theme.is_dir() {
            output::info(&format!(
                "Leaving {} in place (delete manually if you want).",
                theme.display()
            ));
        }

        for name in ["Darkly", "darkly"] {
            let _ = fs::remove_file(paths::data_home().join("plasma/desktoptheme").join(name));
        }

        let gtk4 = paths::config_dir().join("gtk-4.0");
        let backup = gtk4.join("gtk.css.created_by_darkly_installer.bak");
        if backup.is_file() {
            let _ = fs::rename(&backup, gtk4.join("gtk.css"));
        }

        if gtk4.join("gtk-darkly.css").is_file() || gtk4.join("darkly-gtk-assets").is_dir() {
            let _ = fs::remove_file(gtk4.join("gtk-darkly.css"));
            let _ = remove_tree(&gtk4.join("darkly-gtk-assets"));
            output::ok("Removed Darkly GTK libadwaita files");
        }

        if let Some(backup) = &self.backup {
            let konsolerc = backup.join(".config/konsolerc");
            if konsolerc.is_file() && copy_file(&konsolerc, &self.path(".config/konsolerc")) {
                output::ok("Restored konsolerc from backup");
            }
            let konsole = backup.join("local/konsole");
            if konsole.is_dir() {
                let _ = remove_tree(&self.path(".local/share/konsole"));
                if copy_tree(&konsole, &self.path(".local/share/konsole")) {
                    output::ok("Restored ~/.local/share/konsole from backup");
                }
            }
        }
    }
    pub(super) fn ensure_konsave(&mut self) -> bool {
        if let Some(binary) = process::locate("konsave") {
            self.konsave = Some(binary);
            return true;
        }

        if !process::available("python3") {
            return false;
        }

        output::info("Installing konsave for KDE profile restore...");
        let venv = paths::cache_dir()
            .join("caelestia-kde")
            .join("konsave-venv");
        let binary = venv.join("bin").join("konsave");

        if !binary.is_file() {
            if !process::succeeds("python3", &["-m", "venv", &venv.to_string_lossy()], None) {
                return false;
            }
            let python = venv.join("bin").join("python");
            let python = python.to_string_lossy().to_string();
            let _ = process::succeeds(&python, &["-m", "pip", "install", "--upgrade", "pip"], None);
            if !process::succeeds(
                &python,
                &["-m", "pip", "install", "--upgrade", "konsave"],
                None,
            ) {
                return false;
            }
        }

        self.konsave = Some(binary.clone());
        binary.is_file()
    }
    pub(super) fn restore_login_screen(&self) {
        for rule in [
            "/etc/sudoers.d/ydotoold-nopasswd",
            "/etc/sudoers.d/caelestia-greeter-sync",
            "/etc/sudoers.d/caelestia-sddm-sync",
        ] {
            let path = Path::new(rule);
            if path.is_file() && sudo_remove(path, false) {
                output::ok(&format!(
                    "Removed sudoers rule: {}",
                    path.file_name().unwrap_or_default().to_string_lossy()
                ));
            }
        }

        for dropin in [
            "/etc/sddm.conf.d/caelestia.conf",
            "/etc/sddm.conf.d/zz-caelestia.conf",
        ] {
            let path = Path::new(dropin);
            if path.is_file() && sudo_remove(path, false) {
                output::ok(&format!(
                    "Removed the SDDM drop-in an older install wrote: {}",
                    path.file_name().unwrap_or_default().to_string_lossy()
                ));
            }
        }

        let recorded = paths::state_dir().join("caelestia/sddm.conf.theme-current");
        if process::available("kwriteconfig6") {
            if let Some(previous) = read_line(&recorded) {
                let arguments: Vec<&str> = if previous == "#none" {
                    vec![
                        "--file",
                        "/etc/sddm.conf",
                        "--group",
                        "Theme",
                        "--key",
                        "Current",
                        "--delete",
                    ]
                } else {
                    vec![
                        "--file",
                        "/etc/sddm.conf",
                        "--group",
                        "Theme",
                        "--key",
                        "Current",
                        &previous,
                    ]
                };
                let _ = privileges::elevated("kwriteconfig6", &arguments, None);
                if previous == "#none" {
                    output::ok("Removed the SDDM theme selection an older install made");
                } else {
                    output::ok(&format!(
                        "Restored the SDDM theme selection an older install replaced ({previous})"
                    ));
                }
                let _ = fs::remove_file(&recorded);
            } else if kde_read("/etc/sddm.conf", &["Theme"], "Current").as_deref()
                == Some("caelestia")
            {
                let _ = privileges::elevated(
                    "kwriteconfig6",
                    &[
                        "--file",
                        "/etc/sddm.conf",
                        "--group",
                        "Theme",
                        "--key",
                        "Current",
                        "--delete",
                    ],
                    None,
                );
                output::ok("Removed the SDDM theme selection an older install made");
            }
        }

        if Path::new("/etc/plasmalogin.conf").is_file() && process::available("kwriteconfig6") {
            let _ = privileges::elevated(
                "kwriteconfig6",
                &[
                    "--file",
                    "/etc/plasmalogin.conf",
                    "--group",
                    "Greeter",
                    "--group",
                    "Wallpaper",
                    "--group",
                    "org.kde.image",
                    "--group",
                    "General",
                    "--key",
                    "Image",
                    "--delete",
                ],
                None,
            );
            output::ok("Removed the login screen wallpaper from /etc/plasmalogin.conf");
        }

        let home = passwd_home("plasmalogin");
        if let Some(home) = home.filter(|home| home != "/") {
            let home = PathBuf::from(home);
            let wallpapers = home.join("wallpapers/caelestia");
            if wallpapers.is_dir() && sudo_remove(&wallpapers, true) {
                output::ok("Removed the login screen's copy of the wallpaper");
            }
            for file in [
                "kdeglobals",
                "plasmarc",
                "kxkbrc",
                "kcminputrc",
                "plasma-localerc",
            ] {
                let path = home.join(".config").join(file);
                if path.is_file() {
                    let _ = sudo_remove(&path, false);
                }
            }
            let schemes = home.join(".local/share/color-schemes");
            if schemes.is_dir() {
                remove_by_prefix(&schemes, "Matugen");
                output::ok("Removed the login screen's copies of the color schemes");
            }
        }
    }
    pub(super) fn reload_kde(&self) {
        section("Step 11 - Reload KDE");

        let _ = process::succeeds("qdbus6", &["org.kde.KWin", "/KWin", "reconfigure"], None);
        let _ = process::succeeds(
            "systemctl",
            &["--user", "restart", "plasma-kglobalaccel.service"],
            None,
        );
        let _ = process::succeeds("kbuildsycoca6", &["--noincremental"], None);

        if process::available("lookandfeeltool") {
            if !self.previous_lookandfeel.is_empty() {
                output::info(&format!(
                    "Reapplying previous KDE look-and-feel: {}",
                    self.previous_lookandfeel
                ));
                if !process::succeeds(
                    "lookandfeeltool",
                    &["--apply", &self.previous_lookandfeel],
                    None,
                ) {
                    output::warn(&format!(
                        "Could not apply {} with lookandfeeltool.",
                        self.previous_lookandfeel
                    ));
                }
            } else if self.theme_restored {
                output::info(
                    "Skipping Breeze look-and-feel apply because theme was restored from backup.",
                );
            } else {
                let _ = process::succeeds(
                    "lookandfeeltool",
                    &["--apply", "org.kde.breeze.desktop"],
                    None,
                );
            }
        }

        output::ok("KDE reloaded");
    }
}
