use super::*;

/// What was found before anything is decided.
pub(super) fn detect(frame: &mut Frame, app: &App, body: Rect) {
    let theme = &app.theme;

    if ui::rail(body, 44).is_some() {
        let rows = findings(app);
        let label_width = label_column(&rows);
        let left_width = ui::panel_width(finding_width(&rows, label_width), body.width / 2);

        let right_width = ui::panel_width(ui::PROSE_WIDTH, body.width / 2);
        let mut narrative = verdict_body(app, right_width.saturating_sub(2));
        narrative.push(blank());
        narrative.push(ui::cursor_row(
            theme,
            app.glyphs,
            &format!("{} continue", app.glyphs.cursor()),
            right_width.saturating_sub(2),
        ));

        const GAP: u16 = 4;
        let total = left_width + GAP + right_width;
        let x = body.x + body.width.saturating_sub(total) / 2;
        let left_height = rows.len() as u16 + 2;
        let right_height = narrative.len() as u16 + 2;
        let height = left_height.max(right_height);
        let y = body.y + body.height.saturating_sub(height) / 2;

        let inner = ui::section(
            frame,
            Rect {
                x,
                y,
                width: left_width,
                height: left_height,
            },
            theme,
            app.glyphs,
            Some("this machine"),
            Role::Rule,
        );
        ui::prose(
            frame,
            inner,
            finding_lines(&rows, label_width, theme),
            theme.body(),
        );

        let border = verdict(app);
        let inner = ui::section(
            frame,
            Rect {
                x: x + left_width + GAP,
                y,
                width: right_width,
                height: right_height,
            },
            theme,
            app.glyphs,
            None,
            border,
        );
        ui::prose(frame, inner, narrative, theme.body());
        return;
    }

    let rows = findings(app);
    let label_width = label_column(&rows);
    let headline = headline(app);
    let tail = reasons(app, body.width);
    let explanation = installed_explanation(app, body.width);

    let mut lines: Vec<Line<'static>> = vec![
        Line::from(Span::styled("This machine".to_string(), theme.heading())),
        blank(),
    ];

    let mut room = (body.height as usize)
        .saturating_sub(lines.len() + 1 + tail.len())
        .saturating_sub(explanation.len());
    let row_count = rows.len();
    let show_rows = row_count < room;
    if show_rows {
        room = room.saturating_sub(row_count + 1);
    }
    let show_explanation = !explanation.is_empty() && explanation.len() <= room;

    if show_rows {
        lines.extend(finding_lines(&rows, label_width, theme));
    }
    lines.push(headline);
    if show_explanation {
        for line in explanation {
            lines.push(Line::from(Span::styled(format!(" {line}"), theme.muted())));
        }
    }
    lines.extend(tail);

    lines.extend(notice(app));
    lines.push(blank());
    lines.push(ui::cursor_row(
        &app.theme,
        app.glyphs,
        &format!("{} continue", app.glyphs.cursor()),
        body.width.min(ui::PROSE_WIDTH),
    ));
    ui::prose_middle(frame, body, lines, theme.body());
}

/// The findings: what was looked for, what was found, and whether the install depends on it.
pub(super) fn findings(app: &App) -> Vec<(&'static str, String, bool)> {
    let found = &app.detection;
    vec![
        (
            "distribution",
            found.distro_label().to_string(),
            found.distro != crate::distro::Distro::Unknown,
        ),
        (
            "install layout",
            format!("{} install", found.kind_label()),
            true,
        ),
        (
            "package manager",
            found
                .package_manager
                .map(str::to_string)
                .unwrap_or_else(|| "not found".to_string()),
            found.package_manager.is_some(),
        ),
        (
            "quickshell",
            if found.quickshell {
                "found".to_string()
            } else {
                "not installed yet".to_string()
            },
            true,
        ),
        (
            "git",
            if found.git {
                "found".to_string()
            } else {
                "missing".to_string()
            },
            found.git,
        ),
    ]
}

/// The label column is as wide as the labels in it.
pub(super) fn label_column(rows: &[(&'static str, String, bool)]) -> usize {
    rows.iter()
        .map(|(label, _, _)| label.chars().count())
        .max()
        .unwrap_or(0)
}

pub(super) fn finding_lines(
    rows: &[(&'static str, String, bool)],
    label_width: usize,
    theme: &Theme,
) -> Vec<Line<'static>> {
    rows.iter()
        .map(|(label, value, ok)| {
            let value_style = if *ok { theme.body() } else { theme.warn_text() };
            Line::from(vec![
                Span::styled(
                    format!(" {label:<label_width$}  "),
                    if *ok {
                        theme.muted()
                    } else {
                        theme.warn_text()
                    },
                ),
                Span::styled(value.clone(), value_style),
            ])
        })
        .collect()
}

/// How wide the findings panel has to be to hold its longest row.
pub(super) fn finding_width(rows: &[(&'static str, String, bool)], label_width: usize) -> u16 {
    rows.iter()
        .map(|(_, value, _)| label_width + value.chars().count() + 3)
        .max()
        .unwrap_or(0) as u16
}

/// The fact this screen turns on, in one line.
///
/// A run that stopped part way is not "already installed": the home screen sent the user here to
/// finish it, and a cross in the error colour would answer that invitation with a refusal.
pub(super) fn headline(app: &App) -> Line<'static> {
    let theme = &app.theme;
    let glyphs = app.glyphs;
    let (mark, role, text) = if app.detection.installed && !app.unfinished.is_empty() {
        (
            Status::Warn,
            theme.warn_text(),
            "The last install did not finish.",
        )
    } else if app.detection.installed {
        (
            Status::Failed,
            theme.error_text(),
            "Caelestia is already installed.",
        )
    } else {
        (
            Status::Ok,
            theme.status(Status::Ok),
            "Nothing is in the way.",
        )
    };
    Line::from(vec![
        Span::styled(format!("{} ", glyphs.status(mark)), role),
        Span::styled(text.to_string(), role),
    ])
}

/// Why the screen will not go on: a missing precondition, or the reason a guard sent the user back.
pub(super) fn reasons(app: &App, width: u16) -> Vec<Line<'static>> {
    let theme = &app.theme;
    let glyphs = app.glyphs;
    let mut lines: Vec<Line<'static>> = Vec::new();
    for blocker in app.detection.blockers() {
        lines.push(blank());
        lines.push(Line::from(vec![
            Span::styled(
                format!("{} ", glyphs.status(Status::Failed)),
                theme.error_text(),
            ),
            Span::styled(blocker.to_string(), theme.error_text()),
        ]));
    }
    if let Some(problem) = &app.problem {
        lines.push(blank());
        for line in wrap(problem, (width as usize).saturating_sub(4)) {
            lines.push(Line::from(Span::styled(
                format!(" {line}"),
                theme.error_text(),
            )));
        }
    }
    lines
}

/// What an existing install means for what happens next.
pub(super) fn installed_explanation(app: &App, width: u16) -> Vec<String> {
    if !app.detection.installed {
        return Vec::new();
    }
    let measure = (width as usize).saturating_sub(4);
    if let Some((step, _)) = app.unfinished.last() {
        return wrap(
            &format!(
                "It stopped at {step}. What finished is kept, so continuing carries on from \
                 there, and your own configs are backed up first."
            ),
            measure,
        );
    }
    wrap(
        "Continuing installs over it: the shell configuration and the deployed files are \
         replaced. Your own configs are backed up first.",
        measure,
    )
}

/// Everything the verdict panel says: the fact, what it means, and why the screen will not go on.
pub(super) fn verdict_body(app: &App, width: u16) -> Vec<Line<'static>> {
    let theme = &app.theme;
    let mut lines = vec![headline(app)];
    let explanation = installed_explanation(app, width);
    if !explanation.is_empty() {
        lines.push(blank());
        lines.extend(
            explanation
                .into_iter()
                .map(|line| Line::from(Span::styled(format!(" {line}"), theme.muted()))),
        );
    }
    lines.extend(reasons(app, width));
    lines
}

/// What the findings mean, as the role the verdict's border carries. Ordinary "ready" stays
/// neutral, since color is spent on what is wrong.
pub(super) fn verdict(app: &App) -> Role {
    let found = &app.detection;
    if !found.blockers().is_empty() {
        Role::Error
    } else if found.installed {
        Role::Warn
    } else {
        Role::Rule
    }
}
