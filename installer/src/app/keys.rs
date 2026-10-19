//! The keys each screen answers to, and the stage changes they cause.
//!
//! Split out of `app` because this is the part that grows with every screen, and none of it is
//! about the run itself: it reads the state, asks for the next stage, and reports what the
//! frame after this key should say.

use super::*;

impl App {
    pub(super) fn home_key(&mut self, key: KeyEvent) -> Flow {
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => self.home = self.step_action(-1),
            KeyCode::Down | KeyCode::Char('j') => self.home = self.step_action(1),
            KeyCode::Char('?') => self.overlay = Some(Overlay::Keys),
            KeyCode::Enter | KeyCode::Char(' ') => return self.choose_action(),
            KeyCode::Esc | KeyCode::Char('q') => return Flow::Done(Outcome::Canceled),
            _ => {}
        }
        Flow::Continue
    }

    fn step_action(&self, direction: isize) -> usize {
        let count = Action::ALL.len() as isize;
        let mut index = self.home as isize;
        for _ in 0..count {
            index = (index + direction).rem_euclid(count);
            if self.action_enabled(Action::ALL[index as usize]) {
                return index as usize;
            }
        }
        self.home
    }

    fn choose_action(&mut self) -> Flow {
        let action = Action::ALL[self.home];
        if !self.action_enabled(action) {
            self.notice = Some(Action::unavailable(action).to_string());
            return Flow::Continue;
        }
        self.action = action;
        match action {
            Action::Install => {
                self.show_detection();
                self.stage = Stage::Detect;
            }
            Action::Update | Action::Uninstall => {
                self.ask(Confirm::Handoff(action));
            }
        }
        Flow::Continue
    }

    fn show_detection(&mut self) {
        self.refresh_detection();
        self.reinstall_acknowledged = false;
    }

    pub(super) fn detect_key(&mut self, key: KeyEvent) -> Flow {
        match key.code {
            KeyCode::Enter | KeyCode::Char(' ') => {
                if !self.detection.blockers().is_empty() {
                    self.problem = Some("Sort out the above first.".to_string());
                    return Flow::Continue;
                }
                if self.installed() {
                    self.reinstall_acknowledged = true;
                }
                self.problem = None;
                self.question = 0;
                self.cursor = 0;
                self.stage = Stage::Options;
            }
            KeyCode::Char('u') if self.installed() => {
                self.home = Action::ALL
                    .iter()
                    .position(|action| *action == Action::Update)
                    .unwrap_or(self.home);
                self.action = Action::Update;
                self.ask(Confirm::Handoff(Action::Update));
            }
            KeyCode::Char('?') => self.overlay = Some(Overlay::Keys),
            KeyCode::Esc => {
                self.problem = None;
                self.stage = Stage::Home;
            }
            _ => {}
        }
        Flow::Continue
    }

    pub(super) fn privilege_key(&mut self, key: KeyEvent) -> Flow {
        if self.checking.is_some() {
            return Flow::Continue;
        }
        if self.auth == Some(AuthMethod::Fingerprint) {
            match key.code {
                KeyCode::Enter | KeyCode::Char(' ') => {
                    self.start_check();
                    Flow::Continue
                }
                KeyCode::Esc => {
                    self.leave_the_gate();
                    Flow::Continue
                }
                _ => Flow::Continue,
            }
        } else {
            self.password_key(key)
        }
    }

    fn password_key(&mut self, key: KeyEvent) -> Flow {
        match key.code {
            KeyCode::Enter => {
                self.start_check();
                Flow::Continue
            }
            KeyCode::Char('r') if self.attempts > 0 => {
                self.start_check();
                Flow::Continue
            }
            KeyCode::Backspace => {
                self.password.pop();
                Flow::Continue
            }
            KeyCode::Esc => {
                self.leave_the_gate();
                Flow::Continue
            }
            KeyCode::Char(character) if !key.modifiers.contains(KeyModifiers::CONTROL) => {
                if !character.is_control() {
                    self.password.push(character);
                }
                Flow::Continue
            }
            _ => Flow::Continue,
        }
    }

    fn start_check(&mut self) {
        if self.checking.is_some() {
            return;
        }
        let fingerprint = self.auth == Some(AuthMethod::Fingerprint);
        if !fingerprint {
            if self.password.is_empty() {
                self.problem = Some("Type the password first.".to_string());
                return;
            }
            if !self.pretend && privileges::locked_out() {
                self.problem =
                    Some("Locked out for a few minutes after too many tries.".to_string());
                return;
            }
        }
        let (sender, receiver) = std::sync::mpsc::channel();
        if self.pretend {
            let _ = sender.send(true);
        } else {
            let secret = self.password.clone();
            thread::spawn(move || {
                let accepted = if fingerprint {
                    privileges::accepts_fingerprint()
                } else {
                    privileges::accepts(&secret)
                };
                let _ = sender.send(accepted);
            });
        }
        self.checking = Some(receiver);
        self.problem = None;
    }

    pub(super) fn check_password(&mut self) {
        let Some(receiver) = self.checking.as_ref() else {
            return;
        };
        match receiver.try_recv() {
            Ok(true) => {
                self.checking = None;
                self.accept_password();
            }
            Ok(false) => {
                self.checking = None;
                self.password.clear();
                if self.auth == Some(AuthMethod::Fingerprint) {
                    let problem = self.refusal();
                    self.fingerprint_spent = true;
                    self.cursor = 0;
                    self.problem = Some(problem);
                    self.gate();
                    return;
                }
                self.attempts += 1;
                self.problem = Some(self.refusal());
            }
            Err(std::sync::mpsc::TryRecvError::Empty) => {}
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                self.checking = None;
                self.problem = Some("The password could not be checked.".to_string());
            }
        }
    }

    fn refusal(&self) -> String {
        if self.auth == Some(AuthMethod::Fingerprint) {
            return "No finger was recognized. Type your password instead.".to_string();
        }
        if !self.pretend && privileges::on_the_last_entry() {
            "Not accepted. A wrong entry now locks the account.".to_string()
        } else {
            "That password was not accepted.".to_string()
        }
    }

    fn accept_password(&mut self) {
        if !self.pretend {
            let credential = if self.auth == Some(AuthMethod::Fingerprint) {
                Credential::fingerprint()
            } else {
                let exe =
                    std::env::current_exe().unwrap_or_else(|_| PathBuf::from("caelestia-install"));
                Credential::install(&self.password, &exe)
            };
            match credential {
                Ok(credential) => self.credential = Some(credential),
                Err(problem) => {
                    eprintln!("caelestia-install: {problem}");
                    self.problem = Some(problem);
                    self.leave(Outcome::Internal);
                    return;
                }
            }
        }
        self.password.clear();
        self.attempts = 0;
        self.problem = None;
        self.begin_install();
    }

    pub(super) fn need_auth(&mut self) {
        if !self.needs_password {
            self.begin_install();
            return;
        }
        self.problem = None;
        self.cursor = 0;
        self.attempts = 0;
        self.gate();
    }

    fn gate(&mut self) {
        if self.auth_methods() == [AuthMethod::Password] {
            self.auth = Some(AuthMethod::Password);
            self.stage = Stage::Privilege;
        } else {
            self.auth = None;
            self.stage = Stage::Auth;
        }
    }

    fn leave_the_gate(&mut self) {
        self.password.clear();
        self.problem = None;
        self.attempts = 0;
        if self.auth_methods() == [AuthMethod::Password] {
            self.auth = None;
            self.stage = Stage::Options;
        } else {
            self.gate();
        }
    }

    pub fn auth_methods(&self) -> Vec<AuthMethod> {
        let mut methods = Vec::new();
        if self.fingerprint && !self.fingerprint_spent {
            methods.push(AuthMethod::Fingerprint);
        }
        methods.push(AuthMethod::Password);
        methods
    }

    pub(super) fn auth_key(&mut self, key: KeyEvent) -> Flow {
        let methods = self.auth_methods();
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                self.cursor = if self.cursor == 0 {
                    methods.len() - 1
                } else {
                    self.cursor - 1
                };
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.cursor = (self.cursor + 1) % methods.len().max(1);
            }
            KeyCode::Enter | KeyCode::Char(' ') => {
                let Some(method) = methods.get(self.cursor).copied() else {
                    return Flow::Continue;
                };
                self.auth = Some(method);
                self.problem = None;
                self.attempts = 0;
                match method {
                    AuthMethod::Fingerprint => {
                        self.stage = Stage::Privilege;
                        self.start_check();
                    }
                    AuthMethod::Password => self.stage = Stage::Privilege,
                }
            }
            KeyCode::Char('?') => self.overlay = Some(Overlay::Keys),
            KeyCode::Esc => {
                self.problem = None;
                self.stage = Stage::Options;
            }
            _ => {}
        }
        Flow::Continue
    }

    pub(super) fn options_key(&mut self, key: KeyEvent) -> Flow {
        let entries = self.current_entries();
        let rows = entries.len() + 1;
        self.cursor = self.cursor.min(rows - 1);
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                self.cursor = if self.cursor == 0 {
                    rows - 1
                } else {
                    self.cursor - 1
                };
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.cursor = (self.cursor + 1) % rows;
            }
            KeyCode::Char('r') => self.reset_highlighted(),
            KeyCode::Char('R') => self.reset_all(),
            KeyCode::Char('?') => self.overlay = Some(Overlay::Keys),
            KeyCode::Char(' ') | KeyCode::Right | KeyCode::Char('l') => match self.selected() {
                Some(entry) => {
                    menu::change(entry, &mut self.answers);
                }
                None => self.next_question(),
            },
            KeyCode::Left | KeyCode::Char('h') => {
                if let Some(entry) = self.selected() {
                    menu::change(entry, &mut self.answers);
                }
            }
            KeyCode::Enter => self.next_question(),
            KeyCode::Esc => self.previous_question(),
            _ => {}
        }
        Flow::Continue
    }

    pub fn on_continue(&self) -> bool {
        self.cursor >= self.current_entries().len()
    }

    pub fn next_question(&mut self) {
        if self.question + 1 < menu::question_count() {
            self.question += 1;
            self.cursor = 0;
        } else {
            self.ask(Confirm::Install);
        }
    }

    pub fn previous_question(&mut self) {
        if self.question == 0 {
            self.stage = Stage::Detect;
        } else {
            self.question -= 1;
            self.cursor = 0;
        }
    }

    fn reset_highlighted(&mut self) {
        let Some(entry) = self.selected() else {
            return;
        };
        if menu::reset(entry, &mut self.answers, &self.baseline) {
            self.notice = Some(format!("{} reset", entry.title()));
        }
    }

    fn reset_all(&mut self) {
        self.answers = self.baseline.clone();
        self.notice = Some("every option is back at its default".to_string());
    }

    pub(super) fn run_key(&mut self, key: KeyEvent) -> Flow {
        if self.paused && matches!(key.code, KeyCode::Enter | KeyCode::Char(' ')) {
            self.overlay = Some(Overlay::Failure);
            return Flow::Continue;
        }
        let rows = self.log_rows();
        match key.code {
            KeyCode::Up => self.scroll_log(-1, rows),
            KeyCode::Down => self.scroll_log(1, rows),
            KeyCode::PageUp => self.scroll_log(-(rows as isize), rows),
            KeyCode::PageDown => self.scroll_log(rows as isize, rows),
            KeyCode::Home | KeyCode::Char('g') => {
                self.log_follow = false;
                self.log_scroll = 0;
                self.log_problem = None;
            }
            KeyCode::End => {
                self.log_follow = true;
                self.log_problem = None;
            }
            KeyCode::Char('f') => {
                self.log_follow = !self.log_follow;
                self.log_problem = None;
                self.notice = Some(if self.log_follow {
                    "following the log".to_string()
                } else {
                    "log paused".to_string()
                });
            }
            KeyCode::Char('w') => self.toggle_loud_only(),
            KeyCode::Char('n') => self.jump_to_problem(true, rows),
            KeyCode::Char('p') => self.jump_to_problem(false, rows),
            KeyCode::Char('l') => self.overlay = Some(Overlay::Log),
            KeyCode::Char('?') => self.overlay = Some(Overlay::Keys),
            KeyCode::Char('q') => self.ask(Confirm::Quit),
            _ => {}
        }
        Flow::Continue
    }

    pub(super) fn scroll_log(&mut self, step: isize, rows: usize) {
        self.log_follow = false;
        self.log_problem = None;
        let furthest = self.visible_log().len().saturating_sub(rows) as isize;
        self.log_scroll = (self.log_scroll as isize + step).clamp(0, furthest) as usize;
    }

    fn toggle_loud_only(&mut self) {
        self.log_loud_only = !self.log_loud_only;
        self.log_scroll = 0;
        self.log_problem = None;
        self.notice = Some(if self.log_loud_only {
            "showing warnings and errors only".to_string()
        } else {
            "showing every line".to_string()
        });
    }

    pub(super) fn done_key(&mut self, key: KeyEvent) -> Flow {
        match key.code {
            KeyCode::Char('r') => self.retry_failed(),
            KeyCode::Char('l') => self.overlay = Some(Overlay::Log),
            KeyCode::Char('o') if self.can_log_out() => self.ask(Confirm::LogOut),
            KeyCode::Char('?') => self.overlay = Some(Overlay::Keys),
            KeyCode::Enter | KeyCode::Char(' ') | KeyCode::Esc | KeyCode::Char('q') => {
                return self.finish();
            }
            _ => {}
        }
        Flow::Continue
    }

    pub fn can_log_out(&self) -> bool {
        self.tally().unresolved() == 0
    }

    pub fn finish(&mut self) -> Flow {
        if self.tally().unresolved() > 0 {
            Flow::Done(Outcome::Failed)
        } else {
            Flow::Done(Outcome::Ok)
        }
    }

    pub(super) fn overlay_key(&mut self, overlay: Overlay, key: KeyEvent) -> Flow {
        match overlay {
            Overlay::Keys => {
                if matches!(
                    key.code,
                    KeyCode::Esc | KeyCode::Enter | KeyCode::Char(' ') | KeyCode::Char('?')
                ) {
                    self.overlay = None;
                }
                Flow::Continue
            }
            Overlay::Log => self.log_overlay_key(key),
            Overlay::Failure => self.failure_key(key),
            Overlay::Confirm => self.confirm_key(key),
        }
    }

    fn log_overlay_key(&mut self, key: KeyEvent) -> Flow {
        let rows = ui::log_overlay_rows(self.area).max(1) as usize;
        match key.code {
            KeyCode::Esc | KeyCode::Char('l') => self.overlay = None,
            KeyCode::Up => self.scroll_log(-1, rows),
            KeyCode::Down => self.scroll_log(1, rows),
            KeyCode::PageUp => self.scroll_log(-(rows as isize), rows),
            KeyCode::PageDown => self.scroll_log(rows as isize, rows),
            KeyCode::Home | KeyCode::Char('g') => {
                self.log_follow = false;
                self.log_scroll = 0;
                self.log_problem = None;
            }
            KeyCode::End | KeyCode::Char('G') => {
                self.log_follow = true;
                self.log_problem = None;
            }
            KeyCode::Char('f') => {
                self.log_follow = !self.log_follow;
                self.log_problem = None;
            }
            KeyCode::Char('w') => self.toggle_loud_only(),
            KeyCode::Char('n') => self.jump_to_problem(true, rows),
            KeyCode::Char('p') => self.jump_to_problem(false, rows),
            _ => {}
        }
        Flow::Continue
    }

    fn failure_key(&mut self, key: KeyEvent) -> Flow {
        if self.failure.is_none() {
            self.overlay = None;
            return Flow::Continue;
        }
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                if let Some(card) = self.failure.as_mut() {
                    card.choice = card.choice.saturating_sub(1);
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if let Some(card) = self.failure.as_mut() {
                    card.choice = (card.choice + 1).min(FailureCard::CHOICES.len() - 1);
                }
            }
            KeyCode::Char('r') => return self.resolve_failure(0),
            KeyCode::Char('s') => return self.resolve_failure(1),
            KeyCode::Char('i') => return self.resolve_failure(2),
            KeyCode::Char('q') => return self.resolve_failure(3),
            KeyCode::Enter | KeyCode::Char(' ') => {
                let choice = self.failure.as_ref().map_or(0, |card| card.choice);
                return self.resolve_failure(choice);
            }
            KeyCode::Esc => {
                self.overlay = None;
                self.paused = true;
            }
            _ => {}
        }
        Flow::Continue
    }

    fn resolve_failure(&mut self, choice: usize) -> Flow {
        self.overlay = None;
        self.paused = false;
        match choice {
            0 => {
                self.run.retry();
                self.start_next();
            }
            1 => {
                self.run.skip_current();
                self.start_next();
            }
            2 => {
                self.run.ignore_current();
                self.start_next();
            }
            _ => return Flow::Done(Outcome::Failed),
        }
        Flow::Continue
    }

    fn confirm_key(&mut self, key: KeyEvent) -> Flow {
        let Some(confirm) = self.confirm else {
            self.overlay = None;
            return Flow::Continue;
        };
        let buttons = confirm.buttons().len();
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                self.confirm_focus = if self.confirm_focus == 0 {
                    buttons - 1
                } else {
                    self.confirm_focus - 1
                };
                return Flow::Continue;
            }
            KeyCode::Down | KeyCode::Char('j') | KeyCode::Tab => {
                self.confirm_focus = (self.confirm_focus + 1) % buttons;
                return Flow::Continue;
            }
            KeyCode::Enter | KeyCode::Char(' ') => {}
            KeyCode::Char(letter) => match confirm.shortcut(letter) {
                Some(index) => self.confirm_focus = index,
                None => return Flow::Continue,
            },
            KeyCode::Esc => {
                self.confirm = None;
                self.overlay = None;
                return Flow::Continue;
            }
            _ => return Flow::Continue,
        }
        let chosen = self.confirm_focus.min(buttons - 1);
        self.confirm = None;
        self.overlay = None;
        if chosen != 0 {
            return Flow::Continue;
        }
        match confirm {
            Confirm::Quit => self.stop_and_leave(),
            Confirm::Handoff(action) => {
                self.stop_running();
                self.handoff = action.handoff();
                Flow::Done(Outcome::Ok)
            }
            Confirm::Install => self.confirm_install(),
            Confirm::LogOut => {
                self.handoff = Some(Handoff::LogOut);
                self.finish()
            }
        }
    }

    pub(crate) fn ask(&mut self, confirm: Confirm) {
        self.confirm = Some(confirm);
        self.confirm_focus = confirm.focus();
        self.overlay = Some(Overlay::Confirm);
    }
}
