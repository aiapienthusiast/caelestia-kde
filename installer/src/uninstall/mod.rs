use std::fs;
use std::io::{BufRead, Write};
use std::path::{Path, PathBuf};

use crate::distro::{self, Distro};
use crate::output;
use crate::packages;
use crate::paths;
use crate::pkg;
use crate::privileges;
use crate::process;
use crate::report::{Report, Row};
use crate::steps::Failure;
use crate::theme::Role;

const STOCK_PANEL: &str = r"if (panels().length === 0) {
    var panel = new Panel;
    panel.alignment = 'center';
    panel.location = 'bottom';
    panel.hiding = 'dodgewindows';
    panel.height = 48;
    panel.addWidget('org.kde.plasma.kickoff');
    panel.addWidget('org.kde.plasma.icontasks');
    panel.addWidget('org.kde.plasma.systemtray');
    panel.addWidget('org.kde.plasma.digitalclock');
}";

const POSTHOOK_CLEANER: &str = r##"
import json, os, re, sys

cli_path = sys.argv[1]
if not os.path.exists(cli_path):
    sys.exit(0)

with open(cli_path) as f:
    config = json.load(f)

ours = re.compile(
    r'\s*&&\s*sudo\s+\S*(?:sync\.sh|caelestia-greeter-sync)\s+--posthook'
    r'|sudo\s+\S*(?:sync\.sh|caelestia-greeter-sync)\s+--posthook\s*&&\s*'
    r'|sudo\s+\S*(?:sync\.sh|caelestia-greeter-sync)\s+--posthook'
)

changed = False
for section in ("wallpaper", "theme"):
    hook = config.get(section, {}).get("postHook", "")
    if not hook:
        continue
    cleaned = ours.sub('', hook).strip()
    if cleaned != hook:
        changed = True
        if cleaned:
            config[section]["postHook"] = cleaned
        else:
            del config[section]["postHook"]

if changed:
    with open(cli_path, "w") as f:
        json.dump(config, f, indent=4)
"##;

mod kde;
mod services;
mod shell;
mod system;

struct Uninstaller {
    bundle: PathBuf,
    home: PathBuf,
    remove_packages: bool,
    family: Distro,
    backup: Option<PathBuf>,
    konsave_archive: Option<PathBuf>,
    konsave: Option<PathBuf>,
    theme_restored: bool,
    previous_lookandfeel: String,
    shell_rc_restored: bool,
}

const SHELL_ROW: &str = "shell";
const CONFIGS_ROW: &str = "configs";

pub fn run(bundle: &Path) -> Result<(), Failure> {
    let mut uninstaller = Uninstaller {
        bundle: bundle.to_path_buf(),
        home: paths::home(),
        remove_packages: false,
        family: distro::detect(),
        backup: None,
        konsave_archive: None,
        konsave: None,
        theme_restored: false,
        previous_lookandfeel: String::new(),
        shell_rc_restored: false,
    };

    uninstaller.greet()?;
    uninstaller.stop_services();
    uninstaller.remove_service_files();
    uninstaller.remove_shell_installation();
    uninstaller.remove_shell_state();
    uninstaller.remove_bridge_scripts();
    uninstaller.restore_or_remove_configs();
    uninstaller.revert_kde_settings();
    uninstaller.revert_shell_changes();
    uninstaller.remove_system_files();
    uninstaller.remove_packages();
    uninstaller.clean_caches();
    uninstaller.reload_kde();

    Report::for_terminal().panel(
        "uninstall",
        "done",
        &[
            Row::new(
                "removed",
                "the shell, its configs and its integrations",
                Role::Ok,
            ),
            Row::new(
                "backups",
                format!("{}/backups/", bundle.display()),
                Role::Foreground,
            ),
        ],
        &["Log out and back in for the desktop to be fully its own again.".to_string()],
    );
    println!();

    if asked("Would you like to log out now?") {
        println!("Logging out...");
        if let Err(problem) = crate::logout::now() {
            output::err(&problem);
        }
    } else {
        println!("Exiting script. Please remember to log out manually later.");
    }
    Ok(())
}

impl Uninstaller {
    pub(super) fn path(&self, relative: &str) -> PathBuf {
        self.home.join(relative)
    }
    pub(super) fn greet(&mut self) -> Result<(), Failure> {
        if self.family == Distro::Unknown {
            println!("Could not detect distribution. Select base:");
            println!("  1) Arch-based   2) Fedora-based   3) Debian-based   4) Exit");
            match prompt("Choice [1-4]:").trim() {
                "1" => self.family = Distro::Arch,
                "2" => self.family = Distro::Fedora,
                "3" => self.family = Distro::Debian,
                _ => return Err("Exiting.".into()),
            }
        }

        let report = Report::for_terminal();
        report.panel(
            "uninstall",
            self.family.label(),
            &[
                Row::new(
                    SHELL_ROW,
                    display_home(&self.path(".config/quickshell/caelestia"), &self.home),
                    Role::Foreground,
                ),
                Row::new(
                    CONFIGS_ROW,
                    display_home(&self.path(".config/caelestia"), &self.home),
                    Role::Foreground,
                ),
                Row::new(
                    "integrations",
                    "autostart, bridges, KWin script, lock screen greeter",
                    Role::Foreground,
                ),
                Row::new("packages", "left in place unless you ask", Role::Muted),
            ],
            &[format!(
                "Your original KDE settings are put back from the backup in {}/backups/, and \
                     anything this does not recognise is left alone.",
                self.bundle.display()
            )],
        );

        self.prime_sudo()?;

        println!();
        if !asked("Are you sure you want to uninstall Caelestia?") {
            return Err("Uninstall canceled.".into());
        }

        println!();
        println!("Remove installed packages as well? This will uninstall");
        println!("tools like foot, btop, fastfetch, and others.");
        self.remove_packages = asked("Remove packages?");

        self.choose_backup()?;
        Ok(())
    }
    pub(super) fn prime_sudo(&self) -> Result<(), Failure> {
        if privileges::is_root() || std::env::var("CAELESTIA_SUDO_PRIMED").as_deref() == Ok("1") {
            return Ok(());
        }

        match privileges::elevated("true", &[], None) {
            Ok(0) => Ok(()),
            _ => Err("Failed to obtain sudo privileges.".into()),
        }
    }
    pub(super) fn choose_backup(&mut self) -> Result<(), Failure> {
        let directory = self.bundle.join("backups");
        if !directory.is_dir() {
            return Ok(());
        }

        let mut backups: Vec<PathBuf> = fs::read_dir(&directory)
            .map(|entries| {
                entries
                    .flatten()
                    .map(|entry| entry.path())
                    .filter(|path| {
                        path.is_dir()
                            && path
                                .file_name()
                                .map(|name| is_stamp(&name.to_string_lossy()))
                                .unwrap_or(false)
                    })
                    .collect()
            })
            .unwrap_or_default();
        backups.sort();
        backups.reverse();

        if backups.is_empty() {
            return Ok(());
        }

        println!();
        println!("Available backups to restore from:");
        for (index, backup) in backups.iter().enumerate() {
            let mut tag = String::new();
            if konsave_archive(backup).is_none() {
                tag.push_str("");
            } else {
                tag.push_str(" [konsave]");
            }
            if let Some(previous) = read_line(&backup.join("previous_shell.txt")) {
                tag.push_str(&format!(
                    " [Shell: {}]",
                    Path::new(&previous)
                        .file_name()
                        .map(|name| name.to_string_lossy().to_string())
                        .unwrap_or(previous.clone())
                ));
            }
            println!("  {}) {}{tag}", index + 1, pretty_stamp(backup));
        }
        println!("  0) None (Do not restore from backup)");

        loop {
            let answer = prompt("Select a backup to restore [1]:");
            let answer = if answer.trim().is_empty() {
                "1".to_string()
            } else {
                answer.trim().to_string()
            };

            if answer == "0" {
                return Ok(());
            }

            let Ok(choice) = answer.parse::<usize>() else {
                println!("Invalid selection.");
                continue;
            };
            if choice < 1 || choice > backups.len() {
                println!("Invalid selection.");
                continue;
            }

            let selected = backups[choice - 1].clone();
            let archive = konsave_archive(&selected);

            if selected
                .join(".config")
                .join("quickshell")
                .join("caelestia")
                .join("shell.qml")
                .is_file()
            {
                println!();
                output::warn("The selected backup contains Caelestia configurations.");
                println!("   Restoring this backup will NOT revert to a clean KDE desktop!");
                println!("    Instead, it will restore a previous Caelestia state.");
                if !asked("Are you sure you want to restore this backup?") {
                    println!("  Backup selection canceled. Please select again.");
                    continue;
                }
            }

            self.backup = Some(selected);
            self.konsave_archive = archive;
            if let Some(previous) = read_line(
                &self
                    .backup
                    .clone()
                    .unwrap()
                    .join("previous_lookandfeel.txt"),
            ) {
                self.previous_lookandfeel = previous;
            }
            return Ok(());
        }
    }
    pub(super) fn restore_or_remove(&self, name: &str, target: &Path, backup_subdir: &str) {
        let backup_path = self
            .backup
            .as_ref()
            .map(|backup| backup.join(backup_subdir).join(name))
            .filter(|path| path.exists());

        if backup_path.is_none() && target.is_dir() {
            output::skip(&format!(
                "No backup for {name} - left in place (remove {} manually if you want it gone)",
                target.display()
            ));
            return;
        }

        let existed = target.exists();
        if existed {
            let _ = remove_path(target);
        }

        match backup_path {
            Some(backup) => {
                if copy_tree(&backup, target) {
                    output::ok(&format!("Restored {name} from backup"));
                } else {
                    output::warn(&format!(
                        "Failed to restore {name} from backup - check {} manually",
                        target.display()
                    ));
                }
            }
            None => output::skip(&format!("No backup for {name} - removed without restore")),
        }
    }
}

fn section(title: &str) {
    println!();
    println!("-------------------------------------------------------------");
    println!("  {title}");
    println!("-------------------------------------------------------------");
}

fn prompt(text: &str) -> String {
    print!("{text} ");
    let _ = std::io::stdout().flush();
    read_stdin_line()
}

fn asked(text: &str) -> bool {
    let answer = prompt(&format!("{text} [y/N]:")).trim().to_lowercase();
    answer == "y" || answer == "yes"
}

fn read_stdin_line() -> String {
    let mut line = String::new();
    let _ = std::io::stdin().lock().read_line(&mut line);
    line
}

pub fn select_indices(count: usize, input: &str) -> Vec<usize> {
    if count == 0 {
        return Vec::new();
    }

    let mut picked: Vec<usize> = Vec::new();
    for token in input.split(|letter: char| letter == ',' || letter.is_whitespace()) {
        if token.is_empty() {
            continue;
        }

        let (from, to) = match token.split_once('-') {
            Some((from, to)) => (from, to),
            None => (token, token),
        };
        let (Ok(from), Ok(to)) = (from.parse::<usize>(), to.parse::<usize>()) else {
            continue;
        };
        if from < 1 || from > count {
            continue;
        }

        for number in from..=to.min(count) {
            let index = number - 1;
            if !picked.contains(&index) {
                picked.push(index);
            }
        }
    }

    picked.sort_unstable();
    picked
}

fn konsave_archive(backup: &Path) -> Option<PathBuf> {
    let entries = fs::read_dir(backup).ok()?;
    let mut archives: Vec<PathBuf> = entries
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| {
            path.is_file() && path.extension().map(|kind| kind == "knsv").unwrap_or(false)
        })
        .collect();
    archives.sort();
    archives.into_iter().next()
}

fn is_stamp(name: &str) -> bool {
    let (date, time) = match name.split_once('_') {
        Some(parts) => parts,
        None => return false,
    };
    date.len() == 8
        && time.len() == 6
        && date.bytes().all(|byte| byte.is_ascii_digit())
        && time.bytes().all(|byte| byte.is_ascii_digit())
}

fn pretty_stamp(backup: &Path) -> String {
    let Some(name) = backup
        .file_name()
        .map(|name| name.to_string_lossy().to_string())
    else {
        return String::new();
    };
    let (date, time) = match name.split_once('_') {
        Some(parts) => parts,
        None => return name,
    };
    if date.len() != 8 || time.len() != 6 {
        return name;
    }
    format!(
        "{}-{}-{} {}:{}:{}",
        &date[0..4],
        &date[4..6],
        &date[6..8],
        &time[0..2],
        &time[2..4],
        &time[4..6]
    )
}

fn display_home(path: &Path, home: &Path) -> String {
    match path.strip_prefix(home) {
        Ok(relative) => format!("~/{}", relative.display()),
        Err(_) => path.display().to_string(),
    }
}

fn read_line(path: &Path) -> Option<String> {
    let text = fs::read_to_string(path).ok()?;
    let text = text.trim().to_string();
    if text.is_empty() { None } else { Some(text) }
}

fn file_mentions(path: &Path, needle: &str) -> bool {
    fs::read_to_string(path)
        .map(|text| {
            text.lines()
                .any(|line| line.trim_start().starts_with(&format!("{needle}=")))
        })
        .unwrap_or(false)
}

fn panel_in(appletsrc: &str) -> bool {
    appletsrc.lines().any(|line| {
        let line = line.trim();
        matches!(
            line.strip_prefix("plugin="),
            Some("org.kde.panel" | "org.kde.plasma.panel")
        )
    })
}

fn remove_tree(path: &Path) -> bool {
    fs::remove_dir_all(path).is_ok()
}

fn remove_path(path: &Path) -> bool {
    if path.is_dir() {
        remove_tree(path)
    } else {
        fs::remove_file(path).is_ok()
    }
}

fn remove_by_prefix(directory: &Path, prefix: &str) -> bool {
    let Ok(entries) = fs::read_dir(directory) else {
        return false;
    };
    let mut removed = false;
    for entry in entries.flatten() {
        if entry.file_name().to_string_lossy().starts_with(prefix) {
            let path = entry.path();
            removed |= if path.is_dir() {
                remove_tree(&path)
            } else {
                fs::remove_file(&path).is_ok()
            };
        }
    }
    removed
}

fn copy_file(source: &Path, target: &Path) -> bool {
    if let Some(parent) = target.parent() {
        let _ = fs::create_dir_all(parent);
    }
    fs::copy(source, target).is_ok()
}

fn copy_tree(source: &Path, target: &Path) -> bool {
    crate::copy::tree(source, target).is_ok()
}

fn sudo_remove(path: &Path, tree: bool) -> bool {
    let target = path.to_string_lossy().to_string();
    if tree {
        privileges::elevated("rm", &["-rf", &target], None) == Ok(0)
    } else {
        privileges::elevated("rm", &["-f", &target], None) == Ok(0)
    }
}

fn kwrite(file: &str, groups: &[&str], key: &str, value: &str) {
    let mut args = vec!["--file", file];
    for group in groups {
        args.push("--group");
        args.push(group);
    }
    args.push("--key");
    args.push(key);
    args.push(value);
    let _ = process::succeeds("kwriteconfig6", &args, None);
}

fn kwrite_delete(file: &str, groups: &[&str], key: &str) {
    let mut args = vec!["--file", file];
    for group in groups {
        args.push("--group");
        args.push(group);
    }
    args.push("--key");
    args.push(key);
    args.push("--delete");
    let _ = process::succeeds("kwriteconfig6", &args, None);
}

fn kde_read(file: &str, groups: &[&str], key: &str) -> Option<String> {
    let mut args = vec!["--file", file];
    for group in groups {
        args.push("--group");
        args.push(group);
    }
    args.push("--key");
    args.push(key);
    process::capture("kreadconfig6", &args).filter(|value| !value.is_empty())
}

fn unit_is_enabled_or_active(unit: &str) -> bool {
    process::succeeds(
        "systemctl",
        &["--user", "is-enabled", "--quiet", unit],
        None,
    ) || process::succeeds("systemctl", &["--user", "is-active", "--quiet", unit], None)
}

fn system_unit_is_enabled_or_active(unit: &str) -> bool {
    process::succeeds("systemctl", &["is-enabled", "--quiet", unit], None)
        || process::succeeds("systemctl", &["is-active", "--quiet", unit], None)
}

fn login_shell() -> Option<String> {
    let user = user_name();
    if user.is_empty() {
        return None;
    }
    process::capture("getent", &["passwd", &user])
        .and_then(|entry| entry.split(':').nth(6).map(str::to_string))
        .filter(|shell| !shell.is_empty())
}

fn shell_is_registered(shell: &str) -> bool {
    fs::read_to_string("/etc/shells")
        .map(|text| text.lines().any(|line| line.trim() == shell))
        .unwrap_or(false)
}

fn bash_path() -> String {
    process::locate("bash")
        .map(|path| path.to_string_lossy().to_string())
        .unwrap_or_else(|| "/bin/bash".to_string())
}

fn user_name() -> String {
    std::env::var("USER")
        .ok()
        .filter(|user| !user.is_empty())
        .or_else(|| process::capture("id", &["-un"]))
        .unwrap_or_default()
}

fn passwd_home(user: &str) -> Option<String> {
    process::capture("getent", &["passwd", user])
        .and_then(|entry| entry.split(':').nth(5).map(str::to_string))
        .filter(|home| !home.is_empty())
}

fn strip_caelestia_env(rc: &Path, prefixes: &[&str]) -> bool {
    let Ok(text) = fs::read_to_string(rc) else {
        return false;
    };
    let before = text.clone();

    let kept: Vec<&str> = text
        .lines()
        .filter(|line| {
            let trimmed = line.trim_start();
            !prefixes.iter().any(|prefix| {
                trimmed.starts_with(&format!("{prefix} QML2_IMPORT_PATH"))
                    || trimmed.starts_with(&format!("{prefix} CAELESTIA_LIB_DIR"))
            }) || !trimmed.contains("caelestia")
        })
        .collect();

    let updated = format!("{}\n", kept.join("\n"));
    if updated == before {
        return false;
    }
    fs::write(rc, updated).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_selection_reads_numbers_lists_and_ranges() {
        assert_eq!(select_indices(5, "1"), [0]);
        assert_eq!(select_indices(5, "1,3"), [0, 2]);
        assert_eq!(select_indices(5, "1 3"), [0, 2]);
        assert_eq!(select_indices(5, "2-4"), [1, 2, 3]);
        assert_eq!(select_indices(5, "1 3-5,2"), [0, 1, 2, 3, 4]);
    }

    #[test]
    fn a_selection_ignores_what_it_cannot_use() {
        assert_eq!(select_indices(3, ""), Vec::<usize>::new());
        assert_eq!(select_indices(3, "0"), Vec::<usize>::new());
        assert_eq!(select_indices(3, "9"), Vec::<usize>::new());
        assert_eq!(select_indices(3, "x"), Vec::<usize>::new());
        assert_eq!(select_indices(0, "1"), Vec::<usize>::new());
    }

    #[test]
    fn a_range_stops_at_the_last_item() {
        assert_eq!(select_indices(2, "1-9"), [0, 1]);
    }

    #[test]
    fn a_restore_clears_a_file_as_readily_as_a_directory() {
        let work =
            crate::tmp::TempDir::new("caelestia-uninstall-test").expect("a temporary directory");
        let file = work.path().join("starship.toml");
        let directory = work.path().join("btop");
        fs::write(&file, b"left by the install").expect("a file");
        fs::create_dir_all(&directory).expect("a directory");

        assert!(remove_path(&file));
        assert!(remove_path(&directory));
        assert!(!file.exists(), "the file is gone, not left to be re-read");
        assert!(!directory.exists());
    }

    #[test]
    fn a_backup_directory_is_the_stamp_the_installer_writes() {
        assert!(is_stamp("20260101_120000"));
        assert!(!is_stamp("20260101_12000"));
        assert!(!is_stamp("2026010_120000"));
        assert!(!is_stamp("backup"));
        assert_eq!(
            pretty_stamp(Path::new("/b/20260101_120000")),
            "2026-01-01 12:00:00"
        );
        assert_eq!(pretty_stamp(Path::new("/b/whatever")), "whatever");
    }

    #[test]
    fn an_environment_line_the_install_wrote_is_taken_out_and_the_rest_stays() {
        let work =
            crate::tmp::TempDir::new("caelestia-uninstall-test").expect("a temporary directory");
        let rc = work.path().join(".bashrc");
        fs::write(
            &rc,
            "# mine\nexport PATH=$HOME/.local/bin:$PATH\nexport QML2_IMPORT_PATH=$HOME/.local/lib/qt6/qml:$HOME/.config/quickshell/caelestia\nexport CAELESTIA_LIB_DIR=$HOME/.local/lib/caelestia\nexport EDITOR=vim\n",
        )
        .expect("a shell rc");

        assert!(strip_caelestia_env(&rc, &["export"]));
        let text = fs::read_to_string(&rc).expect("the file is still there");
        assert!(text.contains("# mine"));
        assert!(text.contains("EDITOR=vim"));
        assert!(!text.contains("QML2_IMPORT_PATH"));
        assert!(!text.contains("CAELESTIA_LIB_DIR"));
        assert!(
            !strip_caelestia_env(&rc, &["export"]),
            "a second pass has nothing to do"
        );
    }

    #[test]
    fn the_stock_panel_script_is_the_one_that_builds_a_panel() {
        assert!(STOCK_PANEL.contains("new Panel"));
        assert!(STOCK_PANEL.contains("org.kde.plasma.kickoff"));
    }

    #[test]
    fn a_desktop_with_only_a_wallpaper_is_a_desktop_without_a_panel() {
        let wallpaper = "[Containments][43]\nplugin=org.kde.plasma.folder\nformfactor=0\n";
        assert!(!panel_in(wallpaper));

        let panel = "[Containments][7]\nplugin=org.kde.panel\nlocation=4\n";
        assert!(panel_in(panel));
        assert!(panel_in(&format!("{wallpaper}{panel}")));
        assert!(panel_in("plugin=org.kde.plasma.panel\n"));
        assert!(!panel_in(""));
    }
}
