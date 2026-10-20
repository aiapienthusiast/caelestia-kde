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

    /// The stable spelling of a phase, for anything machine-read: `--plan --json` prints it.
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
        matches!(
            self,
            Status::Ok | Status::Warn | Status::Skipped | Status::Ignored
        )
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
    /// Run by the user's half of a packaged install, where the system has already been set up by
    /// the package manager. `caelestia packaged-install` asks for this list rather than keeping one.
    pub packaged: bool,
    pub run: Act,
}

pub struct Context {
    pub bundle: PathBuf,
    /// The answers this run was given, typed. A step asks for the one it needs instead of reading
    /// the process environment, so the plan's view of an answer and the step's cannot differ.
    answers: Answers,
}

impl Context {
    pub fn new(bundle: PathBuf, answers: Answers) -> Self {
        Self { bundle, answers }
    }

    /// The context a step run on its own is given: the answers arrive in its environment, and this
    /// is the one place they are read back out of it.
    pub fn from_environment(bundle: PathBuf) -> Self {
        let mut answers = Answers::new();
        for id in crate::menu::answer_ids() {
            if let Ok(value) = std::env::var(id) {
                answers.insert(id, value);
            }
        }
        Self { bundle, answers }
    }

    pub fn answer(&self, name: &str) -> Option<&str> {
        self.answers.get(name).map(String::as_str)
    }

    /// A switch, or `None` when the run said nothing about it.
    pub fn bool_of(&self, name: &str) -> Option<bool> {
        match self.answer(name) {
            Some("true") => Some(true),
            Some("false") => Some(false),
            _ => None,
        }
    }

    pub fn choice_of(&self, name: &str) -> Option<&str> {
        self.answer(name)
    }

    /// A switch, as the run answered it.
    pub fn flag(&self, name: &str, fallback: bool) -> bool {
        self.bool_of(name).unwrap_or(fallback)
    }

    /// A step-local switch that is not one of the install's answers, such as the ones a step offers
    /// so it can be re-run by hand. It can only come from the environment.
    pub fn overridden(&self, name: &str, fallback: bool) -> bool {
        std::env::var(name)
            .map(|value| value == "true")
            .unwrap_or(fallback)
    }

    /// An operational variable that is not an answer: what to build with, where to write, which
    /// plan this run belongs to. Ambient state, read from the environment because that is what it
    /// is.
    pub fn variable(&self, name: &str) -> Option<String> {
        std::env::var(name).ok().filter(|value| !value.is_empty())
    }
}

pub const STEPS: [Step; 17] = [
    Step {
        name: "Update system",
        id: "00a-system-update",
        phase: Phase::Prepare,
        packaged: false,
        run: system_update::run,
    },
    Step {
        name: "Check for required tools",
        id: "01-ensure-prereqs",
        phase: Phase::Prepare,
        packaged: false,
        run: prereqs::run,
    },
    Step {
        name: "Fetch the shell sources",
        id: "02a-submodules",
        phase: Phase::Prepare,
        packaged: false,
        run: submodules::run,
    },
    Step {
        name: "Back up current setup",
        id: "00-backup-themes",
        phase: Phase::Prepare,
        packaged: false,
        run: backup::run,
    },
    Step {
        name: "Install packages",
        id: "02-all-packages",
        phase: Phase::Packages,
        packaged: false,
        run: packages::all,
    },
    Step {
        name: "Set up the theming tools",
        id: "02-packages",
        phase: Phase::Packages,
        packaged: false,
        run: packages::tooling,
    },
    Step {
        name: "Deploy config files",
        id: "03-deploy-configs",
        phase: Phase::Configure,
        packaged: true,
        run: configs::run,
    },
    Step {
        name: "Download wallpapers",
        id: "03a-wallpapers",
        phase: Phase::Configure,
        packaged: true,
        run: wallpapers::run,
    },
    Step {
        name: "Apply KDE theme",
        id: "04-deploy-kde",
        phase: Phase::Configure,
        packaged: true,
        run: kde_theme::run,
    },
    Step {
        name: "Apply window rules",
        id: "04a-window-rules",
        phase: Phase::Configure,
        packaged: true,
        run: window_rules::run,
    },
    Step {
        name: "Match the login screen",
        id: "05-login-screen",
        phase: Phase::Configure,
        packaged: true,
        run: login_screen::run,
    },
    Step {
        name: "Enable system services",
        id: "06-services",
        phase: Phase::Configure,
        packaged: true,
        run: services::run,
    },
    Step {
        name: "Configure KDE applications",
        id: "07-kde-apps",
        phase: Phase::Configure,
        packaged: false,
        run: kde_apps::run,
    },
    Step {
        name: "Build Caelestia shell",
        id: "08-build-shell",
        phase: Phase::Build,
        packaged: true,
        run: build_shell::run,
    },
    Step {
        name: "Apply system tweaks",
        id: "09-system-tweaks",
        phase: Phase::Finalize,
        packaged: true,
        run: system_tweaks::run,
    },
    Step {
        name: "Create autostart entries",
        id: "10-autostart",
        phase: Phase::Finalize,
        packaged: true,
        run: autostart::run,
    },
    Step {
        name: "Install the apps you picked",
        id: "11-optional-apps",
        phase: Phase::Finalize,
        packaged: false,
        run: optional_apps::run,
    },
];

pub fn find(id: &str) -> Option<&'static Step> {
    STEPS.iter().find(|step| step.id == id)
}

/// What every step is told about the run it belongs to, whoever started it. The TUI and the
/// headless path both come through here, because the day they disagree is the day a step behaves
/// one way on a desktop and another on a server.
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

    let count = crate::kde::read("kwinrc", &["Desktops"], "Number").unwrap_or_default();

    if !count.is_empty() && count != "1" {
        output::skip(&format!("Keeping the existing {count} virtual desktops."));
        return Ok(());
    }

    output::info("Configuring 5 virtual desktops...");

    let mut written = true;
    for (key, value) in [("Number", "5"), ("Rows", "1")] {
        written &= crate::kde::write("kwinrc", &["Desktops"], key, value);
    }
    for index in 1..=5 {
        written &= crate::kde::write(
            "kwinrc",
            &["Desktops"],
            &format!("Name_{index}"),
            &format!("Desktop {index}"),
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

/// Which of the table's steps a caller is asking about. The table is the list; these are the two
/// cuts of it something else needs, so no launcher has to keep its own copy.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Purpose {
    /// Everything an install runs, in the order it runs them.
    Install,
    /// The steps a packaged install still has to do for the user.
    Packaged,
    /// The repairs an update re-runs.
    Update,
}

impl Purpose {
    pub fn parse(text: &str) -> Option<Self> {
        match text {
            "install" => Some(Purpose::Install),
            "packaged" => Some(Purpose::Packaged),
            "update" => Some(Purpose::Update),
            _ => None,
        }
    }
}

/// The steps a step runs before its own body when it is asked to run on its own, because they are
/// what it needs already in place. The table is the single answer: the update's plan asks for them
/// too, so a step never runs another one that the plan did not list.
pub fn prerequisites(step_id: &str) -> &'static [&'static str] {
    match step_id {
        "08-build-shell" => &[
            "02a-submodules",
            "02-all-packages",
            "06-services",
            "10-autostart",
            "03a-wallpapers",
        ],
        _ => &[],
    }
}

pub fn ids(purpose: Purpose) -> Vec<&'static str> {
    match purpose {
        Purpose::Install => STEPS.iter().map(|step| step.id).collect(),
        Purpose::Packaged => STEPS
            .iter()
            .filter(|step| step.packaged)
            .map(|step| step.id)
            .collect(),
        // An update re-runs its repairs, and each repair's prerequisites come first and are named.
        Purpose::Update => crate::update::step_ids()
            .into_iter()
            .flat_map(|id| prerequisites(id).iter().copied().chain(std::iter::once(id)))
            .collect(),
    }
}

/// The one switch a step hangs on: the answer it watches, and the value that makes it stand down.
/// The plan and the step's own body read the same table, so a step the plan marks "switched off" is
/// the same step that does nothing when run on its own.
enum StandDown {
    /// An opt-out: the step stands down when this answer is true.
    WhenTrue(&'static str),
    /// An opt-in: the step stands down unless this answer is true.
    WhenFalse(&'static str),
}

fn switch(step_id: &str) -> Option<StandDown> {
    match step_id {
        "00a-system-update" => Some(StandDown::WhenTrue(SKIP_SYSTEM_UPDATE)),
        "05-login-screen" => Some(StandDown::WhenFalse(SYNC_LOGIN_SCREEN)),
        _ => None,
    }
}

impl StandDown {
    fn met_by(&self, asked: impl Fn(&'static str) -> Option<bool>) -> bool {
        match self {
            StandDown::WhenTrue(name) => asked(name) == Some(true),
            StandDown::WhenFalse(name) => asked(name) != Some(true),
        }
    }
}

/// One policy, two sources of answers: the wizard has every answer in hand, a step run on its own
/// has only the ones it was given.
fn leaves_out(step_id: &str, asked: impl Fn(&'static str) -> Option<bool>) -> bool {
    if step_id == "11-optional-apps" {
        return !OPTIONAL_APP_ANSWERS
            .iter()
            .any(|answer| asked(answer) == Some(true));
    }
    switch(step_id).is_some_and(|stand_down| stand_down.met_by(asked))
}

/// Which steps the plan leaves out, from the answers the wizard collected.
pub fn is_skipped(step: &Step, answers: &Answers) -> bool {
    leaves_out(step.id, |name| {
        answers.get(name).map(|value| value == "true")
    })
}

/// Whether a step was told to stand down, from the answers its context carries.
pub fn switches_off(step_id: &str, context: &Context) -> bool {
    leaves_out(step_id, |name| context.bool_of(name))
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
    fn the_whole_table_is_what_an_install_runs_in_order() {
        assert_eq!(
            ids(Purpose::Install),
            STEPS.iter().map(|step| step.id).collect::<Vec<_>>()
        );
        assert_eq!(ids(Purpose::Install).len(), STEPS.len());
    }

    #[test]
    fn a_packaged_install_runs_the_user_half_and_nothing_system_wide() {
        // This list used to live in `src/bin/caelestia`; it is the table's flag now, so a step
        // added to the install cannot be forgotten by the packaged path.
        assert_eq!(
            ids(Purpose::Packaged),
            [
                "03-deploy-configs",
                "03a-wallpapers",
                "04-deploy-kde",
                "04a-window-rules",
                "05-login-screen",
                "06-services",
                "08-build-shell",
                "09-system-tweaks",
                "10-autostart",
            ]
        );
    }

    #[test]
    fn an_update_runs_its_own_repairs_out_of_the_same_table() {
        let update = ids(Purpose::Update);
        assert_eq!(
            update,
            [
                "03-deploy-configs",
                "02a-submodules",
                "02-all-packages",
                "06-services",
                "10-autostart",
                "03a-wallpapers",
                "08-build-shell",
                "09-system-tweaks",
            ]
        );
        for id in update {
            assert!(find(id).is_some(), "{id} is not in the step table");
        }
    }

    #[test]
    fn an_update_names_every_step_the_shell_run_would_have_run_itself() {
        let update = ids(Purpose::Update);
        for id in prerequisites("08-build-shell") {
            assert!(update.contains(id), "the update plan leaves out {id}");
        }
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
