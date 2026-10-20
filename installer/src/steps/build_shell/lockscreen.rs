use super::*;

pub(super) const LOCKSCREEN_PACKAGE: &str = "src/kde/shells/caelestia.desktop";

pub(super) fn cleanup_legacy_lockscreen(home: &Path) {
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
    let _ = fs::remove_dir_all(
        home.join(".local/share/plasma/wallpapers/net.dosowisko.PlasmaApplicationWallpaper"),
    );
    let _ = fs::remove_file(
        paths::cache_dir()
            .join("caelestia-kde")
            .join("wallpaper-plugin-installed"),
    );

    if !crate::kde::available() {
        return;
    }
    let _ = crate::kde::write(
        "kscreenlockerrc",
        &["Greeter"],
        "WallpaperPlugin",
        "org.kde.image",
    );
    for (group, key) in [
        (
            "Wallpaper/net.dosowisko.PlasmaApplicationWallpaper/General",
            "command",
        ),
        ("LnF/General", "alwaysShowClock"),
        ("LnF/General", "showMediaControls"),
    ] {
        let groups: Vec<&str> = group.split('/').collect();
        crate::kde::delete("kscreenlockerrc", &groups, key);
    }
    let _ = crate::kde::delete("kscreenlockerrc", &["Greeter"], "Theme");
}

pub(super) fn install_lockscreen_greeter(bundle: &Path, home: &Path) -> bool {
    let source = bundle.join(LOCKSCREEN_PACKAGE);
    let destination = home.join(".local/share/plasma/shells/caelestia.desktop");

    if !source.is_dir() {
        output::warn(&format!(
            "Caelestia lock screen greeter source not found: {}",
            source.display()
        ));
        return false;
    }

    output::info("Installing Caelestia lock screen greeter.");
    if install_fs::atomic_replace_tree(&source, &destination, Some("metadata.json")).is_err() {
        output::warn(&format!(
            "Failed to install Caelestia lock screen greeter to {}",
            destination.display()
        ));
        return false;
    }

    if !carry_lockscreen_modules(&destination) {
        output::warn(
            "The QML modules the lock screen draws with were not found, so the lock screen will \
             fall back to the built-in one. Log out and back in to give the greeter this \
             session's import path.",
        );
        return false;
    }

    output::ok("Caelestia lock screen greeter installed.");
    true
}

/// The QML modules the lock screen is built from. `Caelestia` has one module per directory, so the
/// probe is the one that has to be there rather than a `qmldir` at the top.
pub(super) const LOCKSCREEN_MODULES: [(&str, &str); 2] =
    [("Caelestia", "Services/qmldir"), ("M3Shapes", "qmldir")];

pub(super) fn carry_lockscreen_modules(package: &Path) -> bool {
    let imports: Vec<PathBuf> = install_kind::qml_import_path()
        .split(':')
        .filter(|base| !base.is_empty())
        .map(PathBuf::from)
        .collect();
    let beside = package.join("contents").join("lockscreen");
    LOCKSCREEN_MODULES
        .iter()
        .all(|(name, probe)| match module_source(&imports, name, probe) {
            Some(source) => module_link(&source, &beside.join(name)),
            None => false,
        })
}

/// Where a module was installed, from the list of paths the engine searches for one.
pub(super) fn module_source(imports: &[PathBuf], name: &str, probe: &str) -> Option<PathBuf> {
    imports
        .iter()
        .map(|base| base.join(name))
        .find(|candidate| candidate.join(probe).is_file())
}

#[cfg(unix)]
pub(super) fn module_link(source: &Path, link: &Path) -> bool {
    if link.is_dir() && !link.is_symlink() {
        let _ = fs::remove_dir_all(link);
    }
    let _ = fs::remove_file(link);
    std::os::unix::fs::symlink(source, link).is_ok()
}

#[cfg(not(unix))]
pub(super) fn module_link(source: &Path, link: &Path) -> bool {
    install_fs::atomic_replace_tree(source, link, None).is_ok()
}

pub(super) fn configure_lockscreen_greeter(home: &Path) -> bool {
    if !crate::kde::available() {
        output::warn(
            "KDE config tools (kwriteconfig6) not found. Skipping KDE Lock Screen configuration.",
        );
        return false;
    }

    let package = home.join(".local/share/plasma/shells/caelestia.desktop");
    if !package.is_dir() || !package.join("metadata.json").is_file() {
        output::warn(&format!(
            "Caelestia lock screen shell package not found at {}. Skipping lock screen configuration to prevent session lockout.",
            package.display()
        ));
        return false;
    }

    if crate::kde::write(
        "plasmashellrc",
        &["Shell"],
        "ShellPackage",
        "caelestia.desktop",
    ) {
        output::ok("KDE Lock Screen configured to use Caelestia greeter.");
        return true;
    }
    output::warn("Failed to apply KDE Lock Screen configuration.");
    false
}
