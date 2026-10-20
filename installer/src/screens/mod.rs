use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Paragraph};

use crate::app::{Action, App, AuthMethod, Confirm, FailureCard, Overlay, Stage};
use crate::glyphs::Glyphs;
use crate::logline::{Kind, LogLine};
use crate::menu::{self, Entry};
use crate::steps::{STEPS, Status};
use crate::theme::{self, Role, Theme};
use crate::ui;

mod auth;
mod detect;
mod done;
mod home;
mod options;
mod overlay;
mod privilege;
mod run;

use auth::auth;
use detect::detect;
use done::done;
use home::home;
use options::options;
use overlay::{confirm_overlay, failure_overlay, keys_overlay, log_overlay};
use privilege::privilege;
use run::run;

/// The width of the value column on the options screen.
pub(super) const VALUE_COLUMN: usize = 12;
pub(super) const NAME_CELL: usize = 52;
pub(super) const LABEL_COLUMN: usize = 8;
pub(super) const LOGOUT_BUTTON: &str = "Log out and apply Caelestia";
pub fn draw(frame: &mut Frame, app: &mut App) {
    app.area = frame.area();
    let header = app.header();
    let counter = app.counter();
    let keys = app.keys();
    let Some(chrome) = ui::chrome(
        frame,
        &app.theme,
        app.glyphs,
        &header,
        &keys,
        counter.as_deref(),
    ) else {
        return;
    };
    let body = chrome.body;

    match app.stage {
        Stage::Home => home(frame, app, body),
        Stage::Detect => detect(frame, app, body),
        Stage::Privilege => privilege(frame, app, body),
        Stage::Auth => auth(frame, app, body),
        Stage::Options => options(frame, app, body),
        Stage::Run => run(frame, app),
        Stage::Done => done(frame, app, body),
    }

    match app.overlay {
        Some(Overlay::Keys) => keys_overlay(frame, app),
        Some(Overlay::Log) => log_overlay(frame, app),
        Some(Overlay::Failure) => failure_overlay(frame, app),
        Some(Overlay::Confirm) => confirm_overlay(frame, app),
        None => {}
    }
}
pub(super) fn blank() -> Line<'static> {
    Line::from("")
}
/// The transient answer to the last key, shown until the next one.
pub(super) fn notice(app: &App) -> Vec<Line<'static>> {
    match &app.notice {
        Some(text) => vec![Line::from(Span::styled(text.clone(), app.theme.accent()))],
        None => Vec::new(),
    }
}
/// Wrap prose to a column without a `Paragraph`, for the places that lay lines out by hand.
pub(super) fn wrap(text: &str, width: usize) -> Vec<String> {
    crate::ui::wrap(text, width, 20)
}

#[cfg(test)]
mod tests {
    use super::options::option_rows;
    use super::*;
    use crate::app::{AuthMethod, Flow, Overlay, Stage};
    use crate::env::Env;
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
    use std::path::PathBuf;

    /// An app on a machine that says nothing about itself.
    ///
    /// Which distribution this host runs, and whether it already has Caelestia on it, are the
    /// host's facts and not the test's: a machine that has both draws a longer home screen, and a
    /// test about where the rows land would then be a test about the host.
    fn on(env: Env) -> App {
        let mut app = App::new(
            PathBuf::from("/tmp/caelestia"),
            PathBuf::from("/tmp/install.log"),
            env,
        );
        app.pretend = true;
        app.detection.installed = false;
        app.unfinished.clear();
        app.needs_password = true;
        app
    }

    fn app() -> App {
        on(Env::default())
    }

    /// The same, on a terminal that can draw a fill and a border.
    fn color_app() -> App {
        on(Env {
            term: Some("xterm-256color".to_string()),
            colorterm: Some("truecolor".to_string()),
            ..Env::default()
        })
    }

    /// An app on a machine with a fingerprint reader enrolled.
    fn app_that_has_a_reader() -> App {
        app().with_fingerprint(true)
    }

    fn app_that_stopped_at(step: &str) -> App {
        let mut app = app();
        app.detection.installed = true;
        app.unfinished = vec![(step.to_string(), crate::steps::Status::Failed)];
        app
    }

    /// The frame as cells rather than as text.
    fn cells(app: &mut App, width: u16, height: u16) -> ratatui::buffer::Buffer {
        let backend = ratatui::backend::TestBackend::new(width, height);
        let mut terminal = ratatui::Terminal::new(backend).expect("a test terminal");
        terminal
            .draw(|frame| draw(frame, app))
            .expect("a drawn frame");
        terminal.backend().buffer().clone()
    }

    fn row_of(buffer: &ratatui::buffer::Buffer, needle: &str) -> u16 {
        (0..buffer.area.height)
            .find(|y| {
                let mut text = String::new();
                for x in 0..buffer.area.width {
                    text.push_str(buffer[(x, *y)].symbol());
                }
                text.contains(needle)
            })
            .unwrap_or_else(|| panic!("no row holds {needle:?}"))
    }

    #[test]
    fn the_footer_is_filled_right_across_the_terminal() {
        let mut app = color_app();
        let accent = app.theme.color(Role::Accent);
        let buffer = cells(&mut app, 100, 30);
        let bar = buffer.area.height - 2;
        for x in 1..buffer.area.width - 1 {
            assert_eq!(buffer[(x, bar)].bg, accent, "the bar stops at column {x}");
        }
        assert_ne!(
            buffer[(0, bar)].bg,
            accent,
            "the bar covers the frame's edge"
        );
        assert_ne!(
            buffer[(1, bar - 1)].bg,
            accent,
            "the body above it is filled"
        );
    }

    #[test]
    fn the_row_the_cursor_is_on_is_the_only_filled_row_of_the_list() {
        let mut app = color_app();
        let accent = app.theme.color(Role::Accent);
        let buffer = cells(&mut app, 100, 30);
        let chosen = row_of(&buffer, "Install Caelestia");
        let other = row_of(&buffer, "Uninstall Caelestia");
        let filled = |row: u16| (0..buffer.area.width).any(|x| buffer[(x, row)].bg == accent);
        assert!(filled(chosen), "the cursor's row is not filled");
        assert!(!filled(other), "a row the cursor is not on is filled");
    }

    /// How many blank cells there are between the frame's left edge and the first thing drawn,
    /// counting cells rather than bytes.
    fn left_margin(line: &str) -> usize {
        let inside: String = line.chars().skip(1).collect();
        inside.chars().count() - inside.trim_start().chars().count()
    }

    #[test]
    fn a_wide_frame_puts_the_home_composition_in_the_middle() {
        let width = 236u16;
        let height = 60u16;
        let mut app = app();
        let text = render(&mut app, width, height);
        let lines: Vec<&str> = text.lines().collect();

        let mark_row = lines
            .iter()
            .position(|line| line.contains('\u{2588}'))
            .unwrap_or_else(|| panic!("the mark is on screen:\n{text}"));
        let panel_bottom = lines
            .iter()
            .position(|line| line.contains('\u{2514}'))
            .expect("the action panel is on screen");

        let left = left_margin(lines[mark_row]);
        let corner = lines[panel_bottom]
            .chars()
            .collect::<Vec<char>>()
            .iter()
            .rposition(|cell| *cell == '\u{2518}')
            .expect("a panel corner");
        let right = width as usize - corner - 2;
        assert!(
            left > 20,
            "the composition is against the left edge: {left} columns of air"
        );
        assert!(
            (left as isize - right as isize).abs() <= 4,
            "the block is off centre: {left} columns on the left, {right} on the right"
        );

        let below = height as usize - 1 - panel_bottom;
        assert!(
            (mark_row as isize - below as isize).abs() <= 4,
            "the block is off centre: {mark_row} rows above, {below} below"
        );
    }

    #[test]
    fn a_wide_frame_puts_the_questions_in_the_middle() {
        let width = 236u16;
        let mut app = app();
        app.stage = Stage::Options;
        let text = render(&mut app, width, 60);
        let lines: Vec<&str> = text.lines().collect();
        let rail_top = lines
            .iter()
            .position(|line| line.contains('\u{250c}') && line.contains("Appearance"))
            .expect("the map is on screen");
        let left = left_margin(lines[rail_top]);
        assert!(left > 20, "the map is against the left edge: {left}");
        assert!(
            left < (width / 2) as usize,
            "the block is past the middle, not in it: {left}"
        );

        assert!(
            lines[rail_top].contains('\u{250c}'),
            "the map's corner is on the heading row:\n{text}"
        );
    }

    #[test]
    fn the_password_is_asked_for_rather_than_chosen_from_a_list() {
        let mut app = app();
        app.pretend = true;
        app.detection = crate::app::Detection {
            distro: crate::distro::Distro::Arch,
            kind: crate::install_kind::Kind::Source,
            installed: false,
            package_manager: Some("pacman"),
            quickshell: true,
            git: true,
        };
        app.stage = Stage::Options;
        app.question = crate::menu::question_count() - 1;
        app.cursor = app.current_entries().len();
        app.key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        assert_eq!(
            app.overlay,
            Some(Overlay::Confirm),
            "the last answer asks before it acts"
        );
        app.key(KeyEvent::new(KeyCode::Char('y'), KeyModifiers::NONE));

        assert_eq!(
            app.stage,
            Stage::Privilege,
            "blockers: {:?}, problem: {:?}",
            app.detection.blockers(),
            app.problem
        );
        let text = render(&mut app, 120, 40);
        assert!(
            text.contains("root privileges"),
            "the question is up: {text}"
        );
        assert!(text.contains("password"), "and the field is on it: {text}");
        assert!(
            !text.contains("Type your password here"),
            "the list of routes is not between them: {text}"
        );

        let mut with_reader = app_that_has_a_reader();
        with_reader.pretend = true;
        with_reader.detection = app.detection.clone();
        with_reader.stage = Stage::Options;
        with_reader.question = crate::menu::question_count() - 1;
        with_reader.cursor = with_reader.current_entries().len();
        with_reader.key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        with_reader.key(KeyEvent::new(KeyCode::Char('y'), KeyModifiers::NONE));
        assert_eq!(with_reader.stage, Stage::Auth);
        let text = render(&mut with_reader, 120, 40);
        assert!(text.contains("Type your password here"), "{text}");
        assert!(text.contains("Touch the fingerprint reader"), "{text}");
    }

    #[test]
    fn a_half_finished_install_is_not_called_installed() {
        let mut half = app_that_stopped_at("08-build-shell");
        let text = render(&mut half, 120, 40);
        assert!(
            text.contains("the last run did not finish"),
            "the state says what happened: {text}"
        );
        assert!(
            text.contains("08-build-shell"),
            "and names the step it stopped at: {text}"
        );

        let mut done = app();
        done.detection.installed = true;
        let text = render(&mut done, 120, 40);
        assert!(text.contains("Caelestia is installed"), "{text}");
        assert!(!text.contains("did not finish"), "{text}");
    }

    #[test]
    fn the_detection_screen_says_where_the_last_install_stopped() {
        let mut app = app_that_stopped_at("08-build-shell");
        app.stage = Stage::Detect;
        let text = render(&mut app, 120, 40);
        assert!(
            text.contains("The last install did not finish"),
            "the headline is the half install, not a refusal: {text}"
        );
        assert!(
            !text.contains("already installed"),
            "and not the cross that greets a finished one: {text}"
        );
        assert!(
            text.contains("stopped at 08-build-shell"),
            "the screen explains the half install: {text}"
        );
        assert!(
            text.contains("carries on"),
            "and what continuing does about it: {text}"
        );
    }

    #[test]
    fn a_middling_terminal_still_puts_the_actions_beside_the_words() {
        let mut app = app();
        let text = render(&mut app, 120, 40);
        let lines: Vec<&str> = text.lines().collect();

        let panel_row = lines
            .iter()
            .position(|line| line.contains("\u{25b6} Install Caelestia"))
            .expect("the actions are on screen");
        let panel_col = lines[panel_row - 1]
            .chars()
            .collect::<Vec<char>>()
            .iter()
            .position(|cell| *cell == '\u{250c}')
            .expect("the actions have a panel around them");
        assert!(
            panel_col > 20,
            "the panel is not beside anything: {panel_col}"
        );
        assert!(
            panel_col < 60,
            "the panel belongs beside the words, not out past them: {panel_col}"
        );
    }

    #[test]
    fn a_wide_terminal_spends_its_width_on_a_second_column() {
        let mut wide = app();
        wide.stage = Stage::Options;
        wide.action = crate::app::Action::Install;
        let text = render(&mut wide, 235, 58);
        assert!(text.contains("Advanced"), "no map:\n{text}");

        let mut narrow = app();
        narrow.stage = Stage::Options;
        narrow.action = crate::app::Action::Install;
        let text = render(&mut narrow, 62, 24);
        assert!(!text.contains("Advanced"), "a map in one column:\n{text}");
        assert!(
            text.contains("Appearance"),
            "and the question left:\n{text}"
        );
    }

    #[test]
    fn a_wide_detection_screen_says_what_it_found_and_what_it_means() {
        let mut app = app();
        app.stage = Stage::Detect;
        app.detection = crate::app::Detection {
            distro: crate::distro::Distro::Arch,
            kind: crate::install_kind::Kind::Source,
            installed: true,
            package_manager: Some("pacman"),
            quickshell: true,
            git: true,
        };
        let text = render(&mut app, 235, 58);
        assert!(text.contains("this machine"), "{text}");
        assert!(text.contains("already installed"), "no verdict: {text}");
        assert!(text.contains("Arch-based Linux"), "{text}");
    }

    #[test]
    fn the_detection_screen_lists_what_it_found_about_this_machine() {
        let mut app = app();
        app.stage = Stage::Detect;
        app.detection = crate::app::Detection {
            distro: crate::distro::Distro::Arch,
            kind: crate::install_kind::Kind::Source,
            installed: false,
            package_manager: Some("pacman"),
            quickshell: true,
            git: true,
        };
        let text = render(&mut app, 100, 30);
        assert!(text.contains("This machine"), "{text}");
        for row in [
            "distribution",
            "Arch-based Linux",
            "package manager",
            "pacman",
            "git",
            "found",
        ] {
            assert!(text.contains(row), "{row} is missing from:\n{text}");
        }
        assert!(
            text.contains("Nothing is in the way"),
            "an install with a clear field says so: {text}"
        );
        assert!(
            text.contains("continue"),
            "the way off this screen is on it: {text}"
        );
    }

    #[test]
    fn the_detection_screen_is_centred_rather_than_pinned_to_the_left_edge() {
        let mut app = app();
        app.stage = Stage::Detect;
        app.detection = crate::app::Detection {
            distro: crate::distro::Distro::Arch,
            kind: crate::install_kind::Kind::Source,
            installed: false,
            package_manager: Some("pacman"),
            quickshell: true,
            git: true,
        };

        let wide = render(&mut app, 164, 46);
        let first = wide
            .lines()
            .find(|line| line.contains("this machine"))
            .unwrap_or_else(|| panic!("no findings panel in:\n{wide}"));
        let indent = first.chars().take_while(|c| *c == ' ' || *c == '│').count();
        assert!(indent > 20, "the panel starts at column {indent}:\n{wide}");
    }

    #[test]
    fn the_detection_screen_warns_before_an_install_is_overwritten() {
        let mut app = app();
        app.stage = Stage::Detect;
        app.detection = crate::app::Detection {
            distro: crate::distro::Distro::Arch,
            kind: crate::install_kind::Kind::Source,
            installed: true,
            package_manager: Some("pacman"),
            quickshell: true,
            git: true,
        };
        app.detection.installed = true;
        let text = render(&mut app, 100, 30);
        assert!(
            text.contains("already installed"),
            "the one fact that changes the decision: {text}"
        );
        assert!(
            text.contains("replaced"),
            "and what continuing does: {text}"
        );
        assert!(
            text.contains("update instead"),
            "and the cheaper route, on the footer: {text}"
        );
    }

    #[test]
    fn the_detection_screen_names_what_would_stop_the_install() {
        let mut unknown = app();
        unknown.stage = Stage::Detect;
        unknown.detection = crate::app::Detection {
            distro: crate::distro::Distro::Unknown,
            kind: crate::install_kind::Kind::Source,
            installed: false,
            package_manager: None,
            quickshell: false,
            git: false,
        };
        let text = render(&mut unknown, 100, 30);
        assert!(
            text.contains("not one the installer knows"),
            "a distribution it cannot help with is said plainly: {text}"
        );
        assert_eq!(
            unknown.detection.blockers().len(),
            1,
            "and said once: {:?}",
            unknown.detection.blockers()
        );

        let mut known = app();
        known.stage = Stage::Detect;
        known.detection = crate::app::Detection {
            distro: crate::distro::Distro::Arch,
            kind: crate::install_kind::Kind::Source,
            installed: false,
            package_manager: None,
            quickshell: false,
            git: false,
        };
        let text = render(&mut known, 100, 30);
        assert!(text.contains("No supported package manager"), "{text}");
        assert!(text.contains("git is missing"), "{text}");

        let mut overlay = app();
        overlay.overlay = Some(Overlay::Keys);
        let text = render(&mut overlay, 100, 34);
        for line in ["dark or light", "this list"] {
            assert!(text.contains(line), "{line} is missing from:\n{text}");
        }
        assert_eq!(
            text.matches("reset").count(),
            1,
            "a key is described once, in the list: {text}"
        );
    }

    #[test]
    fn the_reason_a_screen_is_blocking_survives_a_short_terminal() {
        let problem = "Sort out the above first.";
        let worst = crate::app::Detection {
            distro: crate::distro::Distro::Unknown,
            kind: crate::install_kind::Kind::Source,
            installed: true,
            package_manager: None,
            quickshell: false,
            git: false,
        };

        let mut roomy = app();
        roomy.stage = Stage::Detect;
        roomy.detection = crate::app::Detection {
            distro: crate::distro::Distro::Arch,
            ..worst.clone()
        };
        assert!(render(&mut roomy, 100, 30).contains("Your own configs are backed up first"));

        let mut tight = app();
        tight.stage = Stage::Detect;
        tight.detection = worst;
        tight.problem = Some(problem.to_string());
        let text = render(&mut tight, 60, 20);
        assert!(
            text.contains("not one the installer knows"),
            "the reason survives: {text}"
        );
        assert!(
            text.contains("already installed"),
            "and what it means for the decision: {text}"
        );
        assert!(
            text.contains("Sort out the above first"),
            "and the answer to the key: {text}"
        );
        assert!(
            !text.contains("Your own configs are backed up first"),
            "at the cost of the explanation: {text}"
        );
        let headline = text.find("already installed").expect("the headline");
        let explanation = text.find("Continuing installs over it");
        assert!(
            explanation.is_none_or(|at| at > headline),
            "the explanation follows the thing it explains: {text}"
        );
    }

    #[test]
    fn the_privilege_routes_are_offered_by_name() {
        let mut app = app();
        app.stage = Stage::Auth;
        let text = render(&mut app, 100, 30);
        for phrase in ["root privileges", "Type your password here"] {
            assert!(text.contains(phrase), "{phrase} is missing from:\n{text}");
        }
        assert!(
            !text.contains("sudo -v"),
            "the command appears with the route that needs it, not before: {text}"
        );
        assert!(
            !text.contains("For when it cannot ask"),
            "and nothing is said twice: {text}"
        );
    }

    #[test]
    fn a_reader_is_offered_by_name_and_asked_by_touching_it() {
        let mut app = app().with_fingerprint(true);
        app.stage = Stage::Auth;
        let text = render(&mut app, 100, 30);
        assert!(
            text.contains("Touch the fingerprint reader"),
            "the route is named on the list: {text}"
        );

        app.auth = Some(AuthMethod::Fingerprint);
        app.stage = Stage::Privilege;
        let (sender, receiver) = std::sync::mpsc::channel();
        sender.send(true).expect("the answer fits");
        app.checking = Some(receiver);
        let waiting = render(&mut app, 100, 30);
        assert!(
            waiting.contains("Touch the fingerprint reader"),
            "{waiting}"
        );
        assert!(
            !waiting.contains("password"),
            "a reader is not typed into: {waiting}"
        );
        assert!(
            waiting.contains("Waiting for your finger"),
            "and the wait says what it is waiting for: {waiting}"
        );

        app.checking = None;
        app.problem = Some("No finger was recognized.".to_string());
        let refused = render(&mut app, 100, 30);
        assert!(refused.contains("No finger was recognized."), "{refused}");
        assert!(
            !refused.contains("password"),
            "and still asks for no password: {refused}"
        );
    }

    #[test]
    fn the_detection_screen_says_why_a_guard_sent_the_user_back() {
        let mut app = app();
        app.stage = Stage::Detect;
        app.detection = crate::app::Detection {
            distro: crate::distro::Distro::Arch,
            kind: crate::install_kind::Kind::Source,
            installed: true,
            package_manager: Some("pacman"),
            quickshell: true,
            git: true,
        };
        app.installed_when_started = false;
        app.problem = Some("Caelestia was installed while the wizard was open.".to_string());

        let text = render(&mut app, 100, 30);
        assert!(
            text.contains("while the wizard was open"),
            "an explained block, not a screen that changed its mind: {text}"
        );
        assert!(text.contains("already installed"), "{text}");
    }

    #[test]
    fn a_question_screen_names_the_question_and_says_where_it_is() {
        let mut app = app();
        app.stage = Stage::Options;
        app.question = 0;
        let text = render(&mut app, 100, 30);
        assert!(text.contains(menu::question(0)), "{text}");
        assert_eq!(
            menu::question(0),
            "Appearance",
            "and the heading is the whole of it"
        );
        assert!(
            text.contains(&format!("question 1/{}", menu::question_count())),
            "and how far along it is: {text}"
        );

        app.question = menu::question_count() - 1;
        let last = render(&mut app, 100, 30);
        assert!(
            last.contains(&format!(
                "question {}/{}",
                menu::question_count(),
                menu::question_count()
            )),
            "{last}"
        );
    }

    #[test]
    fn a_question_that_was_answered_says_so_above_its_rows() {
        let mut app = app();
        app.stage = Stage::Options;
        app.question = 0;
        assert!(!render(&mut app, 100, 30).contains("changed"));

        char_key(&mut app, ' ');
        assert!(
            render(&mut app, 100, 30).contains("changed"),
            "stepping back to a screen that was edited should say so"
        );
    }

    #[test]
    fn the_confirmation_states_what_saying_yes_starts() {
        let mut app = app();
        app.confirm = Some(Confirm::Install);
        app.overlay = Some(Overlay::Confirm);
        let text = render(&mut app, 100, 30);
        assert!(text.contains("Install Caelestia now?"), "{text}");
        assert!(
            text.contains("Install (y)"),
            "the answer is a button: {text}"
        );
        assert!(text.contains("Cancel (n)"), "and so is the other: {text}");
        assert!(text.contains("backed up"), "{text}");
        assert!(
            !text.contains("steps"),
            "the step count is the installer's business: {text}"
        );
    }

    fn char_key(app: &mut App, character: char) -> Flow {
        app.key(KeyEvent::new(KeyCode::Char(character), KeyModifiers::NONE))
    }

    /// The frame as text, so a test can read what a user would see.
    fn render(app: &mut App, width: u16, height: u16) -> String {
        let backend = ratatui::backend::TestBackend::new(width, height);
        let mut terminal = ratatui::Terminal::new(backend).expect("a test terminal");
        terminal
            .draw(|frame| draw(frame, app))
            .expect("a drawn frame");
        let buffer = terminal.backend().buffer();
        let mut text = String::new();
        for y in 0..buffer.area.height {
            for x in 0..buffer.area.width {
                text.push_str(buffer[(x, y)].symbol());
            }
            text.push('\n');
        }
        text
    }

    /// The rows that mention a phrase.
    fn rows_with(text: &str, needle: &str) -> Vec<String> {
        text.lines()
            .filter(|row| row.contains(needle))
            .map(str::to_string)
            .collect()
    }

    fn running(statuses: &[Status], log: &[&str]) -> App {
        let mut app = app();
        app.stage = Stage::Run;
        for (index, status) in statuses.iter().enumerate() {
            app.set_status(index, *status);
        }
        app.log = log.iter().map(|line| LogLine::parse(line)).collect();
        app
    }

    #[test]
    fn a_wide_run_screen_gives_the_log_the_whole_width() {
        let mut app = running(&[Status::Ok, Status::Running], &["* fetching packages"]);
        let long = format!("{}END", "x".repeat(120));
        app.log = (0..30)
            .map(|index| LogLine::parse(&format!("  [INFO]  {index} {long}")))
            .collect();
        let text = render(&mut app, 200, 40);
        let log = rows_with(&text, "install log")
            .first()
            .expect("no log heading")
            .clone();
        assert!(
            log.find("install log").expect("a log heading") < 20,
            "the log was pushed into a column:\n{text}"
        );
        assert!(
            text.contains("END"),
            "the log is clipped to a column:\n{text}"
        );
    }

    #[test]
    fn the_home_screen_offers_the_three_actions_and_says_what_they_do() {
        let text = render(&mut app(), 100, 30);
        assert!(text.contains("Install Caelestia"), "{text}");
        assert!(text.contains("Update this install"), "{text}");
        assert!(text.contains("Uninstall Caelestia"), "{text}");
        assert!(text.contains(theme::TAGLINE), "{text}");
        assert!(
            text.contains("██████╗"),
            "the mark is drawn where it fits: {text}"
        );
        assert!(text.contains(theme::AUTHOR), "{text}");
    }

    #[test]
    fn the_mark_is_dropped_rather_than_clipped_when_there_is_no_room_for_it() {
        let text = render(&mut app(), 62, 22);
        assert!(!text.contains("██████╗"), "{text}");
        assert!(text.contains(theme::NAME), "the name stands in: {text}");
        assert!(
            text.contains("Install Caelestia"),
            "and the launcher stays: {text}"
        );
    }

    #[test]
    fn a_terminal_too_small_says_so_instead_of_drawing_a_broken_frame() {
        let text = render(&mut app(), 40, 10);
        assert!(text.contains("terminal too small"), "{text}");
        assert!(text.contains("60x20"), "{text}");
        assert!(!text.contains("Install Caelestia"), "{text}");
    }

    #[test]
    fn the_chrome_shows_where_the_user_is_and_how_the_install_went() {
        let mut app = app();
        app.stage = Stage::Options;
        app.action = crate::app::Action::Install;
        let text = render(&mut app, 100, 30);
        assert!(text.contains("caelestia · install"), "{text}");
        assert!(text.contains("↑↓ move"), "the real keymap: {text}");
    }

    #[test]
    fn the_privilege_screen_never_shows_the_password() {
        let mut app = app();
        app.stage = Stage::Privilege;
        app.password = "hunter2".to_string();
        let problem = app.problem.clone();
        assert_eq!(problem, None);
        let text = render(&mut app, 80, 24);
        assert!(!text.contains("hunter2"), "{text}");
        assert!(
            text.contains("•••••••"),
            "and shows that something was typed: {text}"
        );
    }

    #[test]
    fn a_wrong_password_is_shown_as_a_problem_with_what_to_do_about_it() {
        let mut app = app();
        app.stage = Stage::Privilege;
        app.attempts = 3;
        app.problem = Some("Not accepted. A wrong entry now locks the account.".to_string());
        let text = render(&mut app, 100, 30);
        assert!(text.contains("Not accepted"), "{text}");
        assert!(text.contains("locks the account"), "{text}");
        assert!(text.contains("attempt 4"), "{text}");
    }

    #[test]
    fn the_wait_for_sudo_is_shown_where_the_answer_will_appear() {
        let mut app = app();
        app.stage = Stage::Privilege;
        app.password = "hunter2".to_string();
        let (sender, receiver) = std::sync::mpsc::channel();
        app.checking = Some(receiver);
        let text = render(&mut app, 100, 30);
        assert!(text.contains("Checking your password"), "{text}");
        assert!(!text.contains("hunter2"), "{text}");

        sender.send(false).expect("the answer fits");
        app.tick();
        let text = render(&mut app, 100, 30);
        assert!(text.contains("accepted"), "{text}");
        assert!(!text.contains("Checking your password"), "{text}");
    }

    #[test]
    fn every_question_ends_with_a_row_that_goes_on() {
        let mut app = app();
        app.stage = Stage::Options;
        for question in 0..menu::question_count() {
            app.question = question;
            app.cursor = 0;
            let text = render(&mut app, 100, 30);
            assert!(
                text.contains("continue"),
                "question {question} has no row to go on with:\n{text}"
            );
        }
    }

    #[test]
    fn the_longest_question_shows_its_last_row_and_the_way_on_at_the_smallest_size() {
        let mut app = app();
        app.stage = Stage::Options;
        app.question = (0..menu::question_count())
            .find(|index| {
                menu::items_of(*index)
                    .iter()
                    .any(|entry| entry.id() == menu::PRESET_THEME_APPS)
            })
            .expect("the app question is in the tree");
        let text = render(&mut app, 60, 20);
        assert!(text.contains("Firefox"), "the last row was clipped: {text}");
        assert!(text.contains("continue"), "and the way on: {text}");
    }

    #[test]
    fn the_app_question_offers_every_app_even_on_a_machine_that_has_none() {
        let mut app = app().with_apps(crate::apps::Apps::default());
        app.stage = Stage::Options;
        app.question = (0..menu::question_count())
            .find(|index| {
                menu::items_of(*index)
                    .iter()
                    .any(|entry| entry.id() == menu::PRESET_THEME_APPS)
            })
            .expect("the app question is in the tree");

        let text = render(&mut app, 100, 30);
        for id in crate::apps::Apps::IDS {
            let entry = menu::every_entry()
                .into_iter()
                .find(|entry| entry.id() == id)
                .expect("every app id is a row");
            assert!(
                text.contains(entry.title()),
                "{} is missing: {text}",
                entry.title()
            );
        }
        assert!(text.contains("All of them"), "{text}");
        assert!(text.contains("None of them"), "{text}");
    }

    #[test]
    fn every_option_row_starts_at_the_same_column() {
        let mut app = app();
        app.stage = Stage::Options;
        app.question = 0;
        let rows = option_rows(&app, 96);
        let titles: Vec<&'static str> = app
            .current_entries()
            .iter()
            .map(|entry| entry.title())
            .collect();
        let columns: Vec<usize> = rows
            .iter()
            .zip(titles)
            .map(|(row, title)| {
                let text: String = row.spans.iter().map(|span| span.content.as_ref()).collect();
                let bytes = text.find(title).expect("the title is on its row");
                text[..bytes].chars().count()
            })
            .collect();
        assert!(
            columns.windows(2).all(|pair| pair[0] == pair[1]),
            "the rows start at different columns: {columns:?}"
        );
    }

    #[test]
    fn the_options_screen_puts_values_in_a_column() {
        let mut app = app();
        app.stage = Stage::Options;
        let rows = option_rows(&app, 96);
        assert_eq!(rows.len(), app.current_entries().len());

        let widths: Vec<usize> = rows.iter().map(Line::width).collect();
        assert!(
            widths.windows(2).all(|pair| pair[0] == pair[1]),
            "the rows are not a grid: {widths:?}"
        );

        let text = render(&mut app, 100, 30);
        for entry in app.current_entries() {
            assert!(text.contains(entry.title()), "{}", entry.title());
        }
    }

    #[test]
    fn a_value_that_is_too_long_for_its_column_is_marked_rather_than_silently_cut() {
        let mut app = app();
        app.stage = Stage::Options;
        let narrow = option_rows(&app, 24);
        let drawn: Vec<String> = narrow.iter().map(|row| row.to_string()).collect();
        assert!(
            drawn.iter().any(|row| row.contains('…')),
            "a clipped name says so: {drawn:?}"
        );
        let widths: Vec<usize> = narrow.iter().map(Line::width).collect();
        assert!(
            widths.windows(2).all(|pair| pair[0] == pair[1]),
            "{widths:?}"
        );
    }

    #[test]
    fn the_options_screen_marks_what_was_changed_and_shows_the_help() {
        let mut app = app();
        app.stage = Stage::Options;
        app.question = 0;
        app.cursor = 0;
        let before = render(&mut app, 100, 30);
        assert!(!before.contains("•"), "{before}");
        assert!(
            !before.contains("Replace the login screen"),
            "a row's label is not repeated as its help: {before}"
        );

        app.cursor = 0;
        let shot = render(&mut app, 100, 30);
        assert!(
            shot.contains("Login screen colours"),
            "the first row is the login screen: {shot}"
        );
        assert!(
            shot.contains("Plasma Login draws its own screen"),
            "the row's own help is on screen:\n{shot}"
        );
        app.cursor = 1;
        assert!(
            !render(&mut app, 100, 30).contains("Plasma Login draws its own screen"),
            "and it goes with the cursor"
        );

        app.key(KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE));
        let after = render(&mut app, 100, 30);
        assert!(after.contains("•"), "the change is marked: {after}");
        assert!(
            after.contains("changed"),
            "and the question says so: {after}"
        );
        assert!(after.contains(menu::question(0)), "{after}");
    }
    #[test]
    fn the_run_screen_names_the_step_it_is_on_and_how_far_along_it_is() {
        let mut app = running(
            &[Status::Ok, Status::Warn, Status::Running],
            &["  [INFO]  syncing databases", "  [WARN]  slow mirror"],
        );
        let text = render(&mut app, 100, 30);
        let step = STEPS[2].name;
        assert!(text.contains(step), "the step it is on: {text}");
        assert!(text.contains('%'), "how far along: {text}");
        assert!(
            text.contains("1 warning"),
            "what did not go cleanly: {text}"
        );
        assert!(
            !text.contains("PENDING") && !text.contains("RUNNING"),
            "the states are no longer spelled out: {text}"
        );
    }

    #[test]
    fn the_run_screen_gives_the_log_the_room_it_never_had() {
        let mut app = running(&[Status::Running], &["  [INFO]  one", "  [INFO]  two"]);
        app.log = (0..40)
            .map(|index| LogLine::parse(&format!("  [INFO]  line {index}")))
            .collect();
        let text = render(&mut app, 80, 24);
        let shown = (0..40)
            .filter(|index| text.contains(&format!("line {index}")))
            .count();
        assert!(
            shown >= 6,
            "only {shown} lines of log made it onto the screen: {text}"
        );
    }

    #[test]
    fn the_log_pane_says_where_it_is_and_whether_it_follows() {
        let mut app = running(&[Status::Running], &[]);
        app.log = (0..60)
            .map(|index| LogLine::parse(&format!("  [INFO]  line {index}")))
            .collect();
        let following = render(&mut app, 100, 30);
        assert!(following.contains("FOLLOW"), "{following}");
        assert!(
            following.contains(" of 60"),
            "the position is shown: {following}"
        );

        app.log_follow = false;
        app.log_scroll = 4;
        let paused = render(&mut app, 100, 30);
        assert!(paused.contains("PAUSED"), "{paused}");
    }

    #[test]
    fn the_log_is_classified_so_a_failure_does_not_look_like_chatter() {
        let mut app = running(
            &[Status::Running],
            &[
                "  [INFO]  reading packages",
                "  [WARN]  slow mirror",
                "  [ERR]   package not found",
            ],
        );
        let text = render(&mut app, 100, 30);

        let warning = rows_with(&text, "slow mirror");
        assert!(
            warning.iter().any(|row| row.contains('!')),
            "the warning has its own glyph: {warning:?}"
        );
        let failure = rows_with(&text, "package not found");
        assert!(
            failure.iter().any(|row| row.contains('✗')),
            "and the failure its own: {failure:?}"
        );
        let chatter = rows_with(&text, "reading packages");
        assert!(
            chatter
                .iter()
                .all(|row| !row.contains('!') && !row.contains('✗')),
            "and chatter stays plain: {chatter:?}"
        );
    }

    #[test]
    fn the_log_counts_what_went_wrong_and_offers_to_walk_it() {
        let mut app = running(
            &[Status::Running],
            &[
                "  [INFO]  chatter",
                "  [WARN]  a slow mirror",
                "  [ERR]   a package failed",
            ],
        );
        let text = render(&mut app, 166, 48);
        assert!(text.contains("2 problems to jump to"), "{text}");
        assert!(text.contains("n next problem"), "{text}");
        assert!(text.contains("p previous"), "{text}");
    }

    #[test]
    fn a_long_log_marks_the_lines_that_went_wrong_on_its_scrollbar() {
        let mut app = running(&[Status::Running], &["  [INFO]  chatter"]);
        app.log = (0..300)
            .map(|line| {
                if line == 150 {
                    LogLine::parse("  [ERR]   a package failed")
                } else {
                    LogLine::parse(&format!("  [INFO]  line {line}"))
                }
            })
            .collect();
        let text = render(&mut app, 100, 40);
        let layout = ui::run_layout(ui::body(app.area));
        let column = (layout.log.x + layout.log.width - 1) as usize;
        let bar: String = text
            .lines()
            .filter_map(|row| row.chars().nth(column))
            .collect();
        assert!(bar.contains('•'), "the failure is marked: {bar:?}");
        assert!(bar.contains('█'), "and the window is still drawn: {bar:?}");
    }

    #[test]
    fn the_log_can_be_narrowed_to_the_lines_that_matter() {
        let mut app = running(
            &[Status::Running],
            &["  [INFO]  chatter", "  [ERR]   a package failed"],
        );
        app.log_loud_only = true;
        let text = render(&mut app, 100, 30);
        assert!(text.contains("a package failed"), "{text}");
        assert!(text.contains("warnings and errors only"), "{text}");
        assert!(!text.contains("chatter"), "{text}");
    }

    #[test]
    fn the_failure_card_says_what_to_do_and_what_each_answer_costs() {
        let mut app = app();
        app.stage = Stage::Run;
        app.set_status(0, Status::Failed);
        app.failure = Some(crate::app::FailureCard::new(
            &STEPS[0],
            &[LogLine::parse("  [ERR]   the mirror is unreachable")],
        ));
        app.overlay = Some(Overlay::Failure);
        let text = render(&mut app, 100, 32);
        assert!(text.contains("did not finish"), "{text}");
        assert!(text.contains("Update system"), "{text}");
        assert!(text.contains("the mirror is unreachable"), "{text}");
        assert!(
            text.contains("caelestia-install step 00a-system-update"),
            "{text}"
        );
        for choice in crate::app::FailureCard::CHOICES {
            assert!(text.contains(choice.label), "{}", choice.label);
        }
        assert!(text.contains("leaves the run paused"), "{text}");
    }

    /// What a finished install left behind, as a fixture: the screen that shows it must not read
    /// this machine's home to say what the wizard did.
    fn landed() -> crate::app::Landed {
        crate::app::Landed {
            version: Some("v2.6.3".to_string()),
            commit: Some("9f2a1cb1234567890".to_string()),
            branch: Some("main".to_string()),
            shell: PathBuf::from("/home/tester/.config/quickshell/caelestia"),
            configs: PathBuf::from("/home/tester/.config/caelestia"),
            backup: Some(PathBuf::from(
                "/home/tester/caelestia/backups/20240101_120000",
            )),
        }
    }

    #[test]
    fn the_summary_says_what_is_installed_and_what_to_do_with_it() {
        let mut app = app();
        app.stage = Stage::Done;
        app.fill_statuses(Status::Ok);
        app.landed = Some(landed());
        let text = render(&mut app, 110, 32);

        assert!(text.contains("The install finished."), "{text}");
        assert!(
            text.contains("v2.6.3 \u{00b7} 9f2a1cb \u{00b7} main"),
            "the version, the revision and the channel: {text}"
        );
        assert!(text.contains("quickshell/caelestia"), "{text}");
        assert!(text.contains("caelestia/backups/20240101_120000"), "{text}");
        assert!(text.contains("/tmp/install.log"), "{text}");
        assert!(text.contains("log out and back in"), "{text}");
        assert!(text.contains("caelestia-install --update"), "{text}");
        assert!(text.contains("caelestia-install --uninstall"), "{text}");
        assert!(!text.contains("warning"), "nothing to warn about: {text}");
    }

    #[test]
    fn the_summary_is_not_a_score_of_the_run() {
        let mut app = app();
        app.stage = Stage::Done;
        app.fill_statuses(Status::Ok);
        app.landed = Some(landed());
        let text = render(&mut app, 110, 32);

        assert!(!text.contains("steps finished"), "{text}");
        assert!(!text.contains(&format!("of {}", STEPS.len())), "{text}");
    }

    #[test]
    fn the_summary_does_not_offer_a_retry_it_cannot_do() {
        let mut app = app();
        app.stage = Stage::Done;
        app.fill_statuses(Status::Ok);
        app.landed = Some(landed());
        let text = render(&mut app, 110, 32);
        assert!(!text.contains("retry"), "nothing to retry: {text}");

        app.set_status(3, Status::Ignored);
        let text = render(&mut app, 110, 32);
        assert!(
            text.contains("r retry failed"),
            "the way out of an unfinished run: {text}"
        );
    }

    #[test]
    fn the_summary_offers_to_log_out_for_the_reader() {
        let mut app = app();
        app.stage = Stage::Done;
        app.fill_statuses(Status::Ok);
        app.landed = Some(landed());
        let text = render(&mut app, 110, 32);

        assert!(
            text.contains("Log out and apply Caelestia (o)"),
            "the way to apply it is a button with its key in it: {text}"
        );
        assert!(!text.contains("o log out"), "{text}");
        assert!(text.contains("log out and back in"), "{text}");
    }

    #[test]
    fn the_logout_button_is_filled_like_every_other_button() {
        let mut app = color_app();
        app.stage = Stage::Done;
        app.fill_statuses(Status::Ok);
        app.landed = Some(landed());
        let accent = app.theme.color(Role::Accent);
        let buffer = cells(&mut app, 110, 32);
        let row = row_of(&buffer, LOGOUT_BUTTON);
        let text: String = (0..buffer.area.width)
            .map(|x| buffer[(x, row)].symbol())
            .collect();
        let start = text.find(LOGOUT_BUTTON).expect("the label is on its row") as u16;
        for x in start..start + LOGOUT_BUTTON.chars().count() as u16 {
            assert_eq!(
                buffer[(x, row)].bg,
                accent,
                "column {x} of the button is not filled: {text}"
            );
        }
    }

    #[test]
    fn the_summary_does_not_offer_a_logout_it_will_not_do() {
        let mut app = app();
        app.stage = Stage::Done;
        app.fill_statuses(Status::Ok);
        app.set_status(4, Status::Failed);
        app.landed = Some(landed());
        let text = render(&mut app, 110, 32);

        assert!(!text.contains("Log out and apply Caelestia"), "{text}");
        assert!(!text.contains("o log out"), "{text}");
    }

    #[test]
    fn the_logout_card_says_what_ending_the_session_does() {
        let mut app = app();
        app.stage = Stage::Done;
        app.fill_statuses(Status::Ok);
        app.ask(crate::app::Confirm::LogOut);
        let text = render(&mut app, 100, 30);

        assert!(text.contains("Log out now?"), "{text}");
        assert!(
            text.contains("Save what you") && text.contains("working on first"),
            "the reader is told what it costs them: {text}"
        );
        assert!(text.contains("Log out (y)"), "{text}");
        assert!(text.contains("Not now (n)"), "{text}");
    }

    #[test]
    fn the_summary_does_not_read_as_a_success_when_it_was_not() {
        let mut app = app();
        app.stage = Stage::Done;
        app.fill_statuses(Status::Ok);
        app.set_status(2, Status::Failed);
        app.set_status(5, Status::Ignored);
        app.landed = Some(landed());
        let text = render(&mut app, 110, 32);
        assert!(text.contains("1 failed step"), "{text}");
        assert!(
            text.contains("caelestia-install step"),
            "the way to retry: {text}"
        );
        assert!(text.contains(STEPS[2].name), "{text}");
        assert!(text.contains(STEPS[5].name), "{text}");
        assert!(text.contains("nothing was rolled back"), "{text}");
        assert!(
            !text.contains("log out and back in"),
            "a run that did not finish is not one to log out of: {text}"
        );
    }

    #[test]
    fn the_summary_counts_read_like_english() {
        let mut app = app();
        app.stage = Stage::Done;
        app.fill_statuses(Status::Ok);
        app.set_status(1, Status::Warn);
        let text = render(&mut app, 110, 32);
        assert!(text.contains("finished with 1 warning."), "{text}");
        assert!(!text.contains("1 steps"), "no plurals of one: {text}");
        assert!(!text.contains("1 warnings"), "{text}");
    }

    #[test]
    fn the_summary_says_a_warning_once_and_names_the_step_that_raised_it() {
        let mut app = app();
        app.stage = Stage::Done;
        app.fill_statuses(Status::Ok);
        app.set_status(1, Status::Warn);
        let text = render(&mut app, 110, 32);

        assert_eq!(
            text.matches("1 warning").count(),
            1,
            "one statement of the count, not three: {text}"
        );
        assert!(
            text.lines()
                .any(|line| line.contains("warned") && line.contains(STEPS[1].name)),
            "the step that warned is the useful part: {text}"
        );
    }

    #[test]
    fn the_confirm_card_says_what_will_happen_to_a_running_step() {
        let mut app = app();
        app.stage = Stage::Run;
        app.set_status(0, Status::Running);
        app.ask(crate::app::Confirm::Quit);
        let text = render(&mut app, 100, 30);
        assert!(text.contains("Stop the install here?"), "{text}");
        assert!(
            text.contains("A step that is running") && text.contains("stopped first"),
            "true whether or not a step happens to be running: {text}"
        );
        assert!(text.contains("Stop (y)"), "the answer is a button: {text}");
        assert!(
            text.contains("Keep going (n)"),
            "and so is the one that keeps it, with the letter that picks it: {text}"
        );
        assert_eq!(app.confirm_focus, 1);
    }

    #[test]
    fn the_confirm_card_names_the_action_it_hands_over_to() {
        let mut app = app();
        app.confirm = Some(crate::app::Confirm::Handoff(crate::app::Action::Uninstall));
        app.overlay = Some(Overlay::Confirm);
        let text = render(&mut app, 100, 30);
        assert!(text.contains("Uninstall Caelestia"), "{text}");
        assert!(text.contains("hands the terminal over"), "{text}");
    }

    #[test]
    fn the_keymap_overlay_lists_the_keys_the_screens_answer_to() {
        let mut app = app();
        app.overlay = Some(Overlay::Keys);
        let text = render(&mut app, 100, 30);
        assert!(text.contains("KEYS"), "{text}");
        for key in ["ctrl+c", "?", "w", "l", "r / R"] {
            assert!(text.contains(key), "{key} is missing from {text}");
        }
    }

    #[test]
    fn the_full_screen_log_leaves_the_run_screen_behind() {
        let mut app = running(&[Status::Running], &[]);
        app.log = (0..100)
            .map(|index| LogLine::parse(&format!("  [INFO]  line {index}")))
            .collect();
        app.overlay = Some(Overlay::Log);
        let text = render(&mut app, 100, 34);
        assert!(text.contains("INSTALL LOG"), "{text}");
        let shown = (0..100)
            .filter(|index| text.contains(&format!("line {index}")))
            .count();
        assert!(shown > 20, "the overlay is the whole screen: {shown}");
    }

    #[test]
    fn every_screen_draws_something_at_the_smallest_size_it_accepts() {
        let stages = [
            Stage::Home,
            Stage::Privilege,
            Stage::Options,
            Stage::Auth,
            Stage::Run,
            Stage::Done,
        ];
        for stage in stages {
            let mut app = app();
            app.stage = stage;
            app.set_status(0, Status::Running);
            let text = render(&mut app, ui::MIN_WIDTH, ui::MIN_HEIGHT);
            assert!(!text.trim().is_empty(), "{stage:?} drew nothing");
            assert!(
                text.contains("caelestia"),
                "{stage:?} lost its header: {text}"
            );
        }
    }

    #[test]
    fn the_frame_never_spills_out_of_the_terminal() {
        for (width, height) in [(60u16, 20u16), (80, 24), (100, 30), (200, 50)] {
            let mut app = app();
            app.stage = Stage::Run;
            app.fill_statuses(Status::Ok);
            app.failure = Some(crate::app::FailureCard::new(&STEPS[0], &[]));
            app.overlay = Some(Overlay::Failure);
            let text = render(&mut app, width, height);
            for row in text.lines() {
                assert!(
                    row.chars().count() <= width as usize,
                    "a row is wider than the terminal: {row:?}"
                );
            }
            assert_eq!(text.lines().count(), height as usize);
        }
    }

    #[test]
    fn the_keymap_the_screen_shows_is_the_keymap_it_answers_to() {
        let mut app = app();
        app.stage = Stage::Home;
        app.home = 1;
        let text = render(&mut app, 100, 30);
        assert!(text.contains("choose"), "{text}");
        assert!(text.contains("quit"), "{text}");

        let before = app.theme.mode;
        assert_eq!(
            app.key(crossterm::event::KeyEvent::new(
                crossterm::event::KeyCode::Char('t'),
                crossterm::event::KeyModifiers::NONE,
            )),
            Flow::Continue
        );
        assert_ne!(app.theme.mode, before, "'t theme' did not do what it says");
    }
}
