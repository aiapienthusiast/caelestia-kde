use super::build::normalize_version;
use super::*;

pub(super) const PREBUILT_ASSETS: [&str; 2] = ["caelestia-kde", "caelestia-shell"];

pub(super) const QML_MODULES: [&str; 11] = [
    "Caelestia",
    "Caelestia/Components",
    "Caelestia/Config",
    "Caelestia/Settings",
    "Caelestia/Models",
    "Caelestia/Services",
    "Caelestia/Services/QuickShare",
    "Caelestia/Blobs",
    "Caelestia/Images",
    "Caelestia/Layouts",
    "M3Shapes",
];

pub(super) fn release_tag(bundle: &Path) -> Option<String> {
    let text = fs::read_to_string(bundle.join(".github/version.env")).ok()?;
    text.lines()
        .find_map(|line| line.trim().strip_prefix("VERSION="))
        .map(|tag| tag.trim().to_string())
        .filter(|tag| !tag.is_empty())
}

pub(super) fn checkout_may_use_prebuilt(bundle: &Path) -> bool {
    let Some(release) = release_tag(bundle) else {
        return false;
    };
    let Some(head) = process::capture_in("git", &["rev-parse", "HEAD"], Some(bundle)) else {
        return false;
    };

    if process::capture_in(
        "git",
        &["describe", "--tags", "--exact-match", &head],
        Some(bundle),
    )
    .as_deref()
        == Some(release.as_str())
    {
        return true;
    }

    if process::capture_in("git", &["branch", "--show-current"], Some(bundle)).as_deref()
        != Some("main")
    {
        return false;
    }
    process::capture_in(
        "git",
        &[
            "rev-parse",
            "--verify",
            "--quiet",
            "refs/remotes/origin/main",
        ],
        Some(bundle),
    )
    .as_deref()
        == Some(head.as_str())
}

pub(super) fn shell_qt_abi() -> Option<String> {
    process::capture("pkg-config", &["--modversion", "Qt6Core"])
        .map(|version| normalize_version(&version))
        .filter(|version| !version.is_empty())
}

pub(super) fn try_prebuilt_shell(bundle: &Path, home: &Path) -> bool {
    if process::capture("uname", &["-m"]).as_deref() != Some("x86_64") {
        return false;
    }
    if !Path::new("/etc/arch-release").exists() {
        return false;
    }
    let (Some(abi), Some(tag)) = (shell_qt_abi(), release_tag(bundle)) else {
        return false;
    };

    let Some(work) = TempDir::new("caelestia-prebuilt") else {
        return false;
    };
    let archive = work.path().join("shell.tar.gz");

    output::info(&format!(
        "Downloading prebuilt shell artifacts ({tag}, Qt {abi})..."
    ));
    let mut published = None;
    for asset in PREBUILT_ASSETS {
        let url = format!(
            "https://github.com/ladybug-me/caelestia-kde/releases/download/{tag}/{asset}-x86_64-qt{abi}.tar.gz"
        );
        if download::fetch(&url, &archive, &["--progress-bar"]) {
            published = Some(url);
            break;
        }
        let _ = fs::remove_file(&archive);
    }
    let Some(url) = published else {
        output::warn(&format!(
            "No prebuilt shell artifacts published for {tag} (Qt {abi}) - falling back to a local build."
        ));
        return false;
    };

    match download::verify(&url, &archive) {
        Verification::Mismatch => {
            output::warn(&format!(
                "Checksum mismatch for {url} - falling back to a local build."
            ));
            return false;
        }
        Verification::Unpublished => {
            let digest = download::sha256(&archive).unwrap_or_default();
            output::warn(&format!(
                "No published checksum for {url} - extracting without verification (SHA-256 {digest})."
            ));
        }
        Verification::Match => output::ok("Prebuilt shell artifacts match the published checksum."),
    }

    if !download::archive_entries_are_safe(&archive) {
        output::warn(&format!(
            "Refusing to extract {url}: the archive has setuid, setgid or escaping entries."
        ));
        return false;
    }

    output::info("Extracting prebuilt shell artifacts...");
    let local = home.join(".local");
    let config = home.join(".config");
    let _ = fs::create_dir_all(&local);
    let _ = fs::create_dir_all(&config);
    let archive = archive.to_string_lossy().to_string();

    for (directory, member) in [(&local, "lib"), (&config, "quickshell")] {
        if !extract(&archive, directory, member) {
            output::warn(&format!(
                "Failed to extract {member} from prebuilt shell archive"
            ));
            return false;
        }
    }

    if !archive_has(&archive, "bin/") || !archive_has(&archive, "lib/caelestia/") {
        output::warn(
            "Prebuilt shell artifact has no CMake-owned CLI/data tree - falling back to a local build.",
        );
        return false;
    }
    if !extract(&archive, &local, "bin") {
        output::warn("Failed to extract CMake-owned CLI files");
        return false;
    }
    true
}

pub(super) fn extract(archive: &str, directory: &Path, member: &str) -> bool {
    process::succeeds(
        "tar",
        &["-C", &directory.to_string_lossy(), "-xzf", archive, member],
        None,
    )
}

pub(super) fn archive_has(archive: &str, member: &str) -> bool {
    process::succeeds("tar", &["-tzf", archive, member], None)
}

pub(super) fn check_qml_modules(home: &Path) -> Result<(), Failure> {
    let base = home.join(".local").join("lib").join("qt6").join("qml");

    for module in QML_MODULES {
        let directory = base.join(module);
        if !directory.join("qmldir").is_file() {
            return Err(format!(
                "Missing QML module metadata: {}",
                directory.join("qmldir").display()
            )
            .into());
        }

        let has_library = fs::read_dir(&directory)
            .map(|entries| {
                entries
                    .filter_map(Result::ok)
                    .any(|entry| entry.path().extension().is_some_and(|e| e == "so"))
            })
            .unwrap_or(false);
        if !has_library {
            return Err(format!("Missing QML plugin library in {}", directory.display()).into());
        }
    }
    Ok(())
}
