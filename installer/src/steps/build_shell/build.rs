use super::*;

/// What the build is built with: the checkout, and the generator the user's toolchain was asked
/// for. Its `stamp` is what tells a cached build directory from a stale one.
pub(super) struct Toolchain {
    pub(super) bundle: PathBuf,
    pub(super) generator: String,
}

impl Toolchain {
    fn stamp(&self) -> String {
        let cmake = process::capture("cmake", &["--version"])
            .and_then(|text| {
                text.lines()
                    .next()
                    .map(|line| line.replace("cmake version ", ""))
                    .map(|line| line.trim().to_string())
            })
            .unwrap_or_default();
        let qt = process::capture("pkg-config", &["--modversion", "Qt6Core"])
            .map(|version| normalize_version(&version))
            .unwrap_or_default();

        format!(
            "bundle:{} cmake:{} qt6core:{} gen:{} cava:{}\n",
            self.bundle.display(),
            normalize_version(&cmake),
            qt,
            self.generator,
            toolchain::cava_state()
        )
    }
}

pub(super) fn normalize_version(text: &str) -> String {
    let mut parts = text.trim().split('.');
    match (parts.next(), parts.next()) {
        (Some(major), Some(minor)) => format!("{major}.{minor}"),
        _ => text.trim().to_string(),
    }
}

pub(super) fn prepare_build_dir(directory: &Path, theme: &Toolchain) -> Result<(), Failure> {
    let stamp = directory.join(".caelestia_toolchain_stamp");
    if directory.join("CMakeCache.txt").is_file()
        && stamp.is_file()
        && fs::read_to_string(&stamp).unwrap_or_default() == theme.stamp()
    {
        return Ok(());
    }

    let _ = fs::remove_dir_all(directory);
    fs::create_dir_all(directory)
        .map_err(|error| format!("cannot use {}: {error}", directory.display()))?;
    fs::write(&stamp, theme.stamp())
        .map_err(|error| format!("cannot write {}: {error}", stamp.display()))?;
    Ok(())
}

pub(super) fn build_command(
    program: &str,
    args: &[&str],
    directory: &Path,
) -> std::process::Command {
    let mut command = if process::available("nice") {
        let mut command = std::process::Command::new("nice");
        command.args(["-n", "10", program]);
        command
    } else {
        std::process::Command::new(program)
    };
    command.args(args).current_dir(directory);
    command
}

pub(super) fn show_build_errors(log: &Path) {
    let text = fs::read_to_string(log).unwrap_or_default();
    for line in text.lines().filter(|line| is_build_error(line)) {
        eprintln!("{line}");
    }
    eprintln!("----- last 20 lines of {} -----", log.display());
    let lines: Vec<&str> = text.lines().collect();
    for line in lines.iter().skip(lines.len().saturating_sub(20)) {
        eprintln!("{line}");
    }
}

pub(super) fn is_build_error(line: &str) -> bool {
    line.contains("error:")
        || line.contains("FAILED:")
        || line.contains("ninja: build stopped")
        || line.contains("CMake Error")
        || line.contains("undefined reference")
        || line.starts_with("ld: ")
        || line.contains("make: ***")
}

pub(super) fn build_and_install(
    shell_dir: &Path,
    theme: &Toolchain,
    jobs: &str,
    load: &str,
) -> Result<(), Failure> {
    if !toolchain::linguist_tools_available() {
        output::info("Installing Qt Linguist tools for UI translations...");
        if !toolchain::install_linguist_tools() {
            output::warn("Linguist tools install failed; the shell will stay in English.");
        }
    }

    let cache = paths::cache_dir().join("caelestia-kde");
    fs::create_dir_all(&cache).ok();
    let log = cache.join("shell-build.log");

    output::info("Configuring CMake...");
    let build = shell_dir.join("build");
    prepare_build_dir(&build, theme)?;

    let home = paths::home().to_string_lossy().to_string();
    let offline = std::env::var("CAELESTIA_OFFLINE").unwrap_or_else(|_| "OFF".to_string());
    let qsconf = format!("{home}/.config/quickshell/caelestia");
    let configured = run_logged(
        build_command(
            "cmake",
            &[
                "-G",
                &theme.generator,
                "-B",
                "build",
                "-DCMAKE_BUILD_TYPE=Release",
                "-DCAELESTIA_CACHE_DEPS=ON",
                &format!("-DCAELESTIA_OFFLINE={offline}"),
                &format!("-DCMAKE_INSTALL_PREFIX={home}/.local"),
                "-DINSTALL_BINDIR=bin",
                "-DINSTALL_DATADIR=lib/caelestia",
                &format!("-DINSTALL_QSCONFDIR={qsconf}"),
                "-DINSTALL_LIBDIR=lib/caelestia",
                "-DINSTALL_QMLDIR=lib/qt6/qml",
            ],
            shell_dir,
        ),
        &log,
        Logging::Terminal,
    );
    if !configured? {
        return Err("CMake configuration failed.".into());
    }

    output::info(&format!("Building with {jobs} parallel jobs..."));
    if !run_logged(
        build_command(
            "cmake",
            &["--build", "build", &format!("-j{jobs}"), "--", "-l", load],
            shell_dir,
        ),
        &log,
        Logging::ErrorsOnly,
    )? {
        output::err(&format!("Build failed. Full log: {}", log.display()));
        show_build_errors(&log);
        return Err("The shell build failed.".into());
    }

    output::info("Installing to user local dir...");
    if !run_logged(
        build_command("cmake", &["--install", "build"], shell_dir),
        &log,
        Logging::Append,
    )? {
        return Err(format!("Installation failed. Full log: {}", log.display()).into());
    }
    Ok(())
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub(super) enum Logging {
    Terminal,
    ErrorsOnly,
    Append,
    Silent,
}

pub(super) fn run_logged(
    mut command: std::process::Command,
    log: &Path,
    logging: Logging,
) -> Result<bool, Failure> {
    use std::io::Write;

    let append = matches!(logging, Logging::Append);
    let file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(append)
        .truncate(!append)
        .open(log);
    let mut file = file.map_err(|error| format!("cannot write {}: {error}", log.display()))?;

    let output = command
        .stdin(std::process::Stdio::null())
        .output()
        .map_err(|error| format!("could not run the build command: {error}"))?;
    file.write_all(&output.stdout)
        .and_then(|_| file.write_all(&output.stderr))
        .map_err(|error| format!("cannot write {}: {error}", log.display()))?;

    if logging != Logging::Silent {
        let combined = format!(
            "{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        for line in combined.lines() {
            if logging == Logging::ErrorsOnly
                && (line.contains("warning:") || line.contains("note:"))
            {
                continue;
            }
            println!("{line}");
        }
    }
    Ok(output.status.success())
}
