use std::io::IsTerminal;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::layout::Rect;

use crate::env::Env;
use crate::exit::Outcome;
use crate::glyphs::Glyphs;
use crate::logline::LogLine;
use crate::menu::{self, Answers, Entry};
use crate::privileges::{self, Credential, Inhibitor};
use crate::signals;
use crate::steps::{self, STEPS, Status};
use crate::theme::Theme;
use crate::ui;

mod keys;

/// How many lines of the failing step's output the failure card shows.
const FAILURE_TAIL: usize = 6;

pub fn log_path() -> PathBuf {
    std::env::var_os("CAELESTIA_INSTALL_LOG")
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::temp_dir().join("caelestia-install.log"))
}

/// What the wizard is here to do. Update and Uninstall leave the TUI: their own prompts would
/// otherwise fight the TUI for the terminal.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Action {
    Install,
    Update,
    Uninstall,
}

impl Action {
    pub const ALL: [Action; 3] = [Action::Install, Action::Update, Action::Uninstall];

    pub fn title(self) -> &'static str {
        match self {
            Action::Install => "Install Caelestia",
            Action::Update => "Update this install",
            Action::Uninstall => "Uninstall Caelestia",
        }
    }

    pub fn header(self) -> &'static str {
        match self {
            Action::Install => "install",
            Action::Update => "update",
            Action::Uninstall => "uninstall",
        }
    }

    /// Why an action is not on offer here, said as the way out rather than as the fact the row
    /// above already carries. Install is always on offer - a reinstall is a thing people do - so
    /// that arm exists to keep the match total rather than to be shown.
    pub fn unavailable(self) -> &'static str {
        match self {
            Action::Update | Action::Uninstall => "Install Caelestia first",
            Action::Install => "Caelestia is already installed",
        }
    }

    fn handoff(self) -> Option<Handoff> {
        match self {
            Action::Install => None,
            Action::Update => Some(Handoff::Update),
            Action::Uninstall => Some(Handoff::Uninstall),
        }
    }
}

/// Where the wizard has got to, in the order it works through. Overlays are not stages: closing
/// one returns to the stage it was opened over.
/// What the confirm overlay is asking about.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Stage {
    Home,
    Detect,
    Options,
    Auth,
    Privilege,
    Run,
    Done,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Overlay {
    Keys,
    Log,
    Failure,
    Confirm,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Confirm {
    Handoff(Action),
    Install,
    Quit,
    LogOut,
}

impl Confirm {
    /// The buttons, the first being the one that does the thing.
    pub fn buttons(self) -> &'static [&'static str] {
        match self {
            Confirm::Handoff(Action::Update) => &["Update now", "Cancel"],
            Confirm::Handoff(Action::Uninstall) => &["Uninstall now", "Cancel"],
            Confirm::Handoff(Action::Install) | Confirm::Install => &["Install", "Cancel"],
            Confirm::Quit => &["Stop", "Keep going"],
            Confirm::LogOut => &["Log out", "Not now"],
        }
    }

    /// Where the highlight starts. Two of these questions end something a keystroke cannot get
    /// back — the run, or the session — so their safe answer is the one under the highlight.
    pub fn focus(self) -> usize {
        match self {
            Confirm::Quit | Confirm::LogOut => 1,
            _ => 0,
        }
    }

    /// The button a letter stands for, in the order they are drawn.
    pub fn shortcut(self, letter: char) -> Option<usize> {
        match letter {
            'y' | 'Y' => Some(0),
            'n' | 'N' => Some(self.buttons().len() - 1),
            _ => None,
        }
    }
}

/// How the wizard will become root: its own helper can ask, or the reader can answer.
/// The actions that hand the terminal over to something else: another program of the installer's,
/// or the end of the session itself.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AuthMethod {
    Fingerprint,
    Password,
}

impl AuthMethod {
    pub fn title(self) -> &'static str {
        match self {
            AuthMethod::Fingerprint => "Touch the fingerprint reader",
            AuthMethod::Password => "Type your password here",
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Handoff {
    Update,
    Uninstall,
    LogOut,
}

pub struct Ending {
    pub outcome: Outcome,
    pub handoff: Option<Handoff>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Detection {
    pub distro: crate::distro::Distro,
    pub kind: crate::install_kind::Kind,
    pub installed: bool,
    pub package_manager: Option<&'static str>,
    pub quickshell: bool,
    pub git: bool,
}

impl Detection {
    pub fn read() -> Self {
        let distro = crate::distro::detect();
        let package_manager = match distro {
            crate::distro::Distro::Arch => Some("pacman"),
            crate::distro::Distro::Fedora => Some("dnf"),
            crate::distro::Distro::Debian => Some("apt"),
            crate::distro::Distro::Unknown => None,
        }
        .filter(|name| crate::process::available(name));
        Self {
            distro,
            kind: crate::install_kind::kind(),
            installed: crate::install_kind::shell_config().exists(),
            package_manager,
            quickshell: crate::process::available("quickshell"),
            git: crate::process::available("git"),
        }
    }

    /// What to call the distribution and the layout on screen.
    pub fn distro_label(&self) -> &'static str {
        self.distro.label()
    }

    pub fn kind_label(&self) -> &'static str {
        self.kind.label()
    }

    /// The things that will stop an install, as opposed to the things that are merely true.
    /// Each one is the condition its own sentence describes, so a blocker never fires on a
    /// machine where what it says is missing is not needed - and a distribution that is unknown
    /// is said once, rather than also blaming the package manager it was never going to find.
    pub fn blockers(&self) -> Vec<&'static str> {
        let mut blockers = Vec::new();
        if self.distro == crate::distro::Distro::Unknown {
            blockers.push("This distribution is not one the installer knows how to install on.");
            return blockers;
        }
        if self.package_manager.is_none() {
            blockers.push("No supported package manager was found on PATH.");
        }
        if !self.git && self.kind == crate::install_kind::Kind::Source {
            blockers.push("git is missing, and a source install needs it.");
        }
        blockers
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Flow {
    Continue,
    Done(Outcome),
}

pub struct Choice {
    pub key: char,
    pub label: &'static str,
    pub help: &'static str,
}

/// What went wrong, what can be done about it, and what each answer costs.
pub struct FailureCard {
    pub step: &'static str,
    pub detail: Vec<LogLine>,
    pub choice: usize,
    id: &'static str,
}

impl FailureCard {
    pub const CHOICES: [Choice; 4] = [
        Choice {
            key: 'r',
            label: "Retry",
            help: "",
        },
        Choice {
            key: 's',
            label: "Skip",
            help: "record it as switched off, and continue",
        },
        Choice {
            key: 'i',
            label: "Ignore",
            help: "continue, and report the install as failed",
        },
        Choice {
            key: 'q',
            label: "Stop",
            help: "",
        },
    ];

    pub fn new(step: &'static steps::Step, log: &[LogLine]) -> Self {
        let detail = log.iter().rev().take(FAILURE_TAIL).rev().cloned().collect();
        Self {
            step: step.name,
            id: step.id,
            detail,
            choice: 0,
        }
    }

    /// The command that reproduces the step outside the TUI, so a failure can be worked on
    /// somewhere there is room to read it.
    pub fn reproduction(&self) -> String {
        format!("caelestia-install step {}", self.id)
    }
}

pub struct App {
    pub bundle: PathBuf,
    pub log_path: PathBuf,
    pub env: Env,
    pub theme: Theme,
    pub glyphs: Glyphs,
    /// Which of the themeable apps are on this machine. Read once, and the reason a question can
    /// ask about the apps somebody has instead of listing the ones they might install.
    pub apps: crate::apps::Apps,
    pub fingerprint: bool,
    pub needs_password: bool,
    pub detection: Detection,
    pub reinstall_acknowledged: bool,
    pub installed_when_started: bool,
    pub unfinished: Vec<(String, Status)>,
    pub stage: Stage,
    pub overlay: Option<Overlay>,
    pub confirm: Option<Confirm>,
    pub action: Action,
    pub home: usize,
    pub answers: Answers,
    baseline: Answers,
    pub question: usize,
    pub cursor: usize,
    pub auth: Option<AuthMethod>,
    pub password: String,
    pub(crate) checking: Option<std::sync::mpsc::Receiver<bool>>,
    /// A rehearsal walks the table without running anything. Only a test sets this.
    #[cfg(test)]
    pub pretend: bool,
    pub attempts: usize,
    pub fingerprint_spent: bool,
    pub problem: Option<String>,
    pub notice: Option<String>,
    pub failure: Option<FailureCard>,
    run: crate::run::Session,
    pub log: Vec<LogLine>,
    pub log_follow: bool,
    pub log_scroll: usize,
    pub log_loud_only: bool,
    /// Which of the lines that went wrong `n` and `p` are standing on, counted as a row of the log
    /// as it is shown. Cleared by any other way of moving through the log, so the next jump starts
    /// from what the pane is showing.
    pub log_problem: Option<usize>,
    pub confirm_focus: usize,
    pub area: Rect,
    pub spinner: usize,
    leaving: Option<Outcome>,
    handoff: Option<Handoff>,
    credential: Option<Credential>,
    inhibitor: Option<Inhibitor>,
    /// Set when something may have drawn on the screen besides the wizard, so the next frame is
    /// painted whole instead of diffed against a record that no longer matches.
    pub repaint: bool,
    pub landed: Option<Landed>,
}

pub struct Landed {
    pub version: Option<String>,
    pub commit: Option<String>,
    pub branch: Option<String>,
    pub shell: PathBuf,
    pub configs: PathBuf,
    pub backup: Option<PathBuf>,
}

impl Landed {
    pub fn read(bundle: &Path) -> Self {
        let shell = crate::update::config_dir();
        let installed = crate::update_state::installed(&shell)
            .or_else(|| crate::update::published_version(bundle));
        let (version, commit, branch) = match installed {
            Some(installed) => (
                Some(installed.version),
                Some(installed.commit).filter(|commit| !commit.is_empty()),
                Some(installed.branch).filter(|branch| !branch.is_empty()),
            ),
            None => (None, None, None),
        };
        Self {
            version,
            commit,
            branch,
            shell,
            configs: crate::paths::home().join(".config").join("caelestia"),
            backup: crate::steps::recorded_backup_dir(bundle),
        }
    }
}

impl App {
    pub fn new(bundle: PathBuf, log_path: PathBuf, env: Env) -> Self {
        let answers = menu::starting(&[]);
        let baseline = answers.clone();
        let fingerprint = privileges::fingerprint_ready();
        let run = crate::run::Session::new(&bundle, log_path.clone(), Vec::new(), &answers);
        Self {
            bundle,
            log_path,
            theme: Theme::resolve(&env),
            glyphs: Glyphs::resolve(&env),
            apps: crate::apps::Apps::here(),
            fingerprint,
            needs_password: needs_password(),
            env,
            detection: Detection::read(),
            reinstall_acknowledged: false,
            installed_when_started: crate::install_kind::shell_config().exists(),
            unfinished: crate::journal::unfinished(),
            stage: Stage::Home,
            overlay: None,
            confirm: None,
            action: Action::Install,
            home: 0,
            answers,
            baseline,
            question: 0,
            cursor: 0,
            auth: None,
            password: String::new(),
            checking: None,
            #[cfg(test)]
            pretend: false,
            attempts: 0,
            fingerprint_spent: false,
            problem: None,
            notice: None,
            failure: None,
            run,
            log: Vec::new(),
            log_follow: true,
            log_scroll: 0,
            log_loud_only: false,
            log_problem: None,
            confirm_focus: 0,
            area: Rect::new(0, 0, 0, 0),
            spinner: 0,
            leaving: None,
            handoff: None,
            credential: None,
            inhibitor: None,
            repaint: false,
            landed: None,
        }
    }

    /// Point the wizard at a machine with a known set of apps, and take the answers it would have
    /// started with there. Which apps a machine has is read from the machine, so this is the only
    /// way to say what the app rows are about.
    #[cfg(test)]
    pub fn with_apps(mut self, apps: crate::apps::Apps) -> Self {
        let seeded: Vec<(&'static str, &'static str)> = apps.answers().collect();
        self.apps = apps;
        for (id, answer) in seeded {
            self.answers.insert(id, answer.to_string());
        }
        self.baseline = self.answers.clone();
        self
    }

    /// Point the wizard at a machine that has, or has not, a fingerprint to offer. The same reason
    /// the apps are settable: what the host happens to have must not decide what a test asserts.
    #[cfg(test)]
    pub fn with_fingerprint(mut self, ready: bool) -> Self {
        self.fingerprint = ready;
        self
    }

    #[cfg(test)]
    pub fn with_password_gate(mut self, needed: bool) -> Self {
        self.needs_password = needed;
        self
    }

    pub fn apply_settings(&mut self, settings: &[(&'static str, String)]) {
        menu::apply_set(settings, &mut self.answers);
        self.baseline = self.answers.clone();
    }

    pub fn header(&self) -> ui::Header {
        ui::Header {
            left: match self.stage {
                Stage::Home => "installer".to_string(),
                _ => self.action.header().to_string(),
            },
            right: format!(
                "{} · {} install",
                self.detection.distro_label(),
                self.detection.kind_label()
            ),
        }
    }

    pub fn counter(&self) -> Option<String> {
        match self.stage {
            Stage::Run => Some(format!("step {}/{}", self.finished(), STEPS.len())),
            Stage::Options => Some(format!(
                "question {}/{}",
                self.question + 1,
                menu::question_count()
            )),
            _ => None,
        }
    }

    pub fn finished(&self) -> usize {
        self.run
            .statuses()
            .iter()
            .filter(|status| !matches!(status, Status::Pending | Status::Running))
            .count()
    }

    pub fn statuses(&self) -> &[Status] {
        self.run.statuses()
    }

    #[cfg(test)]
    pub fn status(&self, index: usize) -> Status {
        self.run.status(index)
    }

    #[cfg(test)]
    pub fn set_status(&mut self, index: usize, status: Status) {
        self.run.set_status(index, status);
    }

    /// Mark every step the same, for a test that only needs the shape.
    #[cfg(test)]
    pub fn fill_statuses(&mut self, status: Status) {
        self.run.fill(status);
    }

    #[cfg(test)]
    pub fn current(&self) -> usize {
        self.run.cursor()
    }

    pub fn total_elapsed(&self) -> Duration {
        self.run.total_elapsed()
    }

    /// The step being worked on, as an index into the table.
    pub fn running_step(&self) -> Option<usize> {
        self.run
            .statuses()
            .iter()
            .position(|status| *status == Status::Running)
    }

    pub fn tally(&self) -> Tally {
        let count = |status: Status| {
            self.run
                .statuses()
                .iter()
                .filter(|each| **each == status)
                .count()
        };
        Tally {
            warn: count(Status::Warn),
            failed: count(Status::Failed),
            ignored: count(Status::Ignored),
        }
    }

    /// The rows of the question on screen. This is the list the cursor indexes, so what is drawn
    /// and what a key moves over cannot disagree.
    pub fn current_entries(&self) -> Vec<&'static Entry> {
        menu::items_of(self.question).iter().collect()
    }

    pub fn selected(&self) -> Option<&'static Entry> {
        self.current_entries().into_iter().nth(self.cursor)
    }

    /// Whether a row's answer differs from the one this machine started at.
    pub fn is_changed(&self, entry: &Entry) -> bool {
        menu::is_changed(entry, &self.answers, &self.baseline)
    }

    pub fn action_enabled(&self, action: Action) -> bool {
        match action {
            Action::Install => true,
            Action::Update | Action::Uninstall => self.installed(),
        }
    }

    pub fn installed(&self) -> bool {
        self.detection.installed
    }

    /// The run is stopped on a failure the user has not resolved yet. One fact, one field: the card
    /// that is still on record is what stops the run, so the two cannot disagree.
    pub fn paused(&self) -> bool {
        self.failure.is_some()
    }

    /// A rehearsal run walks the table without running anything. Only a test asks for one, so in
    /// production this is always false and the branches below it fold away.
    fn rehearsing(&self) -> bool {
        #[cfg(test)]
        {
            self.pretend
        }
        #[cfg(not(test))]
        {
            false
        }
    }

    pub fn refresh_detection(&mut self) {
        if self.rehearsing() {
            return;
        }
        self.detection = Detection::read();
        self.unfinished = crate::journal::unfinished();
        self.problem = None;
    }

    pub fn question_changed(&self, question: usize) -> bool {
        menu::items_of(question)
            .iter()
            .any(|entry| menu::holds_an_answer(entry) && self.is_changed(entry))
    }

    pub fn changed_answers(&self) -> usize {
        menu::every_entry()
            .into_iter()
            .filter(|entry| menu::holds_an_answer(entry) && self.is_changed(entry))
            .count()
    }

    /// The lines the log pane shows, in order: everything, or only what went wrong.
    pub fn visible_log(&self) -> impl Iterator<Item = &LogLine> + '_ {
        self.log
            .iter()
            .filter(|line| !self.log_loud_only || line.is_loud())
    }

    pub fn log_rows(&self) -> usize {
        ui::log_rows(self.run_layout().log).max(1) as usize
    }

    /// The rows of the log as it is shown that went wrong, in order. These are what `n` and `p`
    /// step through, what the heading counts, and what the scrollbar marks.
    pub fn problem_rows(&self) -> impl Iterator<Item = usize> + '_ {
        self.visible_log()
            .enumerate()
            .filter(|(_, line)| line.is_loud())
            .map(|(row, _)| row)
    }

    pub(super) fn jump_to_problem(&mut self, forward: bool, rows: usize) {
        let problems: Vec<usize> = self.problem_rows().collect();
        let (Some(&first), Some(&last)) = (problems.first(), problems.last()) else {
            self.notice = Some("nothing in the log went wrong".to_string());
            return;
        };
        let from = self.log_problem.unwrap_or_else(|| {
            if self.log_follow {
                usize::MAX
            } else {
                self.log_window_for(rows).0
            }
        });
        let target = if forward {
            problems
                .iter()
                .copied()
                .find(|row| *row > from)
                .unwrap_or(first)
        } else {
            problems
                .iter()
                .copied()
                .rev()
                .find(|row| *row < from)
                .unwrap_or(last)
        };
        let at = problems.iter().position(|row| *row == target).unwrap_or(0);

        self.log_follow = false;
        self.log_scroll = target.saturating_sub(rows / 3);
        self.scroll_log(0, rows);
        self.log_problem = Some(target);
        self.notice = Some(format!("problem {} of {}", at + 1, problems.len()));
    }

    pub fn run_layout(&self) -> ui::RunLayout {
        ui::run_layout(ui::body(self.area))
    }

    /// The first step that has not started: what the run screen names between two steps.
    pub fn next_step(&self) -> Option<usize> {
        self.run
            .statuses()
            .iter()
            .position(|status| *status == Status::Pending)
    }

    pub fn log_window_for(&self, rows: usize) -> (usize, usize) {
        let rows = rows.max(1);
        let total = self.visible_log().count();
        let start = if self.log_follow {
            total.saturating_sub(rows)
        } else {
            self.log_scroll.min(total.saturating_sub(rows))
        };
        (start, rows)
    }

    pub fn leaving(&self) -> Option<Outcome> {
        self.leaving
    }

    pub fn ending(&self, outcome: Outcome) -> Ending {
        Ending {
            outcome,
            handoff: self.handoff,
        }
    }

    pub(super) fn leave(&mut self, outcome: Outcome) {
        self.leaving = Some(outcome);
    }

    /// The signal handler's escape hatch, and `ctrl+c`: stop whatever is running, then leave.
    fn interrupt(&mut self) -> Flow {
        if self.overlay == Some(Overlay::Confirm) {
            return self.stop_and_leave();
        }
        if self.busy() {
            self.ask(Confirm::Quit);
            return Flow::Continue;
        }
        Flow::Done(Outcome::Interrupted)
    }

    /// Whether a step is in flight, which is the only thing that makes quitting worth asking
    /// about.
    fn busy(&self) -> bool {
        self.run.is_running() || self.running_step().is_some()
    }

    pub(super) fn stop_and_leave(&mut self) -> Flow {
        self.stop_running();
        Flow::Done(Outcome::Interrupted)
    }

    pub(super) fn stop_running(&mut self) {
        self.run.abandon();
    }

    pub fn tick(&mut self) {
        if signals::terminated() {
            self.stop_running();
            self.leave(Outcome::Interrupted);
            return;
        }
        self.spinner = self.spinner.wrapping_add(1);
        if self.stage == Stage::Privilege {
            self.check_password();
        }
        if self.stage == Stage::Run && self.run.is_running() {
            self.progress();
        }
    }

    pub fn key(&mut self, key: KeyEvent) -> Flow {
        self.notice = None;

        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            return self.interrupt();
        }
        if let Some(overlay) = self.overlay {
            return self.overlay_key(overlay, key);
        }
        if key.code == KeyCode::Char('t') && self.stage != Stage::Privilege {
            self.theme.cycle();
            self.notice = Some(format!("{} theme", self.theme.mode.name()));
            return Flow::Continue;
        }
        match self.stage {
            Stage::Home => self.home_key(key),
            Stage::Detect => self.detect_key(key),
            Stage::Options => self.options_key(key),
            Stage::Auth => self.auth_key(key),
            Stage::Privilege => self.privilege_key(key),
            Stage::Run => self.run_key(key),
            Stage::Done => self.done_key(key),
        }
    }

    pub(super) fn confirm_install(&mut self) -> Flow {
        self.refresh_detection();
        if self.detection.installed && !self.reinstall_acknowledged {
            self.problem = Some(if self.installed_when_started {
                "Caelestia is already installed on this machine.".to_string()
            } else {
                "Caelestia was installed while the wizard was open. Read this screen, then \
                 continue."
                    .to_string()
            });
            self.stage = Stage::Detect;
            return Flow::Continue;
        }
        if let Some(blocker) = self.detection.blockers().first() {
            self.problem = Some((*blocker).to_string());
            self.stage = Stage::Detect;
            return Flow::Continue;
        }
        self.need_auth();
        Flow::Continue
    }

    pub fn begin_install(&mut self) {
        if !self.rehearsing() && self.inhibitor.is_none() {
            self.inhibitor = Some(Inhibitor::acquire());
        }
        self.stage = Stage::Run;
        self.run.reset(self.step_environment(), &self.answers);
        #[cfg(test)]
        self.run.rehearse(self.pretend);
        let recorded = if self.rehearsing() {
            Ok(())
        } else {
            crate::update::save_answers(&self.answers)
        };
        if let Err(error) = recorded {
            self.notice = Some(format!(
                "could not record the answers for a later update: {error}"
            ));
        }
        self.log.clear();
        self.log_scroll = 0;
        self.log_follow = true;
        self.log_problem = None;
        self.failure = None;
        self.start_next();
    }

    /// Try the failed steps again, without redoing what worked.
    pub fn retry_failed(&mut self) {
        let failed: Vec<usize> = self
            .run
            .statuses()
            .iter()
            .enumerate()
            .filter(|(_, status)| matches!(status, Status::Failed | Status::Ignored))
            .map(|(index, _)| index)
            .collect();
        if failed.is_empty() {
            self.notice = Some("no step needs retrying".to_string());
            return;
        }
        for index in &failed {
            self.run.set_status(*index, Status::Pending);
        }
        self.run.cursor_to(failed[0]);
        self.stage = Stage::Run;
        self.failure = None;
        self.log_follow = true;
        self.log_problem = None;
        self.start_next();
    }

    pub fn step_environment(&self) -> Vec<(&'static str, String)> {
        let mut env = steps::environment(&self.answers, &self.bundle);
        if let Some(credential) = &self.credential {
            env.extend(credential.environment());
        }
        env
    }

    /// Ask the session for the next step, and act on whatever it reports.
    pub fn start_next(&mut self) {
        let events = self.run.advance();
        let finished = self.run.is_complete();
        self.apply(events);
        if finished && !self.paused() {
            self.stage = Stage::Done;
            self.landed = Some(Landed::read(&self.bundle));
            self.credential = None;
            self.inhibitor = None;
        }
    }

    fn progress(&mut self) {
        let events = self.run.pump();
        let settled = !self.run.is_running();
        self.apply(events);
        if settled && !self.paused() && self.stage == Stage::Run {
            self.start_next();
        }
    }

    /// Turn what the session did into what the wizard shows.
    fn apply(&mut self, events: Vec<crate::run::Event>) {
        for event in events {
            match event {
                crate::run::Event::Skipped { index, reason } => {
                    let name = self.run.steps()[index].name;
                    self.log.push(match reason {
                        crate::run::SkipReason::SwitchedOff => {
                            LogLine::skip(format!("{name} is switched off in the options"))
                        }
                        crate::run::SkipReason::AlreadyDone => {
                            LogLine::skip(format!("{name} was already done"))
                        }
                    });
                }
                crate::run::Event::Started { .. } => self.repaint = true,
                crate::run::Event::Output { line, .. } => self.log.push(LogLine::parse(&line)),
                crate::run::Event::Finished { index, status } => {
                    self.repaint = true;
                    if status.is_failure() {
                        self.failure = Some(FailureCard::new(&self.run.steps()[index], &self.log));
                        return;
                    }
                    if self.rehearsing() {
                        self.log.push(LogLine::ok(format!(
                            "{} finished",
                            self.run.steps()[index].name
                        )));
                    }
                }
                crate::run::Event::Broken { index, problem } => {
                    self.repaint = true;
                    self.log.push(LogLine::error(problem));
                    self.failure = Some(FailureCard::new(&self.run.steps()[index], &self.log));
                    return;
                }
            }
        }
    }
}

/// Counted once, so the screen and the exit code cannot disagree. Only what the screens ask for:
/// an install that went through is described by what it left behind, not by how many steps it
/// took to get there.
pub struct Tally {
    pub warn: usize,
    pub failed: usize,
    pub ignored: usize,
}

impl Tally {
    /// Steps that did not do what they were asked to do.
    pub fn unresolved(&self) -> usize {
        self.failed + self.ignored
    }
}

fn needs_password() -> bool {
    !privileges::is_root() && !privileges::sudo_cached()
}

pub fn run(
    bundle: PathBuf,
    log_path: PathBuf,
    env: Env,
    settings: &[(&'static str, String)],
) -> Ending {
    if !std::io::stdin().is_terminal() || !std::io::stdout().is_terminal() {
        eprintln!("caelestia-install: the installer needs a terminal to run");
        return Ending {
            outcome: Outcome::Environment,
            handoff: None,
        };
    }
    signals::install();
    let mut terminal = match crate::tui::Tui::start(&log_path) {
        Ok(terminal) => terminal,
        Err(error) => {
            eprintln!("caelestia-install: cannot open a terminal: {error}");
            return Ending {
                outcome: Outcome::Environment,
                handoff: None,
            };
        }
    };
    let mut app = App::new(bundle, log_path, env);
    app.apply_settings(settings);
    let outcome = event_loop(&mut terminal, &mut app);
    terminal.finish();
    app.ending(outcome)
}

fn event_loop(terminal: &mut crate::tui::Tui, app: &mut App) -> Outcome {
    loop {
        if let Some(outcome) = app.leaving() {
            return outcome;
        }
        if std::mem::take(&mut app.repaint) {
            terminal.repaint();
        }
        if terminal
            .draw(|frame| crate::screens::draw(frame, app))
            .is_err()
        {
            return Outcome::Environment;
        }
        app.tick();
        if event::poll(Duration::from_millis(120)).unwrap_or(false) {
            match event::read() {
                Ok(Event::Key(key)) if key.kind == KeyEventKind::Press => {
                    if let Flow::Done(outcome) = app.key(key) {
                        return outcome;
                    }
                }
                Ok(_) => {}
                Err(_) => return Outcome::Environment,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::menu;

    fn app() -> App {
        let mut app = App::new(
            PathBuf::from("/tmp/caelestia"),
            PathBuf::from("/tmp/install.log"),
            Env::default(),
        )
        .with_apps(crate::apps::Apps::all())
        .with_fingerprint(false)
        .with_password_gate(true);
        app.unfinished.clear();
        app.area = Rect::new(0, 0, 100, 30);
        app.pretend = true;
        app
    }

    fn press(app: &mut App, code: KeyCode) -> Flow {
        app.key(KeyEvent::new(code, KeyModifiers::NONE))
    }

    fn char_key(app: &mut App, character: char) -> Flow {
        press(app, KeyCode::Char(character))
    }

    /// Which question a row lives in, by the row's id.
    fn question_of(id: &str) -> usize {
        (0..menu::question_count())
            .find(|index| menu::items_of(*index).iter().any(|entry| entry.id() == id))
            .unwrap_or_else(|| panic!("{id} is not in a question"))
    }

    fn installable() -> Detection {
        Detection {
            distro: crate::distro::Distro::Arch,
            kind: crate::install_kind::Kind::Source,
            installed: false,
            package_manager: Some("pacman"),
            quickshell: true,
            git: true,
        }
    }

    /// Everything the run screen shows, as a state without any of the work.
    fn run_with(statuses: &[Status]) -> App {
        let mut app = app();
        app.stage = Stage::Run;
        for (index, status) in statuses.iter().enumerate() {
            app.set_status(index, *status);
        }
        app
    }

    #[test]
    fn home_offers_the_three_actions_and_moves_only_between_the_ones_that_run() {
        let mut nothing_installed = app();
        nothing_installed.detection = installable();
        assert_eq!(nothing_installed.stage, Stage::Home);
        assert_eq!(Action::ALL[nothing_installed.home], Action::Install);
        press(&mut nothing_installed, KeyCode::Down);
        assert_eq!(
            Action::ALL[nothing_installed.home],
            Action::Install,
            "the only reachable action"
        );
        press(&mut nothing_installed, KeyCode::Up);
        assert_eq!(Action::ALL[nothing_installed.home], Action::Install);

        let mut already_there = app();
        already_there.detection = Detection {
            installed: true,
            ..installable()
        };
        already_there.home = 1;
        let mut seen = Vec::new();
        for _ in 0..3 {
            seen.push(Action::ALL[already_there.home]);
            press(&mut already_there, KeyCode::Down);
        }
        assert_eq!(
            seen,
            vec![Action::Update, Action::Uninstall, Action::Install]
        );
        assert_eq!(
            Action::ALL[already_there.home],
            Action::Update,
            "and it wraps"
        );
    }

    #[test]
    fn choosing_install_reports_this_machine_before_anything_else() {
        let mut app = app();
        app.detection = installable();
        press(&mut app, KeyCode::Enter);
        assert_eq!(
            app.stage,
            Stage::Detect,
            "what is on the machine is read before anything is decided"
        );

        press(&mut app, KeyCode::Enter);
        assert_eq!(app.stage, Stage::Options, "and then the questions start");
        assert_eq!(app.question, 0);
        assert_eq!(app.cursor, 0);
    }

    #[test]
    fn a_machine_that_cannot_be_installed_on_does_not_start_the_questions() {
        let mut app = app();
        app.detection = Detection {
            package_manager: None,
            ..installable()
        };
        press(&mut app, KeyCode::Enter);
        assert_eq!(app.stage, Stage::Detect);

        press(&mut app, KeyCode::Enter);
        assert_eq!(app.stage, Stage::Detect, "it stays where the reason is");
        assert_eq!(
            app.problem.as_deref(),
            Some("Sort out the above first."),
            "the answer to the key, in words the blocker above does not already use"
        );

        press(&mut app, KeyCode::Esc);
        assert_eq!(app.stage, Stage::Home);
        assert_eq!(app.problem, None, "the screen's message goes with it");
    }

    #[test]
    fn a_bulk_choice_counts_as_the_answers_it_sets_and_not_as_itself() {
        let mut app = app();
        app.stage = Stage::Options;
        app.question = question_of(menu::PRESET_THEME_APPS);
        app.cursor = 0;
        assert_eq!(app.changed_answers(), 0);

        char_key(&mut app, ' ');
        assert_eq!(
            app.changed_answers(),
            6,
            "six apps, not seven options: the row is not an answer"
        );

        char_key(&mut app, ' ');
        assert_eq!(app.changed_answers(), 0);
    }

    #[test]
    fn every_route_into_the_install_arrives_at_the_same_place() {
        let mut app = app();
        app.detection = installable();
        app.stage = Stage::Privilege;
        app.password = "hunter2".to_string();
        press(&mut app, KeyCode::Enter);
        app.tick();
        assert!(
            matches!(app.stage, Stage::Run | Stage::Done),
            "the password route starts the install like any other, got {:?}",
            app.stage
        );
    }

    #[test]
    fn an_update_is_not_offered_where_there_is_nothing_to_update() {
        let mut app = app();
        app.detection = installable();
        app.home = 1;
        assert_eq!(Action::ALL[app.home], Action::Update);

        press(&mut app, KeyCode::Enter);
        assert_eq!(app.stage, Stage::Home, "nothing starts");
        assert_eq!(app.overlay, None, "and nothing is offered to confirm");
        assert_eq!(
            app.notice.as_deref(),
            Some(Action::unavailable(Action::Update)),
            "the answer to the key is what to do about it"
        );

        app.detection.installed = true;
        press(&mut app, KeyCode::Enter);
        assert_eq!(app.overlay, Some(Overlay::Confirm));
        assert_eq!(app.confirm, Some(Confirm::Handoff(Action::Update)));
    }

    #[test]
    fn the_theme_key_works_on_every_screen_but_the_password_one() {
        for stage in [
            Stage::Home,
            Stage::Detect,
            Stage::Options,
            Stage::Auth,
            Stage::Run,
            Stage::Done,
        ] {
            let mut app = app();
            app.stage = stage;
            let before = app.theme.mode;
            char_key(&mut app, 't');
            assert_ne!(app.theme.mode, before, "{stage:?} should flip the theme");
        }

        let mut app = app();
        app.stage = Stage::Privilege;
        let before = app.theme.mode;
        char_key(&mut app, 't');
        assert_eq!(app.theme.mode, before, "a password may contain a t");
        assert_eq!(app.password, "t", "and it is typed, not swallowed");
    }

    #[test]
    fn the_options_footer_keeps_the_way_on_and_loses_the_resets_first() {
        let mut app = app();
        app.stage = Stage::Options;
        let footer: Vec<&str> = app.keys().iter().map(|key| key.what).collect();
        let at = |needle: &str| {
            footer
                .iter()
                .position(|what| *what == needle)
                .unwrap_or_else(|| panic!("{needle} is missing from {footer:?}"))
        };
        assert!(at("continue") < at("reset"), "{footer:?}");
        assert!(at("back") < at("reset"), "{footer:?}");
        assert_eq!(
            footer.last(),
            Some(&"reset all"),
            "the least important hint is the one a narrow footer drops: {footer:?}"
        );
    }

    #[test]
    fn a_machine_that_already_has_caelestia_says_so_before_it_is_overwritten() {
        let mut app = app();
        app.detection = Detection {
            installed: true,
            ..installable()
        };
        app.detection.installed = true;
        app.installed_when_started = true;

        press(&mut app, KeyCode::Enter);
        assert_eq!(app.stage, Stage::Detect);
        assert!(
            app.detection.installed,
            "the screen has something to report"
        );
        assert!(
            !app.reinstall_acknowledged,
            "and nothing has been acknowledged yet"
        );

        press(&mut app, KeyCode::Enter);
        assert!(app.reinstall_acknowledged);
        assert_eq!(app.stage, Stage::Options);

        app.ask(Confirm::Install);
        char_key(&mut app, 'y');
        assert_ne!(app.stage, Stage::Detect);
    }

    #[test]
    fn an_install_that_appeared_while_the_wizard_was_open_stops_at_the_confirmation() {
        let mut app = app();
        app.detection = installable();
        app.installed_when_started = false;
        app.ask(Confirm::Install);

        app.detection.installed = true;
        char_key(&mut app, 'y');

        assert_eq!(
            app.stage,
            Stage::Detect,
            "the wizard goes back to what it found rather than carrying on"
        );
        assert!(
            app.problem
                .as_deref()
                .unwrap_or_default()
                .contains("while the wizard was open"),
            "{:?}",
            app.problem
        );
    }

    #[test]
    fn a_machine_the_installer_cannot_install_on_says_why_and_stops() {
        let mut app = app();
        app.detection = Detection {
            package_manager: None,
            ..installable()
        };
        app.ask(Confirm::Install);
        app.reinstall_acknowledged = true;

        char_key(&mut app, 'y');
        assert_eq!(app.stage, Stage::Detect, "it stops rather than starting");
        assert!(
            app.problem
                .as_deref()
                .unwrap_or_default()
                .contains("package manager"),
            "{:?}",
            app.problem
        );
    }

    #[test]
    fn the_detection_screen_offers_the_update_route_when_there_is_one() {
        let mut app = app();
        app.detection = Detection {
            installed: true,
            ..installable()
        };
        app.detection.installed = true;
        app.stage = Stage::Detect;

        char_key(&mut app, 'u');
        assert_eq!(app.action, Action::Update);
        assert_eq!(app.overlay, Some(Overlay::Confirm));
        assert_eq!(app.confirm, Some(Confirm::Handoff(Action::Update)));
    }

    #[test]
    fn a_machine_with_nothing_installed_is_not_offered_an_update() {
        let mut app = app();
        app.detection = installable();
        app.stage = Stage::Detect;

        char_key(&mut app, 'u');
        assert_eq!(
            app.stage,
            Stage::Detect,
            "nothing to update, nothing to offer"
        );
        assert_eq!(app.overlay, None);
    }

    #[test]
    fn detection_reports_every_precondition_the_install_depends_on() {
        assert!(installable().blockers().is_empty());

        let no_manager = Detection {
            package_manager: None,
            ..installable()
        };
        assert_eq!(no_manager.blockers().len(), 1);

        let no_git = Detection {
            git: false,
            ..installable()
        };
        assert!(
            no_git
                .blockers()
                .iter()
                .any(|blocker| blocker.contains("git")),
            "{:?}",
            no_git.blockers()
        );

        let packaged_without_git = Detection {
            git: false,
            kind: crate::install_kind::Kind::Package,
            ..installable()
        };
        assert!(
            packaged_without_git.blockers().is_empty(),
            "a package install does not need git: {:?}",
            packaged_without_git.blockers()
        );

        let unknown = Detection {
            distro: crate::distro::Distro::Unknown,
            ..installable()
        };
        assert_eq!(unknown.blockers().len(), 1);
    }

    #[test]
    fn the_password_is_asked_for_last_because_it_is_needed_last() {
        let mut app = app();
        app.detection = installable();

        app.stage = Stage::Options;
        app.question = menu::question_count() - 1;
        app.cursor = app.current_entries().len();
        press(&mut app, KeyCode::Enter);
        assert_eq!(app.overlay, Some(Overlay::Confirm));
        assert_eq!(app.confirm, Some(Confirm::Install));
        assert_eq!(app.stage, Stage::Options, "and nothing has started yet");

        char_key(&mut app, 'y');
        assert_eq!(app.stage, Stage::Privilege);
        assert_eq!(app.auth, Some(AuthMethod::Password));
    }

    #[test]
    fn the_password_screen_is_only_reached_by_picking_the_password_route() {
        let mut app = app();
        app.detection = installable();
        app.stage = Stage::Auth;
        app.cursor = 0;

        press(&mut app, KeyCode::Enter);
        assert_eq!(app.auth, Some(AuthMethod::Password));
        assert_eq!(app.stage, Stage::Privilege);
    }

    #[test]
    fn a_gate_with_one_route_asks_for_the_password_itself() {
        let mut app = app();
        app.need_auth();
        assert_eq!(app.auth, Some(AuthMethod::Password));
        assert_eq!(app.stage, Stage::Privilege);

        press(&mut app, KeyCode::Esc);
        assert_eq!(app.stage, Stage::Options);
        assert_eq!(app.auth, None);
        assert_eq!(app.leaving(), None);
    }

    #[test]
    fn a_reader_that_is_there_is_offered_first_and_a_machine_without_one_is_not_asked_about_it() {
        let with_reader = app().with_fingerprint(true);
        assert_eq!(
            with_reader.auth_methods(),
            vec![AuthMethod::Fingerprint, AuthMethod::Password],
            "the reader is the only route that needs nothing typed, so it is first"
        );

        let without = app();
        assert_eq!(
            without.auth_methods(),
            vec![AuthMethod::Password],
            "a route that cannot open is not offered"
        );
    }

    #[test]
    fn the_reader_is_asked_as_soon_as_it_is_chosen() {
        let mut app = app().with_fingerprint(true);
        app.detection = installable();
        app.stage = Stage::Auth;
        app.cursor = 0;

        press(&mut app, KeyCode::Enter);
        assert_eq!(app.auth, Some(AuthMethod::Fingerprint));
        assert_eq!(app.stage, Stage::Privilege);
        assert!(
            app.checking.is_some(),
            "choosing the reader is the whole question: the read starts on the key"
        );

        assert_eq!(app.password, "");
        app.tick();
        assert_eq!(
            app.password, "",
            "a finger is not a password and must not become one"
        );
        assert_ne!(
            app.stage,
            Stage::Privilege,
            "an accepted read carries on like any other route"
        );
    }

    #[test]
    fn a_read_that_did_not_happen_hands_the_gate_to_the_password_rather_than_asking_again() {
        let mut app = app().with_fingerprint(true);
        app.stage = Stage::Privilege;
        app.auth = Some(AuthMethod::Fingerprint);

        let (sender, receiver) = std::sync::mpsc::channel();
        sender.send(false).expect("the answer fits");
        app.checking = Some(receiver);
        app.tick();

        assert_eq!(
            app.stage,
            Stage::Privilege,
            "the gate asks for the password"
        );
        assert_eq!(app.auth, Some(AuthMethod::Password));
        assert_eq!(
            app.auth_methods(),
            vec![AuthMethod::Password],
            "the route that cannot open is gone"
        );
        assert!(app.fingerprint_spent);
        assert_eq!(
            app.problem.as_deref(),
            Some("No finger was recognized. Type your password instead."),
            "and says what happened, not what a wrong password means"
        );
        assert_eq!(app.attempts, 0, "the reader is not the password's count");

        assert!(
            !app.auth_methods().contains(&AuthMethod::Fingerprint),
            "and it stays gone"
        );
    }

    #[test]
    fn keys_do_not_go_into_a_reader() {
        let mut app = app().with_fingerprint(true);
        app.stage = Stage::Privilege;
        app.auth = Some(AuthMethod::Fingerprint);

        for character in "hunter2".chars() {
            char_key(&mut app, character);
        }
        press(&mut app, KeyCode::Backspace);
        assert_eq!(app.password, "", "the field is not there to type into");

        press(&mut app, KeyCode::Esc);
        assert_eq!(app.stage, Stage::Auth);
        assert_eq!(app.auth, None);
    }

    #[test]
    fn update_and_uninstall_confirm_before_handing_the_terminal_over() {
        for (index, action, handoff) in [
            (1, Action::Update, Handoff::Update),
            (2, Action::Uninstall, Handoff::Uninstall),
        ] {
            let mut app = app();
            app.detection.installed = true;
            app.home = index;
            assert_eq!(Action::ALL[app.home], action);

            assert_eq!(press(&mut app, KeyCode::Enter), Flow::Continue);
            assert_eq!(app.overlay, Some(Overlay::Confirm));
            assert_eq!(app.confirm, Some(Confirm::Handoff(action)));

            char_key(&mut app, 'n');
            assert_eq!(app.overlay, None);
            assert_eq!(app.leaving(), None);

            press(&mut app, KeyCode::Enter);
            char_key(&mut app, 'y');
            let ending = app.ending(Outcome::Ok);
            assert_eq!(ending.handoff, Some(handoff));
            assert_eq!(app.leaving(), None, "the handoff is the caller's job");
        }
    }

    #[test]
    fn the_confirmation_is_answered_by_the_button_under_the_highlight() {
        let mut app = app();
        app.detection.installed = true;
        app.home = 2;
        press(&mut app, KeyCode::Enter);
        assert_eq!(app.confirm, Some(Confirm::Handoff(Action::Uninstall)));
        assert_eq!(
            app.confirm_focus, 0,
            "the action is what the user came here to do"
        );

        press(&mut app, KeyCode::Down);
        assert_eq!(app.confirm_focus, 1, "the other button is reachable");
        press(&mut app, KeyCode::Enter);
        assert_eq!(app.overlay, None, "choosing cancel closes the question");
        assert_eq!(app.leaving(), None, "and nothing happened");

        press(&mut app, KeyCode::Enter);
        press(&mut app, KeyCode::Enter);
        assert_eq!(
            app.ending(Outcome::Ok).handoff,
            Some(Handoff::Uninstall),
            "the highlighted button is what enter chooses"
        );
    }

    #[test]
    fn stopping_a_run_does_not_put_the_stopping_button_under_the_highlight() {
        let mut app = app();
        app.stage = Stage::Run;
        app.set_status(0, Status::Running);
        char_key(&mut app, 'q');
        assert_eq!(app.confirm, Some(Confirm::Quit));
        assert_eq!(app.confirm_focus, 1, "the safe answer is the focused one");

        press(&mut app, KeyCode::Enter);
        assert_eq!(app.leaving(), None, "the run carries on");
        assert_eq!(app.overlay, None);
    }

    #[test]
    fn uninstall_is_refused_where_there_is_nothing_installed() {
        let mut app = app();
        app.detection.installed = false;
        app.home = 2;
        char_key(&mut app, ' ');
        assert_eq!(app.stage, Stage::Home);
        assert!(app.notice.is_some(), "and it says why");
        assert_eq!(app.overlay, None);
    }

    #[test]
    fn esc_steps_back_rather_than_leaving() {
        let mut solo = app();
        solo.need_auth();
        assert_eq!(solo.stage, Stage::Privilege);
        char_key(&mut solo, 'x');
        press(&mut solo, KeyCode::Esc);
        assert_eq!(
            solo.stage,
            Stage::Options,
            "the gate hands back to the answers"
        );
        assert_eq!(solo.password, "", "and it forgets the password");
        assert_eq!(solo.leaving(), None);

        let mut with_reader = app().with_fingerprint(true);
        with_reader.auth = Some(AuthMethod::Fingerprint);
        with_reader.stage = Stage::Privilege;
        press(&mut with_reader, KeyCode::Esc);
        assert_eq!(
            with_reader.stage,
            Stage::Auth,
            "the routes are where the choice was made"
        );
        assert_eq!(with_reader.auth, None);

        solo.stage = Stage::Options;
        solo.question = menu::question_count() - 1;
        for expected in (0..menu::question_count() - 1).rev() {
            press(&mut solo, KeyCode::Esc);
            assert_eq!(solo.question, expected);
        }
        press(&mut solo, KeyCode::Esc);
        assert_eq!(solo.stage, Stage::Detect);
        press(&mut solo, KeyCode::Esc);
        assert_eq!(solo.stage, Stage::Home);
        assert_eq!(solo.leaving(), None);
    }

    #[test]
    fn quitting_from_home_is_a_cancel_and_not_an_interruption() {
        let mut app = app();
        assert_eq!(press(&mut app, KeyCode::Esc), Flow::Done(Outcome::Canceled));
        assert_eq!(char_key(&mut app, 'q'), Flow::Done(Outcome::Canceled));
    }

    #[test]
    fn the_steps_are_told_this_run_is_an_install() {
        let env = app().step_environment();
        assert!(
            env.iter()
                .any(|(key, value)| *key == "CAELESTIA_FRESH_INSTALL" && value == "1"),
            "{env:?}"
        );
    }

    #[test]
    fn a_bad_password_is_asked_for_again_and_never_kept() {
        let mut app = app();
        app.stage = Stage::Privilege;
        for character in "hunter2".chars() {
            char_key(&mut app, character);
        }
        assert_eq!(app.password, "hunter2");
        press(&mut app, KeyCode::Backspace);
        assert_eq!(app.password, "hunter");
    }

    #[test]
    fn the_check_runs_off_the_draw_path_and_the_wait_is_visible() {
        let mut app = app();
        app.stage = Stage::Privilege;
        for character in "hunter2".chars() {
            char_key(&mut app, character);
        }
        press(&mut app, KeyCode::Enter);
        assert!(
            app.checking.is_some(),
            "the check has to run somewhere the frame can carry on without it"
        );
        assert_eq!(
            app.password, "hunter2",
            "the password is still needed for the steps"
        );
        assert_eq!(app.stage, Stage::Privilege, "and nothing has moved on yet");
        assert!(
            app.keys().is_empty(),
            "no key does anything while sudo has it"
        );

        press(&mut app, KeyCode::Enter);
        char_key(&mut app, 'x');
        assert_eq!(app.password, "hunter2");

        app.tick();
        assert!(
            matches!(app.stage, Stage::Run | Stage::Done),
            "the password was the last thing asked for, so the work starts, got {:?}",
            app.stage
        );
        assert!(
            app.statuses()
                .iter()
                .any(|status| *status != Status::Pending),
            "and the steps are under way"
        );
        assert_eq!(
            app.password, "",
            "the password leaves the field once it is used"
        );
    }

    #[test]
    fn a_refused_password_is_asked_for_again() {
        let mut app = app();
        app.stage = Stage::Privilege;
        app.password = "wrong".to_string();
        let (sender, receiver) = std::sync::mpsc::channel();
        sender.send(false).expect("the answer fits");
        app.checking = Some(receiver);

        app.tick();
        assert_eq!(app.stage, Stage::Privilege, "a refusal keeps the gate open");
        assert_eq!(app.attempts, 1);
        assert_eq!(app.password, "");
        assert!(
            app.problem
                .as_deref()
                .unwrap_or_default()
                .contains("accepted"),
            "the gate has to say it was refused: {:?}",
            app.problem
        );
    }

    #[test]
    fn a_control_chord_never_becomes_part_of_the_password() {
        let mut app = app();
        app.stage = Stage::Privilege;
        for character in "hunter2".chars() {
            char_key(&mut app, character);
        }
        for code in [
            KeyCode::Char('j'),
            KeyCode::Char('a'),
            KeyCode::Char('u'),
            KeyCode::Char('h'),
        ] {
            app.key(KeyEvent::new(code, KeyModifiers::CONTROL));
        }
        assert_eq!(
            app.password, "hunter2",
            "a chord is not a character of a password"
        );
    }

    #[test]
    fn the_run_screen_names_the_step_it_is_on_and_the_one_that_comes_next() {
        let mut app = run_with(&[Status::Ok, Status::Running]);
        assert_eq!(app.running_step(), Some(1));
        assert_eq!(app.next_step(), Some(2));

        app.set_status(1, Status::Ok);
        assert_eq!(app.running_step(), None);
        assert_eq!(app.next_step(), Some(2), "the next step is still named");
    }

    #[test]
    fn the_way_on_from_a_failure_is_a_decision_rather_than_a_dismissal() {
        let mut app = run_with(&[Status::Failed]);
        app.run.cursor_to(0);
        app.failure = Some(FailureCard::new(&STEPS[0], &[]));
        app.failure = Some(FailureCard::new(&STEPS[0], &[]));
        app.overlay = Some(Overlay::Failure);

        press(&mut app, KeyCode::Esc);
        assert_eq!(app.overlay, None);
        assert!(app.paused());
        assert_eq!(app.status(0), Status::Failed);
        assert_eq!(app.current(), 0, "and nothing advanced past it");

        assert_eq!(press(&mut app, KeyCode::Enter), Flow::Continue);
        assert_eq!(app.overlay, Some(Overlay::Failure));
    }

    #[test]
    fn stopping_at_a_failure_ends_the_session_as_a_failure() {
        let mut app = run_with(&[Status::Failed]);
        app.run.cursor_to(0);
        app.failure = Some(FailureCard::new(&STEPS[0], &[]));
        app.overlay = Some(Overlay::Failure);
        assert_eq!(char_key(&mut app, 'q'), Flow::Done(Outcome::Failed));
        assert_eq!(app.status(0), Status::Failed);
    }

    #[test]
    fn skipping_and_ignoring_both_carry_on_but_only_one_of_them_is_a_failure() {
        for (key, expected) in [('s', Status::Skipped), ('i', Status::Ignored)] {
            let last = STEPS.len() - 1;
            let mut app = run_with(&[Status::Failed]);
            app.run.cursor_to(last);
            app.set_status(last, Status::Failed);
            app.failure = Some(FailureCard::new(&STEPS[last], &[]));
            app.overlay = Some(Overlay::Failure);

            char_key(&mut app, key);
            assert_eq!(app.status(last), expected);
            assert_eq!(app.stage, Stage::Done);
            assert!(!app.paused());
        }
    }

    #[test]
    fn an_ignored_step_is_still_reported_as_a_failure_of_the_session() {
        let mut app = app();
        app.fill_statuses(Status::Ok);
        app.set_status(2, Status::Ignored);
        app.stage = Stage::Done;
        assert_eq!(app.tally().unresolved(), 1);
        assert_eq!(press(&mut app, KeyCode::Enter), Flow::Done(Outcome::Failed));
    }

    #[test]
    fn a_clean_install_finishes_with_a_success() {
        let mut app = app();
        app.fill_statuses(Status::Ok);
        app.set_status(1, Status::Warn);
        app.stage = Stage::Done;
        assert_eq!(app.tally().unresolved(), 0);
        assert_eq!(press(&mut app, KeyCode::Enter), Flow::Done(Outcome::Ok));
    }

    #[test]
    fn the_whole_install_walks_to_the_summary() {
        let mut app = app();
        app.begin_install();
        assert_eq!(app.stage, Stage::Done);
        assert!(
            app.statuses()
                .iter()
                .all(|status| matches!(status, Status::Ok | Status::Skipped)),
            "every step either ran or was switched off: {:?}",
            app.statuses()
        );
        assert_eq!(app.finished(), STEPS.len());
        assert!(app.total_elapsed() > Duration::ZERO);
    }

    #[test]
    fn retrying_from_the_summary_goes_back_to_the_run() {
        let mut app = app();
        app.begin_install();
        app.set_status(4, Status::Ignored);
        app.stage = Stage::Done;

        char_key(&mut app, 'r');
        assert_ne!(app.status(4), Status::Ignored, "the step was tried again");
        assert_eq!(app.stage, Stage::Done, "and the walk carried on to the end");
    }

    #[test]
    fn retrying_when_nothing_failed_says_so() {
        let mut app = app();
        app.fill_statuses(Status::Ok);
        app.stage = Stage::Done;
        char_key(&mut app, 'r');
        assert_eq!(app.stage, Stage::Done);
        assert!(app.notice.is_some());
    }

    #[test]
    fn logging_out_is_asked_for_before_the_session_ends() {
        let mut app = app();
        app.fill_statuses(Status::Ok);
        app.stage = Stage::Done;

        assert_eq!(char_key(&mut app, 'o'), Flow::Continue);
        assert_eq!(
            app.confirm,
            Some(Confirm::LogOut),
            "the session is not ended on one key"
        );
        assert_eq!(app.overlay, Some(Overlay::Confirm));
        assert_eq!(
            app.confirm_focus, 1,
            "the highlight starts on the answer that keeps the session"
        );

        press(&mut app, KeyCode::Esc);
        assert_eq!(app.confirm, None);
        assert_eq!(app.stage, Stage::Done);
        assert_eq!(app.handoff, None);

        char_key(&mut app, 'o');
        assert_eq!(char_key(&mut app, 'y'), Flow::Done(Outcome::Ok));
        assert_eq!(app.handoff, Some(Handoff::LogOut));
    }

    #[test]
    fn a_run_that_did_not_finish_is_not_logged_out_of() {
        let mut app = app();
        app.fill_statuses(Status::Ok);
        app.set_status(4, Status::Failed);
        app.stage = Stage::Done;

        assert!(
            !app.can_log_out(),
            "a half-installed desktop is not a session to restart"
        );
        assert_eq!(char_key(&mut app, 'o'), Flow::Continue);
        assert_eq!(app.confirm, None);
        assert_eq!(app.handoff, None);
    }

    #[test]
    fn options_reset_one_row_or_all_of_them() {
        let mut app = app();
        app.stage = Stage::Options;
        app.question = question_of(menu::PRESET_THEME_APPS);
        app.cursor = 0;
        assert!(
            matches!(app.selected(), Some(Entry::Preset { .. })),
            "the bulk row is the one under the cursor"
        );

        char_key(&mut app, ' ');
        let preset = app.selected().expect("a row");
        assert!(app.is_changed(preset));

        char_key(&mut app, 'r');
        let preset = app.selected().expect("a row");
        assert!(!app.is_changed(preset));

        app.answers.insert("SKIP_SYSTEM_UPDATE", "true".to_string());
        char_key(&mut app, 'R');
        assert_eq!(
            app.answers.get("SKIP_SYSTEM_UPDATE").map(String::as_str),
            Some("false"),
            "reset all put every question back"
        );
        assert!(app.notice.is_some());
    }

    #[test]
    fn a_question_is_answered_then_left_behind() {
        let mut app = app();
        app.stage = Stage::Options;
        let first = app.selected().map(Entry::id).expect("a row");

        press(&mut app, KeyCode::Enter);
        assert_eq!(app.question, 1);
        assert_eq!(app.cursor, 0);
        assert!(app.stage == Stage::Options);
        assert_eq!(
            app.answers.get(first).map(String::as_str),
            crate::menu::baseline_of(
                menu::items_of(0)
                    .iter()
                    .find(|entry| entry.id() == first)
                    .expect("the first row"),
                &app.answers
            ),
            "walking through the questions leaves the answers alone"
        );

        for _ in 1..menu::question_count() {
            press(&mut app, KeyCode::Enter);
        }
        assert_eq!(
            app.stage,
            Stage::Options,
            "the answers stay where they were"
        );
        assert_eq!(app.overlay, Some(Overlay::Confirm));
    }

    #[test]
    fn a_change_key_changes_whatever_row_is_under_it() {
        let mut app = app();
        app.stage = Stage::Options;
        app.question = 0;
        app.cursor = 0;
        let switch = app.selected().map(Entry::id).expect("a switch");

        char_key(&mut app, ' ');
        assert_eq!(
            app.answers.get(switch).map(String::as_str),
            Some("false"),
            "a switch flips"
        );
        press(&mut app, KeyCode::Right);
        assert_eq!(app.answers.get(switch).map(String::as_str), Some("true"));

        app.question = question_of("DEFAULT_SHELL");
        app.cursor = app
            .current_entries()
            .iter()
            .position(|entry| entry.id() == "DEFAULT_SHELL")
            .expect("the login shell row is in its question");
        let select = app.selected().map(Entry::id).expect("a select");
        assert_eq!(app.answers.get(select).map(String::as_str), Some("fish"));
        char_key(&mut app, ' ');
        assert_eq!(app.answers.get(select).map(String::as_str), Some("zsh"));
    }

    #[test]
    fn the_log_pane_and_the_page_keys_agree_about_how_tall_it_is() {
        let mut app = run_with(&[Status::Running]);
        for line in 0..200 {
            app.log
                .push(LogLine::parse(&format!("  [INFO]  line {line}")));
        }
        let (start, rows) = app.log_window_for(app.log_rows());
        assert_eq!(rows, app.log_rows());
        assert_eq!(start, app.log.len() - rows, "following shows the tail");

        app.log_follow = false;
        app.log_scroll = 0;
        press(&mut app, KeyCode::PageDown);
        assert_eq!(app.log_scroll, rows, "a page is the pane, not a guess");
        press(&mut app, KeyCode::End);
        assert!(app.log_follow, "end goes back to following");
    }

    #[test]
    fn n_and_p_walk_the_lines_that_went_wrong() {
        let mut app = run_with(&[Status::Running]);
        app.log = (0..200)
            .map(|line| match line {
                20 => LogLine::parse("  [WARN]  a slow mirror"),
                120 => LogLine::parse("  [ERR]   a package failed"),
                180 => LogLine::parse("  [WARN]  another slow mirror"),
                _ => LogLine::parse(&format!("  [INFO]  line {line}")),
            })
            .collect();
        app.log_follow = true;

        char_key(&mut app, 'n');
        assert!(!app.log_follow, "a jump is a reason to stop following");
        assert_eq!(app.log_problem, Some(20));
        assert_eq!(app.notice.as_deref(), Some("problem 1 of 3"));
        let (start, rows) = app.log_window_for(app.log_rows());
        assert!((start..start + rows).contains(&20), "the warning is shown");

        char_key(&mut app, 'n');
        assert_eq!(app.log_problem, Some(120));
        assert_eq!(app.notice.as_deref(), Some("problem 2 of 3"));
        char_key(&mut app, 'p');
        assert_eq!(app.log_problem, Some(20));

        char_key(&mut app, 'p');
        assert_eq!(app.log_problem, Some(180));
        assert_eq!(app.notice.as_deref(), Some("problem 3 of 3"));

        press(&mut app, KeyCode::PageUp);
        assert_eq!(app.log_problem, None);
    }

    #[test]
    fn a_log_with_nothing_wrong_in_it_says_so_rather_than_jumping() {
        let mut app = run_with(&[Status::Running]);
        app.log = vec![LogLine::parse("  [INFO]  chatter")];
        char_key(&mut app, 'n');
        assert!(app.log_problem.is_none());
        assert_eq!(app.notice.as_deref(), Some("nothing in the log went wrong"));
    }

    #[test]
    fn the_loud_filter_keeps_warnings_and_errors() {
        let mut app = run_with(&[Status::Running]);
        app.log = vec![
            LogLine::parse("  [INFO]  chatter"),
            LogLine::parse("  [WARN]  slow mirror"),
            LogLine::parse("  [ERR]   a package failed"),
        ];
        assert_eq!(app.visible_log().count(), 3);
        char_key(&mut app, 'w');
        assert_eq!(app.visible_log().count(), 2);
        assert!(app.log_loud_only);
        char_key(&mut app, 'w');
        assert_eq!(app.visible_log().count(), 3);
    }

    #[test]
    fn quitting_while_a_step_is_running_asks_first_and_then_stops_it() {
        let mut app = run_with(&[Status::Running]);
        let ctrl_c = KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL);
        assert_eq!(app.key(ctrl_c), Flow::Continue);
        assert_eq!(app.overlay, Some(Overlay::Confirm));
        assert_eq!(app.confirm, Some(Confirm::Quit));
        assert_eq!(app.key(ctrl_c), Flow::Done(Outcome::Interrupted));
    }

    #[test]
    fn quitting_when_nothing_is_running_does_not_ask() {
        let mut app = run_with(&[Status::Ok]);
        let ctrl_c = KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL);
        assert_eq!(app.key(ctrl_c), Flow::Done(Outcome::Interrupted));
    }

    #[test]
    fn the_theme_can_be_flipped_from_the_home_screen() {
        let mut app = app();
        let before = app.theme.mode;
        char_key(&mut app, 't');
        assert_ne!(app.theme.mode, before);
        assert!(app.notice.is_some());
        char_key(&mut app, 't');
        assert_eq!(app.theme.mode, before);
    }

    #[test]
    fn a_notice_lasts_until_the_next_key() {
        let mut app = app();
        char_key(&mut app, 't');
        assert!(app.notice.is_some());
        char_key(&mut app, 'j');
        assert_eq!(app.notice, None);
    }

    #[test]
    fn every_screen_offers_a_way_on() {
        let mut app = app();
        let stages = [
            Stage::Home,
            Stage::Privilege,
            Stage::Options,
            Stage::Run,
            Stage::Done,
        ];
        for stage in stages {
            app.stage = stage;
            app.overlay = None;
            assert!(!app.keys().is_empty(), "{stage:?} has no hints");
            for key in app.keys() {
                assert!(!key.keys.is_empty(), "{stage:?} hints an empty key");
                assert!(!key.what.is_empty(), "{stage:?} hints nothing");
            }
        }
        for overlay in [
            Overlay::Keys,
            Overlay::Log,
            Overlay::Failure,
            Overlay::Confirm,
        ] {
            app.overlay = Some(overlay);
            assert!(!app.keys().is_empty(), "{overlay:?} has no hints");
        }
    }

    #[test]
    fn the_keymap_overlay_closes_on_the_keys_a_user_would_try() {
        let mut app = app();
        for code in [KeyCode::Char('?'), KeyCode::Esc, KeyCode::Enter] {
            char_key(&mut app, '?');
            assert_eq!(app.overlay, Some(Overlay::Keys));
            press(&mut app, code);
            assert_eq!(app.overlay, None, "{code:?} did not close it");
        }
    }

    #[test]
    fn the_log_overlay_closes_and_pages_the_way_the_pane_does() {
        let mut app = run_with(&[Status::Running]);
        for line in 0..200 {
            app.log
                .push(LogLine::parse(&format!("  [INFO]  line {line}")));
        }
        char_key(&mut app, 'l');
        assert_eq!(app.overlay, Some(Overlay::Log));

        let rows = crate::ui::log_overlay_rows(app.area) as usize;
        assert!(rows > 3, "the overlay has room of its own");
        app.log_follow = false;
        app.log_scroll = 100;
        press(&mut app, KeyCode::PageUp);
        assert_eq!(app.log_scroll, 100 - rows, "it pages by its own height");
        press(&mut app, KeyCode::Home);
        assert_eq!(app.log_scroll, 0);

        char_key(&mut app, 'l');
        assert_eq!(app.overlay, None);
    }

    #[test]
    fn the_step_environment_carries_the_answers_and_the_bundle() {
        let app = app();
        let env = app.step_environment();
        assert!(env.iter().any(|(key, _)| *key == "BUNDLE_DIR"));
        assert!(
            env.iter()
                .any(|(key, value)| *key == "CONFIRM_ARG" && value == "--noconfirm")
        );
        assert!(env.iter().any(|(key, _)| *key == "INSTALL_FISH"));
    }

    #[test]
    fn a_scripted_answer_is_where_the_wizard_starts_rather_than_a_change() {
        let mut app = app();
        app.apply_settings(&[
            ("SYNC_LOGIN_SCREEN", "false".to_string()),
            ("DEFAULT_SHELL", "zsh".to_string()),
        ]);

        assert_eq!(
            app.answers.get("DEFAULT_SHELL").map(String::as_str),
            Some("zsh")
        );
        assert_eq!(
            app.answers.get("INSTALL_FISH").map(String::as_str),
            Some("false"),
            "a scripted shell still decides fish"
        );
        assert_eq!(
            app.changed_answers(),
            0,
            "the user has not touched anything"
        );

        assert!(
            app.step_environment()
                .iter()
                .any(|(key, value)| *key == "DEFAULT_SHELL" && value == "zsh")
        );

        app.stage = Stage::Options;
        char_key(&mut app, ' ');
        assert!(app.changed_answers() > 0);
        char_key(&mut app, 'R');
        assert_eq!(
            app.answers.get("DEFAULT_SHELL").map(String::as_str),
            Some("zsh")
        );
        assert_eq!(app.changed_answers(), 0);
    }

    #[test]
    fn a_cursor_past_the_end_of_a_question_is_pulled_back_onto_it() {
        let mut app = app();
        app.stage = Stage::Options;
        app.question = question_of("SYNC_LOGIN_SCREEN");
        app.cursor = 99;

        press(&mut app, KeyCode::Down);
        assert!(
            app.cursor < app.current_entries().len(),
            "the cursor is on the list"
        );
        assert!(app.selected().is_some());

        press(&mut app, KeyCode::Enter);
        assert_eq!(app.stage, Stage::Options, "onward still works");
    }

    #[test]
    fn the_last_row_of_a_question_is_the_way_on_and_the_keys_reach_it() {
        let mut app = app();
        app.stage = Stage::Options;
        app.question = 0;
        app.cursor = 0;
        assert!(!app.on_continue(), "the first row is an answer");
        let answers = app.current_entries().len();

        for _ in 0..answers {
            press(&mut app, KeyCode::Down);
        }
        assert!(app.on_continue(), "the arrows reach the way on");
        assert!(app.selected().is_none(), "and it is not an answer");
        assert_eq!(app.question, 0);

        press(&mut app, KeyCode::Enter);
        assert_eq!(app.question, 1, "enter goes on from it");

        app.question = 0;
        app.cursor = app.current_entries().len();
        char_key(&mut app, ' ');
        assert_eq!(app.question, 1, "and so does space");
    }

    #[test]
    fn switching_a_row_off_leaves_the_cursor_on_it() {
        let mut app = app();
        app.stage = Stage::Options;
        app.question = question_of("SYNC_LOGIN_SCREEN");
        app.cursor = 0;
        let rows = app.current_entries().len();

        char_key(&mut app, ' ');
        assert_eq!(
            app.current_entries().len(),
            rows,
            "the row stays where it can be switched back on"
        );
        assert_eq!(
            app.selected().map(Entry::id),
            Some("SYNC_LOGIN_SCREEN"),
            "the cursor is still on the row that was changed"
        );
        assert!(app.cursor < app.current_entries().len());
    }

    #[test]
    fn the_header_says_which_action_is_running_and_where() {
        let mut app = app();
        assert_eq!(app.header().left, "installer");
        app.stage = Stage::Options;
        app.action = Action::Install;
        assert_eq!(app.header().left, "install");
        assert!(app.header().right.contains("install"));
        assert!(!app.detection.distro_label().is_empty());
        assert!(
            app.header().right.contains(app.detection.distro_label()),
            "the header reports the machine that was read: {}",
            app.header().right
        );
    }
}
