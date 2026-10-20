//! Which step runs next, how it is started, and what its exit means - in one place.
//!
//! The wizard and the unattended run both walk the same table, apply the same skip rules and turn
//! an exit status into the same `Status`. They differ only in what they do with the events: the
//! wizard draws a log pane and opens a failure card, the unattended run prints and stops. Keeping
//! the policy here means the two cannot disagree about what a step did.

use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use crate::journal::Journal;
use crate::menu::Answers;
use crate::runner::Running;
use crate::steps::{self, STEPS, Status, Step};

/// How long a pretend run says a step took.
const PRETEND_DURATION: Duration = Duration::from_secs(2);

/// Something a session did, in the order it did it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Event {
    /// The step did not run: it is switched off, or a resumed run already finished it.
    Skipped {
        index: usize,
        reason: SkipReason,
    },
    Started {
        index: usize,
    },
    /// Output the step produced, as it arrived.
    Output {
        index: usize,
        line: String,
    },
    Finished {
        index: usize,
        status: Status,
    },
    /// The step could not be started or watched, so it never reached a status of its own.
    Broken {
        index: usize,
        problem: String,
    },
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SkipReason {
    /// A switch in the answers says this step has nothing to do.
    SwitchedOff,
    /// A resumed run found it in the journal.
    AlreadyDone,
}

pub struct Session {
    bundle: PathBuf,
    log: PathBuf,
    environment: Vec<(&'static str, String)>,
    answers: Answers,
    steps: &'static [Step],
    statuses: Vec<Status>,
    durations: Vec<Option<Duration>>,
    cursor: usize,
    running: Option<Running>,
    started: Option<Instant>,
    resume: Option<Journal>,
    pretend: bool,
}

impl Session {
    /// A run over the whole table, in table order. The table is the install's: an update repairs a
    /// different list through its own executor, so a session is never handed a subset.
    pub fn new(
        bundle: &Path,
        log: PathBuf,
        environment: Vec<(&'static str, String)>,
        answers: &Answers,
    ) -> Self {
        Self {
            bundle: bundle.to_path_buf(),
            log,
            environment,
            answers: answers.clone(),
            steps: &STEPS,
            statuses: vec![Status::Pending; STEPS.len()],
            durations: vec![None; STEPS.len()],
            cursor: 0,
            running: None,
            started: None,
            resume: None,
            pretend: false,
        }
    }

    /// Continue a run that stopped: a step the journal finished is not run again.
    pub fn resuming(mut self, journal: Journal) -> Self {
        self.resume = Some(journal);
        self
    }

    /// Walk the table without running anything, for a test that needs the shape of a run.
    #[cfg(test)]
    pub fn rehearse(&mut self, on: bool) {
        self.pretend = on;
    }

    pub fn steps(&self) -> &'static [Step] {
        self.steps
    }

    pub fn status(&self, index: usize) -> Status {
        self.statuses.get(index).copied().unwrap_or(Status::Pending)
    }

    pub fn statuses(&self) -> &[Status] {
        &self.statuses
    }

    /// The step being run, or the one that will be started next.
    #[cfg(test)]
    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn is_running(&self) -> bool {
        self.running.is_some()
    }

    pub fn is_complete(&self) -> bool {
        self.running.is_none() && self.cursor >= self.steps.len()
    }

    /// Record a status by hand: the wizard's retry, skip and ignore answers land here.
    /// Set a step's status. A step that goes back to `Pending` is going to run again, so the time
    /// it took before is no longer its time.
    pub fn set_status(&mut self, index: usize, status: Status) {
        let pending = status == Status::Pending;
        if let Some(slot) = self.statuses.get_mut(index) {
            *slot = status;
        }
        if let (true, Some(slot)) = (pending, self.durations.get_mut(index)) {
            *slot = None;
        }
    }

    /// Give every step the same status, for a test that only needs the shape.
    #[cfg(test)]
    pub fn fill(&mut self, status: Status) {
        self.statuses = vec![status; self.steps.len()];
    }

    pub fn cursor_to(&mut self, index: usize) {
        self.cursor = index.min(self.steps.len());
    }

    /// How long the run has taken so far, counting the step in flight.
    pub fn total_elapsed(&self) -> Duration {
        self.durations.iter().flatten().sum::<Duration>()
            + self.started.map(|at| at.elapsed()).unwrap_or_default()
    }

    /// Start the table again from the top, with the answers and environment of this run.
    pub fn reset(&mut self, environment: Vec<(&'static str, String)>, answers: &Answers) {
        self.environment = environment;
        self.answers = answers.clone();
        self.statuses = vec![Status::Pending; self.steps.len()];
        self.durations = vec![None; self.steps.len()];
        self.cursor = 0;
        self.running = None;
        self.started = None;
        self.resume = None;
    }

    /// Start whatever comes next, skipping what the answers switch off and what a resume already
    /// finished. Returns once a step is running, or once there is nothing left to start.
    pub fn advance(&mut self) -> Vec<Event> {
        let mut events = Vec::new();
        while self.cursor < self.steps.len() {
            let index = self.cursor;
            if self.status(index) != Status::Pending {
                self.cursor += 1;
                continue;
            }

            if steps::is_skipped(&self.steps[index], &self.answers) {
                self.statuses[index] = Status::Skipped;
                self.durations[index] = Some(Duration::ZERO);
                if let Some(journal) = self.resume.as_mut() {
                    journal.record(self.steps[index].id, Status::Skipped, 0);
                }
                events.push(Event::Skipped {
                    index,
                    reason: SkipReason::SwitchedOff,
                });
                self.cursor += 1;
                continue;
            }

            if self
                .resume
                .as_ref()
                .is_some_and(|journal| journal.done(self.steps[index].id))
            {
                self.statuses[index] = Status::Skipped;
                self.durations[index] = Some(Duration::ZERO);
                events.push(Event::Skipped {
                    index,
                    reason: SkipReason::AlreadyDone,
                });
                self.cursor += 1;
                continue;
            }

            if self.pretend {
                self.statuses[index] = Status::Ok;
                self.durations[index] = Some(PRETEND_DURATION);
                events.push(Event::Started { index });
                events.push(Event::Finished {
                    index,
                    status: Status::Ok,
                });
                self.cursor += 1;
                continue;
            }

            self.statuses[index] = Status::Running;
            self.started = Some(Instant::now());
            match Running::spawn_native(
                &self.bundle,
                self.steps[index].id,
                &self.log,
                &self.environment,
            ) {
                Ok(running) => {
                    self.running = Some(running);
                    events.push(Event::Started { index });
                    return events;
                }
                Err(problem) => {
                    self.settle(index, Status::Failed);
                    events.push(Event::Broken { index, problem });
                    return events;
                }
            }
        }
        events
    }

    /// Take whatever output the running step has produced, and settle it if it has finished.
    pub fn pump(&mut self) -> Vec<Event> {
        let Some(mut running) = self.running.take() else {
            return Vec::new();
        };
        let index = self.cursor;
        let mut events = Vec::new();
        let mut fresh = Vec::new();
        running.drain(&mut fresh);
        for line in fresh {
            events.push(Event::Output { index, line });
        }

        match running.poll() {
            Ok(Some(exit)) => {
                let status = running.outcome(exit);
                self.settle(index, status);
                events.push(Event::Finished { index, status });
            }
            Ok(None) => self.running = Some(running),
            Err(problem) => {
                self.settle(index, Status::Failed);
                events.push(Event::Broken { index, problem });
            }
        }
        events
    }

    fn settle(&mut self, index: usize, status: Status) {
        self.statuses[index] = status;
        self.durations[index] = Some(
            self.started
                .take()
                .map(|at| at.elapsed())
                .unwrap_or_default(),
        );
        if let Some(journal) = self.resume.as_mut() {
            let seconds = self.durations[index].unwrap_or_default().as_secs();
            journal.record(self.steps[index].id, status, seconds);
        }
    }

    /// Run the step the cursor is on again, whatever the last run said about it.
    pub fn retry(&mut self) {
        if self.cursor < self.steps.len() {
            self.set_status(self.cursor, Status::Pending);
        }
    }

    /// Move past the current step without running it.
    pub fn skip_current(&mut self) {
        if self.cursor < self.steps.len() {
            self.statuses[self.cursor] = Status::Skipped;
            self.durations[self.cursor] = Some(Duration::ZERO);
            self.cursor += 1;
        }
    }

    /// Move past the current step, keeping the failure on the record.
    pub fn ignore_current(&mut self) {
        if self.cursor < self.steps.len() {
            self.statuses[self.cursor] = Status::Ignored;
            self.durations[self.cursor] = Some(Duration::ZERO);
            self.cursor += 1;
        }
    }

    /// Stop whatever is running and settle it as failed: the user asked the run to stop.
    pub fn abandon(&mut self) {
        if let Some(mut running) = self.running.take() {
            running.stop();
            let index = self.cursor;
            self.settle(index, Status::Failed);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::menu::Answers;

    fn answers(pairs: &[(&'static str, &str)]) -> Answers {
        pairs
            .iter()
            .map(|(key, value)| (*key, value.to_string()))
            .collect()
    }

    fn a_session(answers: &Answers) -> Session {
        let mut session = Session::new(
            Path::new("/bundle"),
            PathBuf::from("/tmp/caelestia-test.log"),
            Vec::new(),
            answers,
        );
        session.rehearse(true);
        session
    }

    #[test]
    fn a_switched_off_step_is_skipped_rather_than_run() {
        let mut session = a_session(&answers(&[("SKIP_SYSTEM_UPDATE", "true")]));
        let events = session.advance();
        assert_eq!(
            events.first(),
            Some(&Event::Skipped {
                index: 0,
                reason: SkipReason::SwitchedOff
            })
        );
        assert_eq!(session.status(0), Status::Skipped);
    }

    #[test]
    fn a_pretend_run_walks_the_whole_table_and_finishes() {
        let mut session = a_session(&answers(&[]));
        loop {
            let events = session.advance();
            if events.is_empty() {
                break;
            }
        }
        assert!(session.is_complete());
        assert!(
            session
                .statuses()
                .iter()
                .all(|status| !matches!(status, Status::Pending | Status::Running)),
            "{:?}",
            session.statuses()
        );
        assert!(session.statuses().contains(&Status::Ok));
    }

    #[test]
    fn a_journal_that_finished_a_step_stops_it_running_twice() {
        let work =
            crate::tmp::TempDir::new("caelestia-session-test").expect("a temporary directory");
        let path = work.path().join("plan.journal");
        std::fs::write(&path, "00a-system-update|ok|3\n").expect("a journal");

        let mut session = a_session(&answers(&[])).resuming(Journal::at(path));
        let events = session.advance();
        assert!(
            events.contains(&Event::Skipped {
                index: 0,
                reason: SkipReason::AlreadyDone
            }),
            "{events:?}"
        );
    }

    #[test]
    fn skipping_and_ignoring_move_the_cursor_past_the_step() {
        let mut session = a_session(&answers(&[]));
        session.skip_current();
        assert_eq!(session.cursor(), 1);
        assert_eq!(session.status(0), Status::Skipped);

        session.ignore_current();
        assert_eq!(session.cursor(), 2);
        assert_eq!(session.status(1), Status::Ignored);
    }

    #[test]
    fn a_retry_puts_the_failed_step_back_to_pending() {
        let mut session = a_session(&answers(&[]));
        session.cursor_to(3);
        session.set_status(3, Status::Failed);
        session.retry();
        assert_eq!(session.status(3), Status::Pending);
        assert_eq!(session.cursor(), 3);
    }
}
