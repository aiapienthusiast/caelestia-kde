use super::*;

pub(super) const LEGACY_FONTS: [&str; 3] = ["SF-Pro", "SF-Mono", "google-sans-flex"];

pub(super) const LEGACY_FONT_README: &str = "# Fonts";

pub(super) fn backup_shell_config(home: &Path) -> Result<(), Failure> {
    let source = home.join(".config").join("quickshell").join("caelestia");
    if !source.is_dir() {
        return Ok(());
    }
    let root = paths::cache_dir().join("caelestia-kde").join("backups");

    match install_fs::snapshot_dir(&source, &root, "quickshell-caelestia", 3) {
        Ok(destination) => {
            output::info(&format!(
                "Backed up shell configuration to {}",
                destination.display()
            ));
            Ok(())
        }
        Err(error) => Err(format!(
            "Could not back up shell configuration into {}: {error}",
            root.display()
        )
        .into()),
    }
}

pub(super) fn install_support_files(bundle: &Path, home: &Path) -> Result<(), Failure> {
    let _ = fs::create_dir_all(home.join(".local").join("bin"));
    let units = home.join(".config").join("systemd").join("user");
    fs::create_dir_all(&units)
        .map_err(|error| format!("cannot use {}: {error}", units.display()))?;

    let lib_dir = install_kind::lib_dir();
    fs::create_dir_all(&lib_dir)
        .map_err(|error| format!("cannot use {}: {error}", lib_dir.display()))?;
    let state_lib = bundle.join("scripts/lib/update-state.sh");
    copy::file(&state_lib, &lib_dir.join("update-state.sh"))
        .map_err(|error| format!("cannot install the update-state helper: {error}"))?;

    output::info("Installing the update-checker units...");
    for unit in [
        "caelestia-update-checker.service",
        "caelestia-update-checker.timer",
    ] {
        copy::file(
            &bundle.join("src").join("systemd").join(unit),
            &units.join(unit),
        )
        .map_err(|error| format!("cannot install {unit}: {error}"))?;
    }

    if std::env::var("XDG_RUNTIME_DIR")
        .ok()
        .filter(|value| !value.is_empty())
        .is_some()
    {
        let _ = process::succeeds("systemctl", &["--user", "daemon-reload"], None);
        if process::succeeds(
            "systemctl",
            &[
                "--user",
                "enable",
                "--now",
                "caelestia-update-checker.timer",
            ],
            None,
        ) {
            output::ok("Update-checker timer enabled");
        }
    } else {
        output::info("Skipping update-checker timer setup (no XDG_RUNTIME_DIR)");
    }
    Ok(())
}

pub(super) fn install_icon_set(bundle: &Path, home: &Path) {
    let icons = bundle.join("src/yet-another-monochrome-icon-set");
    let destination =
        home.join(".config/quickshell/caelestia/assets/icons/yet-another-monochrome-icon-set");
    let staging = destination.with_file_name(format!(
        "{}.tmp",
        destination
            .file_name()
            .map(|name| name.to_string_lossy().to_string())
            .unwrap_or_default()
    ));

    if let Some(parent) = destination.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let _ = fs::remove_dir_all(&staging);
    if fs::create_dir_all(&staging).is_err() {
        output::warn("Failed to copy yet-another-monochrome-icon-set.");
        return;
    }

    if copy_tree_without_git(&icons, &staging).is_err() {
        let _ = fs::remove_dir_all(&staging);
        output::warn("Failed to copy yet-another-monochrome-icon-set.");
        return;
    }

    let _ = fs::remove_dir_all(&destination);
    if fs::rename(&staging, &destination).is_err() {
        let _ = fs::remove_dir_all(&staging);
        output::warn("Failed to copy yet-another-monochrome-icon-set.");
        return;
    }
    output::info("Yet another monochrome icon set copied successfully.");
}

pub(super) fn copy_tree_without_git(source: &Path, destination: &Path) -> std::io::Result<()> {
    fs::create_dir_all(destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let name = entry.file_name();
        if name == ".git" {
            continue;
        }
        let target = destination.join(&name);
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            copy_tree_without_git(&entry.path(), &target)?;
        } else if file_type.is_symlink() {
            let _ = copy::symlink(&target, &fs::read_link(entry.path()).unwrap_or_default());
        } else {
            fs::copy(entry.path(), &target)?;
            make_writable(&target);
        }
    }
    Ok(())
}

pub(super) fn make_writable(path: &Path) {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(metadata) = fs::metadata(path) {
            let mut permissions = metadata.permissions();
            permissions.set_mode(permissions.mode() | 0o200);
            let _ = fs::set_permissions(path, permissions);
        }
    }
    #[cfg(not(unix))]
    let _ = path;
}

pub(super) fn cleanup_legacy_fonts(home: &Path) {
    let roots = [
        install_kind::shell_config()
            .parent()
            .map(|directory| directory.join("assets").join("fonts"))
            .unwrap_or_default(),
        home.join(".local/share/caelestia/assets/fonts"),
    ];

    for root in roots {
        let removed = cleanup_fonts_in(&root);
        if !removed.is_empty() {
            output::ok(&format!(
                "Reclaimed the fonts an older install left in {} ({}); the shell no longer loads them.",
                root.display(),
                removed.join(" ")
            ));
        }
    }
}

pub(super) fn cleanup_fonts_in(root: &Path) -> Vec<String> {
    if !root.is_dir() {
        return Vec::new();
    }

    let mut removed = Vec::new();
    for entry in LEGACY_FONTS {
        let path = root.join(entry);
        if path.exists() && fs::remove_dir_all(&path).is_ok() {
            removed.push(entry.to_string());
        }
    }

    let readme = root.join("README.md");
    if fs::read_to_string(&readme)
        .map(|text| text.lines().next() == Some(LEGACY_FONT_README))
        .unwrap_or(false)
    {
        let _ = fs::remove_file(&readme);
    }

    let _ = fs::remove_dir(root);
    removed
}
