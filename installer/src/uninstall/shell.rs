use super::*;

impl Uninstaller {
    pub(super) fn revert_shell_changes(&mut self) {
        section("Step 7 - Revert Shell Changes");

        if self.backup.is_some() {
            for (key, relative) in [
                ("bashrc", ".bashrc"),
                ("zshrc", ".zshrc"),
                ("fish_config", ".config/fish/config.fish"),
            ] {
                if self.restore_shell_rc(key, &self.path(relative)) {
                    self.shell_rc_restored = true;
                }
            }
        }

        self.revert_login_shell();

        if self.shell_rc_restored {
            output::info(
                "Skipped shell rc line cleanup because original rc files were restored exactly from backup.",
            );
        } else {
            for (relative, prefixes) in [
                (".bashrc", &["export"][..]),
                (".zshrc", &["export"][..]),
                (".config/fish/config.fish", &["set -gx", "set -x"][..]),
            ] {
                let rc = self.path(relative);
                if rc.is_file() && strip_caelestia_env(&rc, prefixes) {
                    output::ok(&format!("Removed Caelestia env vars from {}", rc.display()));
                }
            }
        }

        let environment = self.path(".config/environment.d/caelestia.conf");
        if environment.is_file() && fs::remove_file(&environment).is_ok() {
            output::ok("Removed the Caelestia environment file");
        }

        let plasma_env = self.path(".config/plasma-workspace/env/caelestia.sh");
        if plasma_env.is_file() {
            let _ = fs::remove_file(&plasma_env);
            let _ = fs::remove_dir(self.path(".config/plasma-workspace/env"));
            output::ok("Removed the Caelestia Plasma session environment script");
        }
    }
    pub(super) fn restore_shell_rc(&self, key: &str, target: &Path) -> bool {
        let Some(backup) = &self.backup else {
            return false;
        };

        let state_file = backup.join("shellrc").join(format!("{key}.state"));
        let Ok(state) = fs::read_to_string(&state_file) else {
            return false;
        };

        match state.trim() {
            "present" => {
                let backup_file = backup.join("shellrc").join(key);
                if backup_file.is_file() && copy_file(&backup_file, target) {
                    output::ok(&format!("Restored {} from backup", target.display()));
                    return true;
                }
                false
            }
            "missing" => {
                let _ = fs::remove_file(target);
                output::ok(&format!(
                    "Removed {} (it did not exist before install)",
                    target.display()
                ));
                true
            }
            _ => false,
        }
    }
    pub(super) fn revert_login_shell(&self) {
        let recorded = self
            .backup
            .as_ref()
            .and_then(|backup| read_line(&backup.join("previous_shell.txt")));

        let Some(recorded) = recorded else {
            if let Some(current) = login_shell() {
                output::info(&format!(
                    "No shell backup was recorded, so the login shell ({current}) is left as it is."
                ));
            }
            return;
        };

        let restore = if shell_is_registered(&recorded) {
            recorded.clone()
        } else {
            output::warn(&format!(
                "Previous shell ({recorded}) is not listed in /etc/shells. Falling back to bash."
            ));
            bash_path()
        };

        let user = user_name();
        if privileges::elevated("chsh", &["-s", &restore, &user], None) == Ok(0) {
            output::ok(&format!("Login shell reverted to {restore}"));
        } else {
            output::warn(&format!(
                "Could not change login shell to {restore}. Run: chsh -s {restore}"
            ));
        }
    }
    pub(super) fn remove_posthooks(&self) {
        let cli = self.path(".config/caelestia/cli.json");
        if !cli.is_file() || !process::available("python3") {
            return;
        }

        let target = cli.to_string_lossy().to_string();
        let mut command = std::process::Command::new("python3");
        command
            .args(["-", &target])
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null());

        let Ok(mut child) = command.spawn() else {
            return;
        };
        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(POSTHOOK_CLEANER.as_bytes());
        }
        if child.wait().map(|status| status.success()).unwrap_or(false) {
            output::ok("Removed the login screen posthook from cli.json");
        }
    }
    pub(super) fn leave_input_group(&self) {
        let user = user_name();
        let Some(groups) = process::capture("groups", &[&user]) else {
            return;
        };
        if !groups.split_whitespace().any(|group| group == "input") {
            return;
        }

        if privileges::elevated("gpasswd", &["-d", &user, "input"], None) == Ok(0) {
            output::ok(&format!(
                "Removed {user} from 'input' group (takes effect on next login)"
            ));
        } else {
            output::warn(&format!(
                "Could not remove the user from input group. Run: sudo gpasswd -d {user} input"
            ));
        }
    }
}
