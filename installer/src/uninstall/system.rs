use super::*;

impl Uninstaller {
    pub(super) fn remove_system_files(&self) {
        section("Step 8 - Remove System-level Files");

        let keyd = Path::new("/etc/keyd/quickshell.conf");
        if keyd.exists() && sudo_remove(keyd, keyd.is_dir()) {
            output::ok("Removed /etc/keyd/quickshell.conf");
            let _ = privileges::elevated("rmdir", &["/etc/keyd"], None);
        }

        for rule in [
            "/etc/udev/rules.d/70-uinput.rules",
            "/etc/udev/rules.d/80-uinput.rules",
        ] {
            let target = Path::new(rule);
            if target.is_file() && sudo_remove(target, false) {
                let _ = privileges::elevated("udevadm", &["control", "--reload-rules"], None);
                output::ok(&format!(
                    "Removed udev rule: {}",
                    target.file_name().unwrap_or_default().to_string_lossy()
                ));
            }
        }

        self.revert_ccache_flag();
        self.restore_login_screen();

        for target in [
            "/usr/local/bin/caelestia-greeter-sync",
            "/usr/local/bin/sass",
            "/usr/local/bin/qdbus6",
            "/usr/local/bin/caelestia",
            "/usr/local/bin/wl-clip-persist",
            "/usr/local/bin/gpu-screen-recorder",
        ] {
            let path = Path::new(target);
            let is_link = fs::symlink_metadata(path)
                .map(|meta| meta.file_type().is_symlink())
                .unwrap_or(false);
            if (is_link || (target == "/usr/local/bin/caelestia-greeter-sync" && path.is_file()))
                && sudo_remove(path, false)
            {
                output::ok(&format!("Removed: {target}"));
            }
        }

        for library in [
            "/usr/lib/qt6/plugins/kwin/effects/plugins/kwin_workspace_tracker.so",
            "/usr/lib64/qt6/plugins/kwin/effects/plugins/kwin_workspace_tracker.so",
        ] {
            let path = Path::new(library);
            if path.is_file() && sudo_remove(path, false) {
                output::ok(&format!("Removed system KWin effect: {library}"));
            }
        }

        self.remove_posthooks();
        let _ = fs::remove_file(self.path(".config/caelestia/templates/sddm-theme.conf"));
        self.leave_input_group();
    }
    pub(super) fn revert_ccache_flag(&self) {
        let flag = paths::state_dir().join("caelestia/ccache-enabled");
        if !flag.is_file() || !Path::new("/etc/makepkg.conf").is_file() {
            return;
        }

        let reverted = privileges::elevated(
            "sed",
            &[
                "-i",
                "s/\\(^\\|[[:space:]]\\)ccache\\([[:space:]]\\|$\\)/\\1!ccache\\2/",
                "/etc/makepkg.conf",
            ],
            None,
        ) == Ok(0);

        if reverted {
            let _ = fs::remove_file(&flag);
            output::ok("Reverted ccache in /etc/makepkg.conf");
        } else {
            output::warn(
                "Could not revert ccache in /etc/makepkg.conf; retaining ownership marker.",
            );
        }
    }
    pub(super) fn remove_packages(&self) {
        if !self.remove_packages {
            output::skip("Package removal skipped (user chose to keep packages)");
            return;
        }

        section("Step 9 - Remove Packages (Optional)");

        // The package names belong to the package module; an uninstall asks it which of them are
        // also desktop tools rather than keeping a second list to fall behind.
        let listed: Vec<&str> = packages::removable(self.family);

        let installed: Vec<&str> = listed
            .iter()
            .copied()
            .filter(|package| pkg::present(package))
            .collect();

        if !listed.is_empty() {
            self.remove_selected_packages(&installed);
        }

        if process::available("caelestia") {
            let _ = privileges::elevated("pip3", &["uninstall", "-y", "caelestia"], None);
            let _ = process::succeeds("pip3", &["uninstall", "-y", "caelestia"], None);
            output::ok("Removed caelestia pip package");
        }

        if process::available("uv") {
            let _ = process::succeeds(
                "uv",
                &["tool", "uninstall", "kde-material-you-colors"],
                None,
            );
            output::ok("Removed uv tools: kde-material-you-colors");
        }

        if process::available("kpackagetool6")
            && process::succeeds(
                "kpackagetool6",
                &["-t", "KWin/Script", "-s", "krohnkite"],
                None,
            )
        {
            let _ = process::succeeds(
                "kpackagetool6",
                &["-t", "KWin/Script", "-r", "krohnkite"],
                None,
            );
            output::ok("Removed Krohnkite KWin script");
        }
    }
    pub(super) fn remove_selected_packages(&self, installed: &[&str]) {
        if installed.is_empty() {
            output::skip("None of the listed packages are installed");
            return;
        }

        output::warn("Packages installed by caelestia that are also common desktop tools:");
        for (index, package) in installed.iter().enumerate() {
            println!("  {:>2}) {package}", index + 1);
        }
        println!("  Enter the numbers to remove (e.g. 1 3 5-9), 'all' for every package,");
        println!("  or press Enter to keep them all.");

        let answer = prompt("Selection [keep all]:");
        let answer = answer.trim().to_lowercase();
        let selected: Vec<&str> = if answer == "all" || answer == "a" {
            installed.to_vec()
        } else {
            select_indices(installed.len(), &answer)
                .into_iter()
                .filter_map(|index| installed.get(index).copied())
                .collect()
        };

        if selected.is_empty() {
            output::skip("Package removal skipped");
            return;
        }

        let removed = match self.family {
            Distro::Arch => {
                process::run_visible(
                    "yay",
                    &{
                        let mut args = vec!["-Rns", "--noconfirm"];
                        args.extend(selected.iter().copied());
                        args
                    },
                    None,
                ) == Some(0)
            }
            Distro::Fedora => {
                privileges::elevated(
                    "dnf",
                    &{
                        let mut args = vec!["remove", "-y"];
                        args.extend(selected.iter().copied());
                        args
                    },
                    None,
                ) == Ok(0)
            }
            Distro::Debian => {
                privileges::elevated(
                    "apt-get",
                    &{
                        let mut args = vec!["remove", "-y"];
                        args.extend(selected.iter().copied());
                        args
                    },
                    None,
                ) == Ok(0)
            }
            Distro::Unknown => false,
        };

        if removed {
            output::ok(&format!("Removed {} package(s)", selected.len()));
        } else {
            output::warn("Some packages could not be removed automatically. Check manually.");
        }
    }
    pub(super) fn clean_caches(&self) {
        section("Step 10 - Clean Up Cache and Build Artifacts");

        for build in ["shell/build", "shell/plugin/build"] {
            let path = self.bundle.join(build);
            if path.is_dir() && remove_tree(&path) {
                output::ok(&format!("Removed build dir: {}", path.display()));
            }
        }

        let cache = paths::cache_dir().join("caelestia-kde");
        if cache.is_dir() {
            if asked(&format!("Remove installer cache at {}?", cache.display())) {
                if remove_tree(&cache) {
                    output::ok("Removed installer cache");
                }
            } else {
                output::skip(&format!("Kept installer cache at {}", cache.display()));
            }
        }

        let state = paths::state_dir().join("caelestia");
        if state.is_dir() && remove_tree(&state) {
            output::ok(&format!("Removed {}", state.display()));
        }
    }
}
