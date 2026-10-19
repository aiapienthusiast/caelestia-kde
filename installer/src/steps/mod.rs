mod autostart;
mod backup;
mod build_shell;
mod configs;
mod kde_apps;
mod kde_theme;
mod login_screen;
mod optional_apps;
mod prereqs;
mod services;
mod submodules;
mod system_tweaks;
mod system_update;
mod wallpapers;
mod window_rules;

use std::path::PathBuf;

pub use configs::recorded_backup_dir;

use crate::menu::Answers;
use crate::output;
use crate::packages;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Phase {
    Prepare,
    Packages,
    Configure,
    Build,
    Finalize,
}

impl Phase {
    pub const ALL: [Phase; 5] = [
        Phase::Prepare,
        Phase::Packages,
        Phase::Configure,
        Phase::Build,
        Phase::Finalize,
    ];

    pub fn id(self) -> &'static str {
        match self {
            Phase::Prepare => "prepare",
            Phase::Packages => "packages",
            Phase::Configure => "configure",
            Phase::Build => "build",
            Phase::Finalize => "finalize",
        }
    }

    pub fn title(self) -> &'static str {
        match self {
            Phase::Prepare => "Prepare",
            Phase::Packages => "Packages",
            Phase::Configure => "Configure",
            Phase::Build => "Build",
            Phase::Finalize => "Finalize",
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Status {
    Pending,
    Running,
    Ok,
    Warn,
    Failed,
    Skipped,
    Ignored,
}

impl Status {
    #[cfg(test)]
    pub const ALL: [Status; 7] = [
        Status::Pending,
        Status::Running,
        Status::Ok,
        Status::Warn,
        Status::Failed,
        Status::Skipped,
        Status::Ignored,
    ];

    pub fn is_failure(self) -> bool {
        matches!(self, Status::Failed)
    }

    /// The word this status is written as in a journal. The journal is a file on disk, so the
    /// spelling belongs here with the variants rather than at each writer and each reader.
    pub fn as_journal(self) -> &'static str {
        match self {
            Status::Ok => "ok",
            Status::Warn => "warn",
            Status::Failed => "failed",
            Status::Skipped | Status::Ignored => "skipped",
            Status::Pending | Status::Running => "pending",
        }
    }

    pub fn from_journal(word: &str) -> Option<Status> {
        match word {
            "ok" => Some(Status::Ok),
            "warn" => Some(Status::Warn),
            "failed" => Some(Status::Failed),
            "skipped" => Some(Status::Skipped),
            "pending" => Some(Status::Pending),
            _ => None,
        }
    }

    /// A step that will not be run again: whatever went wrong, it is settled.
    pub fn settles(self) -> bool {
        matches!(self, Status::Ok | Status::Warn | Status::Skipped | Status::Ignored)
    }

    /// A step the user should be told about: it finished badly, or it did not finish.
    pub fn is_problem(self) -> bool {
        matches!(self, Status::Warn | Status::Failed)
    }
}

pub enum Failure {
    Message(String),
    Reported,
}

impl std::fmt::Debug for Failure {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Failure::Message(message) => formatter.write_str(message),
            Failure::Reported => formatter.write_str("reported to the user"),
        }
    }
}

impl From<String> for Failure {
    fn from(message: String) -> Self {
        Failure::Message(message)
    }
}

impl From<&str> for Failure {
    fn from(message: &str) -> Self {
        Failure::Message(message.to_string())
    }
}

pub type Act = fn(&Context) -> Result<(), Failure>;

pub struct Step {
    pub name: &'static str,
    pub id: &'static str,
    pub phase: Phase,
    pub run: Act,
}

pub struct Context {
    pub bundle: PathBuf,
}

impl Context {
    pub fn variable(&self, name: &str) -> Option<String> {
        std::env::var(name).ok().filter(|value| !value.is_empty())
    }

    pub fn flag(&self, name: &str, fallback: bool) -> bool {
        match std::env::var(name) {
            Ok(value) => value == "true",
            Err(_) => fallback,
        }
    }
}

pub const STEPS: [Step; 17] = [
    Step {
        name: "Update system",
        id: "00a-system-update",
        phase: Phase::Prepare,
        run: system_update::run,
    },
    Step {
        name: "Check for required tools",
        id: "01-ensure-prereqs",
        phase: Phase::Prepare,
        run: prereqs::run,
    },
    Step {
        name: "Fetch the shell sources",
        id: "02a-submodules",
        phase: Phase::Prepare,
        run: submodules::run,
    },
    Step {
        name: "Back up current setup",
        id: "00-backup-themes",
        phase: Phase::Prepare,
        run: backup::run,
    },
    Step {
        name: "Install packages",
        id: "02-all-packages",
        phase: Phase::Packages,
        run: packages::all,
    },
    Step {
        name: "Set up the theming tools",
        id: "02-packages",
        phase: Phase::Packages,
        run: packages::tooling,
    },
    Step {
        name: "Deploy config files",
        id: "03-deploy-configs",
        phase: Phase::Configure,
        run: configs::run,
    },
    Step {
        name: "Download wallpapers",
        id: "03a-wallpapers",
        phase: Phase::Configure,
        run: wallpapers::run,
    },
    Step {
        name: "Apply KDE theme",
        id: "04-deploy-kde",
        phase: Phase::Configure,
        run: kde_theme::run,
    },
    Step {
        name: "Apply window rules",
        id: "04a-window-rules",
        phase: Phase::Configure,
        run: window_rules::run,
    },
    Step {
        name: "Match the login screen",
        id: "05-login-screen",
        phase: Phase::Configure,
        run: login_screen::run,
    },
    Step {
        name: "Enable system services",
        id: "06-services",
        phase: Phase::Configure,
        run: services::run,
    },
    Step {
        name: "Configure KDE applications",
        id: "07-kde-apps",
        phase: Phase::Configure,
        run: kde_apps::run,
    },
    Step {
        name: "Build Caelestia shell",
        id: "08-build-shell",
        phase: Phase::Build,
        run: build_shell::run,
    },
    Step {
        name: "Apply system tweaks",
        id: "09-system-tweaks",
        phase: Phase::Finalize,
        run: system_tweaks::run,
    },
    Step {
        name: "Create autostart entries",
        id: "10-autostart",
        phase: Phase::Finalize,
        run: autostart::run,
    },
    Step {
        name: "Install the apps you picked",
        id: "11-optional-apps",
        phase: Phase::Finalize,
        run: optional_apps::run,
    },
];

pub fn find(id: &str) -> Option<&'static Step> {
    STEPS.iter().find(|step| step.id == id)
}

pub fn environment(answers: &Answers, bundle: &std::path::Path) -> Vec<(&'static str, String)> {
    let mut env: Vec<(&'static str, String)> = answers
        .iter()
        .map(|(key, value)| (*key, value.clone()))
        .collect();
    env.push(("CONFIRM_ARG", "--noconfirm".to_string()));
    env.push(("BUNDLE_DIR", bundle.display().to_string()));
    env.push(("CAELESTIA_FRESH_INSTALL", "1".to_string()));
    env.push((
        crate::journal::PLAN_ID_VAR,
        crate::journal::plan_id(answers),
    ));
    env
}

pub fn wallpapers_dir(context: &Context) -> PathBuf {
    if let Some(directory) = context.variable("CAELESTIA_WALLPAPERS_DIR") {
        return PathBuf::from(directory);
    }
    if let Some(directory) = context.variable("XDG_PICTURES_DIR") {
        return PathBuf::from(directory).join("Wallpapers");
    }
    if crate::process::available("xdg-user-dir")
        && let Some(pictures) = crate::process::capture("xdg-user-dir", &["PICTURES"])
    {
        return PathBuf::from(pictures).join("Wallpapers");
    }
    crate::paths::home().join("Pictures").join("Wallpapers")
}

pub fn ensure_five_desktops(context: &Context) -> Result<(), Failure> {
    if context.variable("CAELESTIA_FRESH_INSTALL").as_deref() != Some("1") {
        output::skip("Not an install - leaving the virtual desktops alone.");
        return Ok(());
    }

    let count = crate::process::capture(
        "kreadconfig6",
        &["--file", "kwinrc", "--group", "Desktops", "--key", "Number"],
    )
    .unwrap_or_default();

    if !count.is_empty() && count != "1" {
        output::skip(&format!("Keeping the existing {count} virtual desktops."));
        return Ok(());
    }

    output::info("Configuring 5 virtual desktops...");

    let mut written = true;
    for (key, value) in [("Number", "5"), ("Rows", "1")] {
        written &= crate::process::succeeds(
            "kwriteconfig6",
            &[
                "--file", "kwinrc", "--group", "Desktops", "--key", key, value,
            ],
            None,
        );
    }
    for index in 1..=5 {
        written &= crate::process::succeeds(
            "kwriteconfig6",
            &[
                "--file",
                "kwinrc",
                "--group",
                "Desktops",
                "--key",
                &format!("Name_{index}"),
                &format!("Desktop {index}"),
            ],
            None,
        );
    }

    if written {
        output::ok("5 virtual desktops configured.");
    } else {
        output::warn("Could not write the virtual desktop configuration.");
    }
    Ok(())
}

pub fn run(step: &Step, context: &Context) -> i32 {
    match (step.run)(context) {
        Ok(()) => 0,
        Err(Failure::Message(problem)) => {
            output::err(&problem);
            1
        }
        Err(Failure::Reported) => 1,
    }
}

/// The switches a step's own body reads as well as the plan. Spelling each name once means the
/// plan's decision to skip a step and the step's decision to do nothing cannot drift apart.
pub const SKIP_SYSTEM_UPDATE: &str = "SKIP_SYSTEM_UPDATE";
pub const SYNC_LOGIN_SCREEN: &str = "SYNC_LOGIN_SCREEN";

/// The optional apps, which are the apps the installer can theme: one list, owned by `apps`, so the
/// Applications question, the bulk row and the step that installs them cannot disagree.
pub const OPTIONAL_APP_ANSWERS: [&str; 6] = crate::apps::Apps::IDS;

fn answer_is_true(answers: &Answers, name: &str) -> bool {
    answers.get(name).map(String::as_str) == Some("true")
}

/// Which steps the plan leaves out. The bodies ask the same questions of the environment, because
/// a step run on its own (`step <id>`) never passes through here, and that second check is the
/// safety net rather than a second policy: both read the constants above.
pub fn is_skipped(step: &Step, answers: &Answers) -> bool {
    match step.id {
        "00a-system-update" => answer_is_true(answers, SKIP_SYSTEM_UPDATE),
        "05-login-screen" => !answer_is_true(answers, SYNC_LOGIN_SCREEN),
        "11-optional-apps" => !OPTIONAL_APP_ANSWERS
            .iter()
            .any(|answer| answer_is_true(answers, answer)),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    fn answers(pairs: &[(&'static str, &str)]) -> Answers {
        pairs
            .iter()
            .map(|(key, value)| (*key, value.to_string()))
            .collect()
    }

    #[test]
    fn step_ids_are_unique() {
        let mut ids: Vec<&str> = STEPS.iter().map(|step| step.id).collect();
        ids.sort_unstable();
        let count = ids.len();
        ids.dedup();
        assert_eq!(ids.len(), count, "duplicate step ids");
    }

    #[test]
    fn every_step_the_policy_names_really_exists() {
        let named = ["00a-system-update", "05-login-screen", "11-optional-apps"];
        for id in named {
            assert!(
                STEPS.iter().any(|step| step.id == id),
                "the skip policy names {id}, which is not a step"
            );
        }

        let off = answers(&[
            ("SKIP_SYSTEM_UPDATE", "true"),
            ("SYNC_LOGIN_SCREEN", "false"),
        ]);
        let skipped: Vec<&str> = STEPS
            .iter()
            .filter(|step| is_skipped(step, &off))
            .map(|step| step.id)
            .collect();
        assert!(skipped.contains(&"00a-system-update"), "{skipped:?}");
        assert!(skipped.contains(&"05-login-screen"), "{skipped:?}");
        assert!(
            skipped.contains(&"11-optional-apps"),
            "no optional app is on, so that step has nothing to do: {skipped:?}"
        );

        let on = answers(&[
            ("SKIP_SYSTEM_UPDATE", "false"),
            ("SYNC_LOGIN_SCREEN", "true"),
            ("INSTALL_VSCODE", "false"),
            ("INSTALL_ZED", "false"),
            ("INSTALL_SPICETIFY", "false"),
            ("INSTALL_DISCORD", "false"),
            ("INSTALL_TODOIST", "false"),
            ("INSTALL_FIREFOX_THEME", "false"),
        ]);
        let skipped: Vec<&str> = STEPS
            .iter()
            .filter(|step| is_skipped(step, &on))
            .map(|step| step.id)
            .collect();
        assert_eq!(skipped, vec!["11-optional-apps"], "{skipped:?}");
    }

    #[test]
    fn step_names_are_unique() {
        let mut names: Vec<&str> = STEPS.iter().map(|step| step.name).collect();
        names.sort_unstable();
        let count = names.len();
        names.dedup();
        assert_eq!(names.len(), count, "duplicate step names");
    }

    #[test]
    fn every_phase_has_at_least_one_step() {
        for phase in Phase::ALL {
            assert!(
                STEPS.iter().any(|step| step.phase == phase),
                "no step in phase {}",
                phase.id()
            );
        }
    }

    #[test]
    fn the_backup_runs_after_the_submodule_update_on_purpose() {
        let position = |name: &str| STEPS.iter().position(|step| step.name == name).unwrap();
        assert!(position("Back up current setup") > position("Fetch the shell sources"));
    }

    #[test]
    fn the_system_update_is_skipped_only_when_asked() {
        let step = STEPS
            .iter()
            .find(|step| step.name == "Update system")
            .unwrap();
        assert!(!is_skipped(step, &BTreeMap::new()));
        assert!(is_skipped(
            step,
            &answers(&[("SKIP_SYSTEM_UPDATE", "true")])
        ));
        assert!(!is_skipped(
            step,
            &answers(&[("SKIP_SYSTEM_UPDATE", "false")])
        ));
    }

    #[test]
    fn the_login_screen_is_matched_only_when_it_is_asked_for() {
        let step = STEPS
            .iter()
            .find(|step| step.name == "Match the login screen")
            .unwrap();
        assert!(is_skipped(step, &BTreeMap::new()));
        assert!(!is_skipped(
            step,
            &answers(&[("SYNC_LOGIN_SCREEN", "true")])
        ));
    }

    #[test]
    fn optional_components_run_only_when_one_app_is_asked_for() {
        let step = STEPS
            .iter()
            .find(|step| step.name == "Install the apps you picked")
            .unwrap();
        assert!(is_skipped(step, &BTreeMap::new()));
        for answer in OPTIONAL_APP_ANSWERS {
            assert!(
                !is_skipped(step, &answers(&[(answer, "true")])),
                "{answer} should enable the optional components step"
            );
        }
    }

    #[test]
    fn no_other_step_is_ever_skipped() {
        let all_on: Answers = OPTIONAL_APP_ANSWERS
            .iter()
            .map(|answer| (*answer, "true".to_string()))
            .chain([
                ("SKIP_SYSTEM_UPDATE", "false".to_string()),
                ("SYNC_LOGIN_SCREEN", "true".to_string()),
            ])
            .collect();
        for step in STEPS.iter() {
            assert!(!is_skipped(step, &all_on), "{} was skipped", step.name);
        }
    }
}
