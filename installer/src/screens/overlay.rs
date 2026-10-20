use super::run::{log_line, log_pane};
use super::*;

pub(super) fn keys_overlay(frame: &mut Frame, app: &App) {
    let theme = &app.theme;
    let glyphs = app.glyphs;
    let entries: [(&str, &str); 12] = [
        ("\u{2191}\u{2193} / j k", "move"),
        ("\u{2190}\u{2192} / h l", "change a value"),
        (glyphs.enter(), "continue, or open"),
        (glyphs.escape(), "back one step"),
        ("q", "quit"),
        ("ctrl+c", "stop and quit"),
        ("t", "dark or light"),
        ("f", "follow the log"),
        ("w", "warnings and errors only"),
        ("l", "the whole log, full screen"),
        ("r / R", "reset one option, or all of them"),
        ("?", "this list"),
    ];
    let inner = ui::panel(
        frame,
        frame.area(),
        theme,
        glyphs,
        Some("KEYS"),
        (62, entries.len() as u16 + 5),
        Role::Rule,
    );
    let mut lines: Vec<Line<'static>> = Vec::new();
    for (keys, what) in entries {
        lines.push(Line::from(vec![
            Span::styled(format!("  {keys:<12}"), theme.accent()),
            Span::styled(what.to_string(), theme.body()),
        ]));
    }
    ui::prose(frame, inner, lines, theme.body());
}

pub(super) fn log_overlay(frame: &mut Frame, app: &App) {
    let theme = &app.theme;
    let glyphs = app.glyphs;
    let area = frame.area();
    let inner = ui::frame_panel(
        frame,
        ui::log_overlay_rect(area),
        theme,
        glyphs,
        Some("INSTALL LOG"),
        Role::Rule,
    );
    let rows = inner.height.saturating_sub(1).max(1) as usize;
    log_pane(frame, app, inner, rows);
}

pub(super) fn failure_overlay(frame: &mut Frame, app: &App) {
    let theme = &app.theme;
    let glyphs = app.glyphs;
    let Some(card) = &app.failure else {
        return;
    };
    let height = card.detail.len() as u16 + FailureCard::CHOICES.len() as u16 + 10;
    let inner = ui::panel(
        frame,
        frame.area(),
        theme,
        glyphs,
        None,
        (78, height),
        Role::Error,
    );

    let mut lines: Vec<Line<'static>> = vec![
        Line::from(vec![
            Span::styled(
                format!(" {} ", glyphs.status(Status::Failed)),
                theme.error_text(),
            ),
            Span::styled(card.step.to_string(), theme.heading()),
            Span::styled(" did not finish".to_string(), theme.muted()),
        ]),
        blank(),
    ];
    if card.detail.is_empty() {
        lines.push(Line::from(Span::styled(
            "the step printed nothing".to_string(),
            theme.muted(),
        )));
    }
    for line in &card.detail {
        lines.push(log_line(line, theme, glyphs));
    }
    lines.push(blank());
    lines.push(ui::labeled(
        "log",
        app.log_path.display().to_string(),
        theme,
        LABEL_COLUMN,
    ));
    lines.push(ui::labeled(
        "reproduce",
        card.reproduction(),
        theme,
        LABEL_COLUMN,
    ));
    lines.push(blank());
    for (index, choice) in FailureCard::CHOICES.iter().enumerate() {
        let chosen = index == card.choice;
        let pointer = format!(" {} ", if chosen { glyphs.cursor() } else { " " });
        let label = format!("{} {:<8}", choice.key, choice.label);
        if chosen {
            let mut spans = vec![Span::raw(pointer), Span::raw(label)];
            if !choice.help.is_empty() {
                spans.push(Span::raw(choice.help.to_string()));
            }
            lines.push(ui::cursor_spans(theme, spans, inner.width));
        } else {
            let mut spans = vec![
                Span::styled(pointer, theme.muted()),
                Span::styled(label, theme.body()),
            ];
            if !choice.help.is_empty() {
                spans.push(Span::styled(choice.help.to_string(), theme.muted()));
            }
            lines.push(Line::from(spans));
        }
    }
    lines.push(blank());
    lines.push(Line::from(Span::styled(
        format!(
            "{} closes this without deciding, and leaves the run paused",
            glyphs.escape()
        ),
        theme.muted(),
    )));
    ui::prose(frame, inner, lines, theme.body());
}

pub(super) fn confirm_overlay(frame: &mut Frame, app: &App) {
    let theme = &app.theme;
    let glyphs = app.glyphs;
    let asked = app.confirm.unwrap_or(Confirm::Install);
    let (subject, question, border) = match app.confirm {
        Some(Confirm::Handoff(action)) => (
            action.title(),
            "The installer closes and hands the terminal over.".to_string(),
            Role::Accent,
        ),
        Some(Confirm::Install) => (
            "Install Caelestia now?",
            "Packages are installed, the configuration is deployed and the shell is built."
                .to_string(),
            Role::Accent,
        ),
        Some(Confirm::Quit) | None => (
            "Stop the install here?",
            "A step that is running is stopped first. Work already done stays done.".to_string(),
            Role::Warn,
        ),
        Some(Confirm::LogOut) => (
            "Log out now?",
            "Everything open is closed and the next session starts with Caelestia. Save what you \
             are working on first."
                .to_string(),
            Role::Accent,
        ),
    };

    let width = 70u16.min(frame.area().width.saturating_sub(2)).max(8);
    let measure = width.saturating_sub(4) as usize;
    let mut lines = vec![
        Line::from(Span::styled(subject.to_string(), theme.heading())),
        blank(),
    ];
    for line in wrap(&question, measure) {
        lines.push(Line::from(Span::styled(line, theme.muted())));
    }
    if app.confirm == Some(Confirm::Install) {
        let changed = app.changed_answers();
        lines.push(blank());
        lines.push(Line::from(Span::styled(
            "Your settings are backed up first.".to_string(),
            theme.muted(),
        )));
        if changed > 0 {
            lines.push(Line::from(Span::styled(
                format!(
                    "{} changed from the defaults.",
                    ui::plural(changed, "option", "options")
                ),
                theme.accent(),
            )));
        }
    }
    lines.push(blank());
    for (index, label) in asked.buttons().iter().enumerate() {
        let chosen = index == app.confirm_focus;
        let pointer = format!(" {} ", if chosen { glyphs.cursor() } else { " " });
        let letter = if index == 0 { "y" } else { "n" };
        if chosen {
            let spans = vec![Span::raw(pointer), Span::raw(format!("{label} ({letter})"))];
            lines.push(ui::cursor_spans(theme, spans, width - 2));
        } else {
            lines.push(Line::from(vec![
                Span::styled(pointer, theme.muted()),
                Span::styled(format!("{label} "), theme.body()),
                Span::styled(format!("({letter})"), theme.muted()),
            ]));
        }
    }
    let height = lines.len() as u16 + 2;
    let inner = ui::panel(
        frame,
        frame.area(),
        theme,
        glyphs,
        None,
        (width, height),
        border,
    );
    ui::prose(frame, inner, lines, theme.body());
}
