use std::fs;
use std::path::{Path, PathBuf};

use crate::copy;
use crate::download::{self, Verification};
use crate::install_fs;
use crate::install_kind;
use crate::output;
use crate::paths;
use crate::process;
use crate::steps::{self, Context, Failure};
use crate::tmp::TempDir;
use crate::toolchain;
use crate::update_state;

mod build;
mod deploy;
mod effect;
mod lockscreen;
mod prebuilt;

use build::{Toolchain, build_and_install};
use deploy::{backup_shell_config, cleanup_legacy_fonts, install_icon_set, install_support_files};
use effect::build_workspace_tracker;
use lockscreen::{
    cleanup_legacy_lockscreen, configure_lockscreen_greeter, install_lockscreen_greeter,
};
use prebuilt::{check_qml_modules, checkout_may_use_prebuilt, try_prebuilt_shell};

pub fn run(context: &Context) -> Result<(), Failure> {
    let home = paths::home();
    let bundle = &context.bundle;
    let shell_dir = bundle.join("shell");

    if install_kind::is_packaged() {
        write_shell_environment(&home)?;
        output::skip("Nothing to build: the shell tree is the package's.");
        return Ok(());
    }

    let generator = if process::available("ninja") {
        "Ninja"
    } else {
        output::warn("ninja not found; falling back to Unix Makefiles (builds will be slower).");
        "Unix Makefiles"
    };
    let jobs = build_jobs();
    let load = process::capture("nproc", &[])
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "2".to_string());

    prepare_ccache();

    if context.variable("CAELESTIA_FRESH_INSTALL").as_deref() != Some("1") {
        standalone_update(context, &shell_dir)?;
    }

    output::info("Building the Caelestia shell...");
    if !shell_dir.is_dir() {
        return Err(format!("Shell directory not found at {}!", shell_dir.display()).into());
    }
    check_qml_deployment(bundle, &shell_dir)?;

    let theme = Toolchain {
        bundle: bundle.clone(),
        generator: generator.to_string(),
    };

    backup_shell_config(&home)?;

    let prebuilt = context.variable("CAELESTIA_FORCE_BUILD_SHELL").is_none()
        && checkout_may_use_prebuilt(bundle)
        && process::available("curl")
        && try_prebuilt_shell(bundle, &home);

    if prebuilt {
        output::ok("Using prebuilt shell artifacts from the release.");
    } else {
        build_and_install(&shell_dir, &theme, &jobs, &load)?;
    }

    build_workspace_tracker(&shell_dir, &theme, &jobs, &load);

    check_qml_modules(&home)?;

    write_shell_environment(&home)?;
    install_support_files(bundle, &home)?;
    install_icon_set(bundle, &home);

    update_state::record_installed_revision(bundle, &home.join(".config/quickshell/caelestia"));

    cleanup_legacy_fonts(&home);

    let deploying = context
        .variable("CAELESTIA_SKIP_DEPLOY")
        .as_deref()
        .is_none_or(|value| value == "0");
    let lock_screen = context.flag("APPLY_LOCKSCREEN", true);
    if !lock_screen {
        output::skip("Lock screen greeter disabled by user choice.");
    } else if deploying {
        cleanup_legacy_lockscreen(&home);
        if install_lockscreen_greeter(bundle, &home) {
            configure_lockscreen_greeter(&home);
        }
    } else {
        output::info("KDE Lock Screen installation and configuration skipped: deployment is off.");
    }

    output::ok(
        "Caelestia Shell and KDE Bridges built and installed successfully to user directory.",
    );
    Ok(())
}

pub(super) fn build_jobs() -> String {
    if let Some(jobs) = std::env::var("CAELESTIA_BUILD_JOBS")
        .ok()
        .filter(|jobs| !jobs.is_empty())
    {
        return jobs;
    }

    let available: i64 = process::capture("nproc", &[])
        .and_then(|count| count.parse().ok())
        .unwrap_or(2);
    (available - 1).max(1).to_string()
}

pub(super) fn prepare_ccache() {
    let directory = std::env::var("CCACHE_DIR")
        .ok()
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| {
            paths::cache_dir()
                .join("caelestia-kde")
                .join("ccache")
                .to_string_lossy()
                .to_string()
        });
    let _ = fs::create_dir_all(&directory);
    if process::available("ccache") {
        let _ = process::succeeds("ccache", &["--max-size", "8G"], None);
    }
}

pub(super) const ENVIRONMENT_HEADER: &str = "\
# Written by Caelestia. Read by systemd for every session process and by the
# user manager the shell's unit runs under.";

pub(super) const PLASMA_ENVIRONMENT_HEADER: &str = "\
#!/bin/sh
# Written by Caelestia. Sourced by KDE Plasma during session startup for KWin,
# kscreenlocker_greet, and graphical session processes.
#
# The command directory goes on PATH because a checkout's command lives in
# ~/.local/bin, which a session does not carry on every distribution: without
# this, a terminal here cannot run caelestia update by name. A package's
# /usr/bin is already on PATH, so this only matters for a checkout.";

pub fn write_shell_environment(home: &Path) -> Result<(), Failure> {
    let bin_dir = install_kind::bin_dir().to_string_lossy().to_string();
    let lib_dir = install_kind::lib_dir().to_string_lossy().to_string();
    let imports = install_kind::qml_import_path();
    let shell_config = install_kind::shell_config().to_string_lossy().to_string();

    let environment_d = home.join(".config").join("environment.d");
    output::info(&format!(
        "Writing the shell environment to {}",
        environment_d.join("caelestia.conf").display()
    ));
    fs::create_dir_all(&environment_d)
        .map_err(|error| format!("cannot use {}: {error}", environment_d.display()))?;
    fs::write(
        environment_d.join("caelestia.conf"),
        format!(
            "{ENVIRONMENT_HEADER}\n\
             QML2_IMPORT_PATH={imports}\n\
             CAELESTIA_LIB_DIR={lib_dir}\n\
             CAELESTIA_BIN_DIR={bin_dir}\n\
             CAELESTIA_SHELL_CONFIG={shell_config}\n"
        ),
    )
    .map_err(|error| format!("cannot write the shell environment: {error}"))?;
    output::ok("Shell environment written.");

    let plasma_env_d = home.join(".config").join("plasma-workspace").join("env");
    output::info(&format!(
        "Writing the Plasma session environment to {}",
        plasma_env_d.join("caelestia.sh").display()
    ));
    fs::create_dir_all(&plasma_env_d)
        .map_err(|error| format!("cannot use {}: {error}", plasma_env_d.display()))?;
    let script = plasma_env_d.join("caelestia.sh");
    fs::write(
        &script,
        format!(
            "{PLASMA_ENVIRONMENT_HEADER}\n\
             export PATH=\"{bin_dir}${{PATH:+:$PATH}}\"\n\
             export QML2_IMPORT_PATH=\"{imports}${{QML2_IMPORT_PATH:+:$QML2_IMPORT_PATH}}\"\n\
             export CAELESTIA_LIB_DIR=\"{lib_dir}\"\n\
             export CAELESTIA_BIN_DIR=\"{bin_dir}\"\n\
             export CAELESTIA_SHELL_CONFIG=\"{shell_config}\"\n"
        ),
    )
    .map_err(|error| format!("cannot write the Plasma environment: {error}"))?;
    paths::make_executable(&script);
    output::ok("Plasma session environment written.");

    for rc in [
        home.join(".bashrc"),
        home.join(".config").join("fish").join("config.fish"),
        home.join(".zshrc"),
    ] {
        remove_environment_lines(&rc);
    }
    Ok(())
}

pub(super) fn remove_environment_lines(rc: &Path) {
    if !rc.is_file() {
        return;
    }
    let Ok(text) = fs::read_to_string(rc) else {
        return;
    };
    if !text.lines().any(is_environment_line) {
        return;
    }

    let kept: Vec<&str> = text
        .lines()
        .filter(|line| !is_environment_line(line))
        .collect();
    if fs::write(rc, format!("{}\n", kept.join("\n"))).is_ok() {
        output::info(&format!(
            "Removed the Caelestia environment lines from {}",
            rc.file_name().unwrap_or_default().to_string_lossy()
        ));
    }
}

pub(super) fn is_environment_line(line: &str) -> bool {
    line.contains("CAELESTIA_LIB_DIR")
        || (line.contains("QML2_IMPORT_PATH") && line.contains("caelestia"))
}

pub(super) fn sparsify_checkout_paths(bundle: &Path) {
    for rule in SPARSIFY {
        let path = rule.trim_end_matches('/');
        if bundle.join(path).exists() {
            continue;
        }

        let Some(sparse_file) = sparse_checkout_file(bundle) else {
            continue;
        };
        let rules = fs::read_to_string(&sparse_file).unwrap_or_default();
        if !rules.lines().any(|line| line == rule) {
            let _ = fs::write(&sparse_file, format!("{rules}{rule}\n"));
        }
        let _ = process::succeeds("git", &["read-tree", "-mu", "HEAD"], Some(bundle));
        if !bundle.join(path).exists() {
            let _ = process::succeeds("git", &["checkout", "HEAD", "--", path], Some(bundle));
        }
    }
}

pub(super) fn sparse_checkout_file(bundle: &Path) -> Option<PathBuf> {
    let path = process::capture_in(
        "git",
        &["rev-parse", "--git-path", "info/sparse-checkout"],
        Some(bundle),
    )?;
    if path.is_empty() {
        return None;
    }
    let path = PathBuf::from(path);
    let path = if path.is_absolute() {
        path
    } else {
        bundle.join(path)
    };
    path.is_file().then_some(path)
}

pub(super) const SPARSIFY: [&str; 3] = ["installer/", "assets/", "uninstall.sh"];

/// Everything a shell run needs already in place, in the order the step table gives them. The list
/// is `steps::prerequisites`, so an update's plan names them instead of this function running them
/// behind its back.
pub(super) fn standalone_update(context: &Context, shell_dir: &Path) -> Result<(), Failure> {
    output::info("Running standalone update mode... ensuring prerequisites.");

    for id in steps::prerequisites("08-build-shell") {
        match *id {
            "02a-submodules" => run_step(context, id)?,
            "02-all-packages" if context.bundle.join("installer").is_dir() => {
                output::info("Checking core, shell, theme, and utility dependencies...");
                if run_step(context, id).is_err() {
                    output::warn("02-all-packages reported warnings");
                }
            }
            "02-all-packages" => {
                output::warn("Skipping the dependency check: installer/ is missing.");
                continue;
            }
            "06-services" => {
                output::info("Installing Caelestia Services...");
                if run_step(context, id).is_err() {
                    output::warn("06-services failed");
                }
            }
            "10-autostart" => {
                output::info("Updating autostart environment variables");
                if run_step(context, id).is_err() {
                    output::warn("10-autostart failed");
                }
            }
            "03a-wallpapers" => {
                output::info("Downloading the default wallpaper pack...");
                if run_step(context, id).is_err() {
                    output::warn("03a-wallpapers failed");
                }
            }
            _ => {}
        }
        if *id == "02a-submodules" {
            sparsify_checkout_paths(&context.bundle);
            output::info("Deleting yet-another-monochrome-icon-set for lag free update...");
            let icons = shell_dir.join("assets/icons/yet-another-monochrome-icon-set");
            if icons.is_dir() && fs::remove_dir_all(&icons).is_err() {
                output::warn("Failed to delete yet-another-monochrome-icon-set.");
            }
        }
    }

    Ok(())
}

pub(super) fn run_step(context: &Context, id: &str) -> Result<(), Failure> {
    let Some(step) = steps::find(id) else {
        return Err(format!("no step is called '{id}'").into());
    };
    if steps::run(step, context) == 0 {
        Ok(())
    } else {
        Err(format!("the {id} step failed").into())
    }
}

pub(super) fn check_qml_deployment(bundle: &Path, shell_dir: &Path) -> Result<(), Failure> {
    let checker = bundle.join(".github/scripts/check_qml_deployment.py");
    if !process::available("python3") || !checker.is_file() {
        return Ok(());
    }

    let arguments = [
        checker.to_string_lossy().to_string(),
        "--source-root".to_string(),
        shell_dir.to_string_lossy().to_string(),
    ];
    let args: Vec<&str> = arguments.iter().map(String::as_str).collect();
    if !process::succeeds("python3", &args, None) {
        return Err("QML source compatibility validation failed.".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::build::{normalize_version, prepare_build_dir};
    use super::deploy::cleanup_fonts_in;
    use super::lockscreen::{module_link, module_source};
    use super::*;
    use crate::tmp::TempDir;

    fn run_git(directory: &Path, arguments: &[&str]) -> bool {
        std::process::Command::new("git")
            .args(["-c", "user.email=test@example.com"])
            .args(["-c", "user.name=Test"])
            .args(["-c", "init.defaultBranch=main"])
            .args(arguments)
            .current_dir(directory)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .map(|status| status.success())
            .unwrap_or(false)
    }

    fn seed_checkout(work: &TempDir, name: &str, version: &str) -> PathBuf {
        let repo = work.path().join(name);
        fs::create_dir_all(repo.join(".github")).expect("a checkout");
        fs::write(
            repo.join(".github/version.env"),
            format!("VERSION={version}\n"),
        )
        .expect("the version");
        run_git(&repo, &["init", "--quiet"]);
        run_git(&repo, &["symbolic-ref", "HEAD", "refs/heads/main"]);
        run_git(&repo, &["add", "-A"]);
        run_git(&repo, &["commit", "--quiet", "-m", "release"]);
        run_git(&repo, &["update-ref", "refs/remotes/origin/main", "HEAD"]);
        repo
    }

    #[test]
    fn main_sitting_on_its_remote_tip_may_use_the_prebuilt() {
        if !process::available("git") {
            return;
        }
        let work = TempDir::new("caelestia-build-shell-test").expect("a temporary directory");
        let repo = seed_checkout(&work, "repo", "v1.0.0");

        assert!(checkout_may_use_prebuilt(&repo));
    }

    #[test]
    fn local_commits_on_main_build_locally() {
        if !process::available("git") {
            return;
        }
        let work = TempDir::new("caelestia-build-shell-test").expect("a temporary directory");
        let repo = seed_checkout(&work, "repo", "v1.0.0");
        fs::write(repo.join("local.txt"), b"local").expect("a local file");
        run_git(&repo, &["add", "-A"]);
        run_git(&repo, &["commit", "--quiet", "-m", "unreleased local work"]);

        assert!(
            !checkout_may_use_prebuilt(&repo),
            "main with commits of its own must not be overwritten by the prebuilt"
        );
    }

    #[test]
    fn main_behind_its_remote_builds_locally() {
        if !process::available("git") {
            return;
        }
        let work = TempDir::new("caelestia-build-shell-test").expect("a temporary directory");
        let repo = seed_checkout(&work, "repo", "v1.0.0");
        run_git(
            &repo,
            &["commit", "--quiet", "--allow-empty", "-m", "released later"],
        );
        run_git(&repo, &["update-ref", "refs/remotes/origin/main", "HEAD"]);
        run_git(&repo, &["reset", "--quiet", "--hard", "HEAD~1"]);

        assert!(
            !checkout_may_use_prebuilt(&repo),
            "a stale main checkout must not be treated as the released revision"
        );
    }

    #[test]
    fn another_branch_builds_locally() {
        if !process::available("git") {
            return;
        }
        let work = TempDir::new("caelestia-build-shell-test").expect("a temporary directory");
        let repo = seed_checkout(&work, "repo", "v1.0.0");
        run_git(&repo, &["checkout", "--quiet", "-b", "dev"]);

        assert!(
            !checkout_may_use_prebuilt(&repo),
            "dev must never install the prebuilt shell"
        );
    }

    #[test]
    fn the_released_tag_may_use_the_prebuilt() {
        if !process::available("git") {
            return;
        }
        let work = TempDir::new("caelestia-build-shell-test").expect("a temporary directory");
        let repo = seed_checkout(&work, "repo", "v1.0.0");
        run_git(
            &repo,
            &["commit", "--quiet", "--allow-empty", "-m", "next release"],
        );
        run_git(&repo, &["tag", "v2.0.0"]);
        fs::write(repo.join(".github/version.env"), "VERSION=v2.0.0\n").expect("the version");

        assert!(checkout_may_use_prebuilt(&repo));
    }

    #[test]
    fn a_version_that_is_not_the_tag_builds_locally() {
        if !process::available("git") {
            return;
        }
        let work = TempDir::new("caelestia-build-shell-test").expect("a temporary directory");
        let repo = seed_checkout(&work, "repo", "v1.0.0");
        run_git(
            &repo,
            &["commit", "--quiet", "--allow-empty", "-m", "next release"],
        );
        run_git(&repo, &["tag", "v2.0.0"]);

        assert!(!checkout_may_use_prebuilt(&repo));
    }

    #[test]
    fn a_checkout_without_a_version_builds_locally() {
        if !process::available("git") {
            return;
        }
        let work = TempDir::new("caelestia-build-shell-test").expect("a temporary directory");
        let repo = seed_checkout(&work, "repo", "v1.0.0");
        fs::remove_file(repo.join(".github/version.env")).expect("the version goes");

        assert!(
            !checkout_may_use_prebuilt(&repo),
            "without a version there is no release artifact to fetch"
        );
    }

    #[test]
    fn a_tree_that_is_not_a_checkout_builds_locally() {
        let work = TempDir::new("caelestia-build-shell-test").expect("a temporary directory");
        let tree = work.path().join("tree");
        fs::create_dir_all(tree.join(".github")).expect("a tree");
        fs::write(tree.join(".github/version.env"), "VERSION=v1.0.0\n").expect("the version");

        assert!(
            !checkout_may_use_prebuilt(&tree),
            "a source tree without git history must build locally"
        );
    }

    #[test]
    fn the_fonts_an_older_install_left_go_and_the_rest_stays() {
        let work = TempDir::new("caelestia-build-shell-test").expect("a temporary directory");
        let fonts = work.path().join("assets/fonts");
        for directory in ["SF-Pro", "SF-Mono", "google-sans-flex", "MyFonts"] {
            fs::create_dir_all(fonts.join(directory)).expect("a font directory");
            fs::write(fonts.join(directory).join("font.ttf"), b"font").expect("a font");
        }
        fs::write(
            fonts.join("README.md"),
            "# Fonts\n\nThe shell loads every .ttf/.otf found here at startup.\n",
        )
        .expect("the shipped readme");
        fs::write(work.path().join("assets/emojis.txt"), b"emoji").expect("the emoji database");

        let removed = cleanup_fonts_in(&fonts);

        assert_eq!(removed, ["SF-Pro", "SF-Mono", "google-sans-flex"]);
        assert!(!fonts.join("SF-Pro").exists());
        assert!(!fonts.join("README.md").exists());
        assert!(
            fonts.join("MyFonts/font.ttf").is_file(),
            "a font the user added themselves stays"
        );
        assert!(
            work.path().join("assets/emojis.txt").is_file(),
            "the emoji database beside it is not ours to remove"
        );
        assert!(
            fonts.is_dir(),
            "the directory stays while something of the user's is in it"
        );
    }

    #[test]
    fn only_the_shipped_readme_is_removed() {
        let work = TempDir::new("caelestia-build-shell-test").expect("a temporary directory");
        let fonts = work.path().join("fonts");
        fs::create_dir_all(&fonts).expect("the fonts directory");
        fs::write(fonts.join("MyFont.ttf"), b"font").expect("a font");
        fs::write(fonts.join("README.md"), "# My own font notes\n").expect("a readme");

        let removed = cleanup_fonts_in(&fonts);

        assert!(removed.is_empty());
        assert!(fonts.join("README.md").is_file(), "that one is the user's");
        assert!(fonts.join("MyFont.ttf").is_file());
    }

    #[test]
    fn a_fonts_directory_goes_once_it_is_empty() {
        let work = TempDir::new("caelestia-build-shell-test").expect("a temporary directory");
        let fonts = work.path().join("fonts");
        fs::create_dir_all(fonts.join("SF-Pro")).expect("a font directory");
        fs::write(fonts.join("SF-Pro/font.ttf"), b"font").expect("a font");
        fs::write(work.path().join("emojis.txt"), b"emoji").expect("the emoji database");

        assert_eq!(cleanup_fonts_in(&fonts), ["SF-Pro"]);

        assert!(
            !fonts.exists(),
            "an emptied directory goes with its contents"
        );
        assert!(work.path().join("emojis.txt").is_file());
    }

    #[test]
    fn a_machine_that_never_installed_fonts_is_a_no_op() {
        let work = TempDir::new("caelestia-build-shell-test").expect("a temporary directory");

        assert_eq!(
            cleanup_fonts_in(&work.path().join("absent")),
            Vec::<String>::new()
        );
    }

    #[test]
    fn a_toolchain_is_the_same_across_a_patch_release() {
        assert_eq!(normalize_version("3.28.3"), "3.28");
        assert_eq!(
            normalize_version("cmake version 3.28.1"),
            "cmake version 3.28"
        );
        assert_eq!(normalize_version("6"), "6");
        assert_eq!(normalize_version(""), "");
    }

    #[test]
    fn a_build_directory_is_kept_only_while_the_toolchain_is_the_one_it_was_made_with() {
        let work = TempDir::new("caelestia-build-shell-test").expect("a temporary directory");
        let directory = work.path().join("build");
        let theme = Toolchain {
            bundle: work.path().join("bundle"),
            generator: "Ninja".to_string(),
        };

        prepare_build_dir(&directory, &theme).expect("the build directory is prepared");
        fs::write(directory.join("CMakeCache.txt"), b"cache").expect("a configured cache");
        fs::write(directory.join("leftover.o"), b"object").expect("a build output");

        prepare_build_dir(&directory, &theme).expect("the same toolchain keeps the directory");
        assert!(
            directory.join("leftover.o").is_file(),
            "a configured directory with the same stamp is reused"
        );

        let other = Toolchain {
            bundle: work.path().join("bundle"),
            generator: "Unix Makefiles".to_string(),
        };
        prepare_build_dir(&directory, &other).expect("the build directory is prepared");
        assert!(
            !directory.join("leftover.o").exists(),
            "another generator starts from an empty directory"
        );
        assert!(!directory.join("CMakeCache.txt").is_file());
    }

    #[test]
    fn the_environment_names_the_layout_the_install_has() {
        let work = TempDir::new("caelestia-build-shell-test").expect("a temporary directory");
        let home = work.path().join("home");
        fs::create_dir_all(&home).expect("a home directory");

        write_shell_environment(&home).expect("the environment is written");

        let environment = fs::read_to_string(home.join(".config/environment.d/caelestia.conf"))
            .expect("the session environment");
        for key in [
            "QML2_IMPORT_PATH=",
            "CAELESTIA_LIB_DIR=",
            "CAELESTIA_BIN_DIR=",
            "CAELESTIA_SHELL_CONFIG=",
        ] {
            assert!(environment.contains(key), "{key} is missing: {environment}");
        }

        let plasma = home.join(".config/plasma-workspace/env/caelestia.sh");
        let script = fs::read_to_string(&plasma).expect("the Plasma environment");
        assert!(script.starts_with("#!/bin/sh\n"), "{script}");
        assert!(
            script.contains("export PATH=\""),
            "the command directory goes on PATH: {script}"
        );
        assert!(
            plasma.is_file(),
            "the Plasma session sources this file by path"
        );
    }

    #[test]
    fn the_environment_lines_an_older_install_added_are_taken_out() {
        let work = TempDir::new("caelestia-build-shell-test").expect("a temporary directory");
        let home = work.path().join("home");
        fs::create_dir_all(&home).expect("a home directory");
        fs::write(
            home.join(".bashrc"),
            "export EDITOR=nvim\nexport CAELESTIA_LIB_DIR=\"$HOME/.local/lib/caelestia\"\nexport QML2_IMPORT_PATH=\"$HOME/.local/lib/qt6/qml:caelestia\"\n",
        )
        .expect("a shell rc");

        write_shell_environment(&home).expect("the environment is written");

        let rc = fs::read_to_string(home.join(".bashrc")).expect("the shell rc");
        assert!(rc.contains("export EDITOR=nvim"), "{rc}");
        assert!(
            !rc.contains("CAELESTIA_LIB_DIR"),
            "the session gets it from environment.d now: {rc}"
        );
        assert!(!rc.contains("QML2_IMPORT_PATH"), "{rc}");
    }

    #[test]
    fn the_sparse_rules_are_the_paths_an_older_updater_never_fetched() {
        assert_eq!(SPARSIFY, ["installer/", "assets/", "uninstall.sh"]);
        for rule in SPARSIFY {
            assert!(rule.ends_with('/') || rule == "uninstall.sh", "{rule}");
        }
    }

    #[test]
    fn the_repair_fills_what_a_sparse_checkout_left_out() {
        if !process::available("git") {
            return;
        }

        let work = TempDir::new("caelestia-sparse").expect("a temporary directory");
        let repo = work.path().join("checkout");
        fs::create_dir_all(repo.join("shell")).expect("a checkout");
        fs::write(repo.join("shell/CMakeLists.txt"), "project(shell)\n").expect("a file");
        for rule in SPARSIFY {
            let path = repo.join(rule.trim_end_matches('/'));
            if rule.ends_with('/') {
                fs::create_dir_all(&path).expect("a directory");
                fs::write(path.join("content"), rule).expect("a file");
            } else {
                fs::write(&path, rule).expect("a file");
            }
        }
        run_git(&repo, &["init", "--quiet"]);
        run_git(&repo, &["add", "-A"]);
        run_git(&repo, &["commit", "--quiet", "-m", "release"]);
        run_git(&repo, &["config", "core.sparseCheckout", "true"]);
        fs::write(repo.join(".git/info/sparse-checkout"), "/shell/\n").expect("the rules");
        run_git(&repo, &["read-tree", "-mu", "HEAD"]);

        assert!(!repo.join("installer/content").exists(), "the fixture");
        assert!(!repo.join("assets/content").exists(), "the fixture");
        assert!(!repo.join("uninstall.sh").exists(), "the fixture");

        sparsify_checkout_paths(&repo);

        assert!(repo.join("installer/content").is_file(), "installer/");
        assert!(repo.join("assets/content").is_file(), "assets/");
        assert!(repo.join("uninstall.sh").is_file(), "uninstall.sh");
        assert!(
            repo.join("shell/CMakeLists.txt").is_file(),
            "a selected path was dropped"
        );
        let rules = fs::read_to_string(repo.join(".git/info/sparse-checkout")).expect("the rules");
        for rule in SPARSIFY {
            assert!(
                rules.lines().any(|line| line == rule),
                "{rule} is not named in the rules: {rules}"
            );
        }
    }

    #[test]
    fn a_module_is_found_under_the_first_import_path_that_has_it() {
        let work = TempDir::new("caelestia-lockscreen-modules").expect("a temporary directory");
        let empty = work.path().join("empty");
        let second = work.path().join("second");
        fs::create_dir_all(second.join("M3Shapes")).expect("the module");
        fs::write(second.join("M3Shapes/qmldir"), "module M3Shapes\n").expect("the qmldir");
        let imports = [empty, second.clone()];

        let found =
            module_source(&imports, "M3Shapes", "qmldir").expect("it is in the second path");
        assert_eq!(found, second.join("M3Shapes"));

        fs::remove_file(second.join("M3Shapes/qmldir")).expect("the qmldir goes");
        assert!(module_source(&imports, "M3Shapes", "qmldir").is_none());
    }

    #[test]
    fn the_lock_screen_package_points_at_the_modules_instead_of_copying_them() {
        let work = TempDir::new("caelestia-lockscreen-link").expect("a temporary directory");
        let source = work.path().join("M3Shapes");
        fs::create_dir_all(&source).expect("the module");
        fs::write(source.join("qmldir"), "module M3Shapes\n").expect("the qmldir");
        let beside = work.path().join("package/contents/lockscreen");
        fs::create_dir_all(&beside).expect("the package");
        fs::create_dir_all(beside.join("M3Shapes")).expect("the stale copy");

        assert!(module_link(&source, &beside.join("M3Shapes")));
        assert!(
            beside.join("M3Shapes/qmldir").is_file(),
            "the module is reachable"
        );
        #[cfg(unix)]
        assert!(
            beside.join("M3Shapes").is_symlink(),
            "a rebuild replaces the module, and the package must follow it"
        );
    }
}
