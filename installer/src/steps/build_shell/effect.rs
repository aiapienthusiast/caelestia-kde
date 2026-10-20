use super::build::{Logging, build_command, prepare_build_dir, run_logged, show_build_errors};
use super::*;

pub(super) const EFFECT_DIR: &str = "kwin-effects/workspace-tracker";

pub(super) fn workspace_stamp() -> PathBuf {
    paths::cache_dir()
        .join("caelestia-kde")
        .join("workspace-tracker.installed")
}

pub(super) fn built_effect(shell_dir: &Path) -> Option<PathBuf> {
    let root = shell_dir.join("kwin-effects/workspace-tracker/build");
    let mut found = Vec::new();
    collect_so(&root, &mut found)?;
    found.sort();
    found.into_iter().next()
}

pub(super) fn collect_so(directory: &Path, found: &mut Vec<PathBuf>) -> Option<()> {
    for entry in fs::read_dir(directory).ok()?.filter_map(Result::ok) {
        let path = entry.path();
        if path.is_dir() {
            collect_so(&path, found)?;
        } else if path.extension().and_then(|extension| extension.to_str()) == Some("so") {
            found.push(path);
        }
    }
    Some(())
}

pub(super) fn installed_effect(name: &str) -> Option<PathBuf> {
    [
        "/usr/lib/qt6/plugins/kwin/effects/plugins",
        "/usr/lib64/qt6/plugins/kwin/effects/plugins",
    ]
    .iter()
    .map(|directory| Path::new(directory).join(name))
    .find(|candidate| candidate.is_file())
}

/// The KWin the effect is being built against. The plugin version is baked into the binary, and an
/// effect built for another one is a plugin KWin quietly refuses to load - so a KWin update has to
/// count as a change even when nothing in the checkout moved.
pub(super) fn kwin_plugin_version() -> String {
    if let Ok(config) = fs::read_to_string("/usr/include/kwin/config-kwin.h")
        && let Some(version) = plugin_version_in(&config)
    {
        return version;
    }
    process::capture("kwin_wayland", &["--version"])
        .and_then(|text| version_triple_in(&text))
        .unwrap_or_else(|| "unknown".to_string())
}

/// `KWIN_PLUGIN_VERSION_STRING "6.2.1"` in KWin's own config header.
pub(super) fn plugin_version_in(config: &str) -> Option<String> {
    const MARKER: &str = "KWIN_PLUGIN_VERSION_STRING \"";
    let at = config.find(MARKER)? + MARKER.len();
    let rest = &config[at..];
    let end = rest.find('"')?;
    let version = &rest[..end];
    (!version.is_empty()).then(|| version.to_string())
}

/// The first `x.y.z` in a version banner.
pub(super) fn version_triple_in(text: &str) -> Option<String> {
    let bytes: Vec<char> = text.chars().collect();
    let mut start = None;
    for (index, character) in bytes.iter().enumerate() {
        if character.is_ascii_digit() {
            if start.is_none() {
                start = Some(index);
            }
        } else if *character == '.' && start.is_some() {
            continue;
        } else if let Some(begin) = start {
            let candidate: String = bytes[begin..index].iter().collect();
            if candidate.matches('.').count() == 2 {
                return Some(candidate);
            }
            start = None;
        }
    }
    let begin = start?;
    let candidate: String = bytes[begin..].iter().collect();
    (candidate.matches('.').count() == 2).then_some(candidate)
}

/// The KWin version an effect was compiled for, read out of the plugin factory symbol it exports.
pub(super) fn effect_kwin_version(effect: &Path) -> String {
    const MARKER: &str = "org.kde.kwin.EffectPluginFactory";
    let Ok(bytes) = fs::read(effect) else {
        return String::new();
    };
    let text = String::from_utf8_lossy(&bytes);
    let Some(at) = text.find(MARKER) else {
        return String::new();
    };
    text[at + MARKER.len()..]
        .chars()
        .take_while(|character| character.is_ascii_digit() || *character == '.')
        .collect()
}

pub(super) fn effect_signature(built: &Path, installed: &Path) -> String {
    let digest = download::sha256(built).unwrap_or_default();
    let stamp = fs::metadata(installed)
        .map(|metadata| {
            let modified = metadata
                .modified()
                .ok()
                .and_then(|time| time.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|since| since.as_secs())
                .unwrap_or_default();
            format!("{}:{modified}", metadata.len())
        })
        .unwrap_or_else(|_| "missing".to_string());
    format!("{} {digest} {stamp}\n", kwin_plugin_version())
}

pub(super) fn effect_up_to_date(shell_dir: &Path) -> bool {
    let Some(built) = built_effect(shell_dir) else {
        return false;
    };
    let Some(name) = built.file_name() else {
        return false;
    };
    let Some(installed) = installed_effect(&name.to_string_lossy()) else {
        return false;
    };
    let target = kwin_plugin_version();
    if target != "unknown" {
        if effect_kwin_version(&installed) != target {
            return false;
        }
        if effect_kwin_version(&built) != target {
            return false;
        }
    }
    let stamp = workspace_stamp();
    fs::read_to_string(&stamp).unwrap_or_default() == effect_signature(&built, &installed)
}

pub(super) fn record_effect_install(shell_dir: &Path) {
    let Some(built) = built_effect(shell_dir) else {
        return;
    };
    let Some(name) = built.file_name() else {
        return;
    };
    let Some(installed) = installed_effect(&name.to_string_lossy()) else {
        return;
    };

    let stamp = workspace_stamp();
    if let Some(parent) = stamp.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let _ = fs::write(stamp, effect_signature(&built, &installed));
}

pub(super) fn build_workspace_tracker(shell_dir: &Path, theme: &Toolchain, jobs: &str, load: &str) {
    output::info("Building and installing workspace-tracker KWin Effect...");
    let target = kwin_plugin_version();
    if let Some(previous) = built_effect(shell_dir)
        && target != "unknown"
        && !effect_kwin_version(&previous).is_empty()
        && effect_kwin_version(&previous) != target
    {
        output::info(&format!(
            "KWin is {target} now; cleaning the workspace-tracker build for it..."
        ));
        let _ = fs::remove_dir_all(shell_dir.join(EFFECT_DIR).join("build"));
    }
    let build = shell_dir.join(EFFECT_DIR).join("build");
    if prepare_build_dir(&build, theme).is_err() {
        output::warn("Workspace tracker configuration failed; skipping KWin effect build.");
        return;
    }

    let configured = run_logged(
        build_command(
            "cmake",
            &[
                "-G",
                &theme.generator,
                "-B",
                "kwin-effects/workspace-tracker/build",
                "-S",
                EFFECT_DIR,
                "-DCMAKE_BUILD_TYPE=Release",
                "-DCMAKE_INSTALL_PREFIX=/usr",
            ],
            shell_dir,
        ),
        &paths::cache_dir().join("caelestia-kde/workspace-tracker-configure.log"),
        Logging::Silent,
    );
    if !matches!(configured, Ok(true)) {
        output::warn("Workspace tracker configuration failed; skipping KWin effect build.");
        return;
    }

    let build_log = paths::cache_dir().join("caelestia-kde/workspace-tracker-build.log");
    let built = run_logged(
        build_command(
            "cmake",
            &[
                "--build",
                "kwin-effects/workspace-tracker/build",
                &format!("-j{jobs}"),
                "--",
                "-l",
                load,
            ],
            shell_dir,
        ),
        &build_log,
        Logging::Silent,
    );
    if !matches!(built, Ok(true)) {
        output::warn(&format!(
            "Workspace tracker build failed. Full log: {}",
            build_log.display()
        ));
        show_build_errors(&build_log);
        return;
    }

    if effect_up_to_date(shell_dir) {
        output::info("Workspace tracker already up to date; skipping system install.");
        enable_effect(false);
        return;
    }

    let absolute = shell_dir.join(EFFECT_DIR).join("build");
    let absolute = absolute.to_string_lossy().to_string();
    if crate::privileges::elevated("cmake", &["--install", &absolute], None) != Ok(0) {
        output::warn("Workspace tracker system installation failed.");
        return;
    }
    record_effect_install(shell_dir);
    enable_effect(true);
}

pub(super) fn enable_effect(reconfigure: bool) {
    let _ = crate::kde::write(
        "kwinrc",
        &["Plugins"],
        "kwin_workspace_trackerEnabled",
        "true",
    );
    if reconfigure {
        let _ = process::succeeds(
            "qdbus6",
            &[
                "org.kde.KWin",
                "/Effects",
                "org.kde.kwin.Effects.loadEffect",
                "kwin_workspace_tracker",
            ],
            None,
        );
        let _ = process::succeeds("qdbus6", &["org.kde.KWin", "/KWin", "reconfigure"], None);
        output::ok("Installed workspace-tracker to KDE.");
    }
}
