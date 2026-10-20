//! The install without the wizard: the same plan, the same steps, no keys.
//!
//! This is what `--non-interactive` runs, and the reason it exists is a pipeline: an installer
//! that can only be driven by a person is an installer no fleet can use. It is deliberately
//! strict about what it needs - root, or a sudo ticket that is already warm - because the one
//! thing worse than refusing to run unattended is blocking on a password prompt nobody can see.
//!
//! Every step is the same subprocess the TUI launches, so a headless install is not a second
//! implementation of the install; it is the same one with a different front end. Each step's
//! output is streamed as it arrives and recorded in the journal as it finishes.
use std::path::Path;
use std::time::Duration;

use crate::exit::Outcome;
use crate::journal::Journal;
use crate::menu::Answers;
use crate::output;
use crate::privileges;
use crate::run::{Event, Session, SkipReason};
use crate::steps::{self, STEPS, Status};

const POLL: Duration = Duration::from_millis(50);

pub fn run(bundle: &Path, answers: &Answers, resume: bool) -> Outcome {
    if !privileges::is_root() && !privileges::sudo_cached() {
        eprintln!(
            "caelestia-install: an unattended install needs root already available: run `sudo -v` \
             first, or run the installer itself with sudo."
        );
        return Outcome::Environment;
    }

    let plan = crate::journal::plan_id(answers);
    let journal = Journal::for_plan(&plan);
    if !resume {
        journal.clear();
    }
    if resume {
        output::info(&format!("journal: {}", journal.path().display()));
        for (id, status) in journal.problems() {
            if status == Status::Warn {
                output::info(&format!("{id} finished with a warning last time"));
            } else {
                output::warn(&format!(
                    "{id} stopped at {} last time; running it again",
                    status.as_journal()
                ));
            }
        }
    }

    let planned = STEPS
        .iter()
        .filter(|step| !steps::is_skipped(step, answers))
        .count();
    let mut ran = 0;
    let mut resumed = 0;

    let mut session = Session::new(
        bundle,
        crate::app::log_path(),
        steps::environment(answers, bundle),
        answers,
    )
    .resuming(journal);

    loop {
        if !session.is_running() {
            let events = session.advance();
            if events.is_empty() {
                break;
            }
            if let Some(outcome) = report(&session, events, &mut ran, &mut resumed) {
                return outcome;
            }
            continue;
        }

        if crate::signals::terminated() {
            session.abandon();
            output::err("interrupted");
            return Outcome::Failed;
        }

        let events = session.pump();
        match report(&session, events, &mut ran, &mut resumed) {
            Some(outcome) => return outcome,
            None => std::thread::sleep(POLL),
        }
    }

    if resumed > 0 {
        output::ok(&format!(
            "{ran} steps ran, {resumed} were already done of {planned} planned"
        ));
    } else {
        output::ok(&format!("{ran} of {planned} steps ran"));
    }
    Outcome::Ok
}

/// Print what happened, and say whether the run has to stop.
fn report(
    session: &Session,
    events: Vec<Event>,
    ran: &mut usize,
    resumed: &mut usize,
) -> Option<Outcome> {
    for event in events {
        match event {
            Event::Skipped { index, reason } => {
                let step = &session.steps()[index];
                match reason {
                    SkipReason::SwitchedOff => {
                        output::skip(&format!("{} is switched off", step.name));
                    }
                    SkipReason::AlreadyDone => {
                        output::skip(&format!("{} was already done", step.name));
                        *resumed += 1;
                    }
                }
            }
            Event::Started { index } => {
                let step = &session.steps()[index];
                output::info(&format!("{}  {}", step.id, step.name));
            }
            Event::Output { line, .. } => println!("{}", line.trim_end()),
            Event::Finished { index, status } => {
                let step = &session.steps()[index];
                if status.is_failure() {
                    output::err(&format!("{} failed", step.name));
                    return Some(Outcome::Failed);
                }
                if status == Status::Warn {
                    output::warn(step.name);
                } else {
                    output::ok(step.name);
                }
                *ran += 1;
            }
            Event::Broken { index, problem } => {
                output::err(&format!("{}: {problem}", session.steps()[index].name));
                return Some(Outcome::Failed);
            }
        }
    }
    None
}
