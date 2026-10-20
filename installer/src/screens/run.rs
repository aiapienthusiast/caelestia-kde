use super::*;

pub(super) fn run(frame: &mut Frame, app: &App) {
    let theme = &app.theme;
    let layout = app.run_layout();

    ui::prose(frame, layout.step, vec![step_line(app)], theme.body());
    ui::prose(
        frame,
        layout.bar,
        vec![bar_line(app, layout.bar.width)],
        theme.body(),
    );
    ui::prose(frame, layout.note, vec![note_line(app)], theme.body());
    log_pane(frame, app, layout.log, app.log_rows());
}

/// What is happening now: the spinner and the step's own name.
pub(super) fn step_line(app: &App) -> Line<'static> {
    let theme = &app.theme;
    let running = app.running_step();
    let index = running.or_else(|| app.next_step());
    let Some(index) = index else {
        return Line::from(Span::styled(
            format!("{} Finishing up", app.glyphs.status(Status::Ok)),
            theme.status(Status::Ok),
        ));
    };
    let style = if running.is_some() {
        theme.selected()
    } else {
        theme.body()
    };
    let glyph = if running.is_some() {
        ui::running_glyph(app)
    } else {
        app.glyphs.status(Status::Pending)
    };
    Line::from(vec![
        ui::gutter(glyph, style, app.glyphs),
        Span::styled(STEPS[index].name.to_string(), style),
    ])
}

/// The one bar on the screen, with how far along and how long it has taken.
pub(super) fn bar_line(app: &App, width: u16) -> Line<'static> {
    let done = app.finished();
    let total = STEPS.len();
    let percent = done
        .saturating_mul(100)
        .checked_div(total)
        .unwrap_or_default();
    let elapsed = app.total_elapsed().as_secs();
    let caption = if elapsed == 0 {
        format!("{percent:>3}%")
    } else {
        format!("{percent:>3}%   {}", ui::elapsed(elapsed))
    };
    ui::progress(done, total, width, &caption, &app.theme, app.glyphs)
}

/// What did not go cleanly, on the line under the bar, so that a warning is seen without opening
/// the log. Nothing here means nothing went wrong, and the line stays empty.
pub(super) fn note_line(app: &App) -> Line<'static> {
    let theme = &app.theme;
    let glyphs = app.glyphs;
    let counts = app.tally();
    let mut spans: Vec<Span<'static>> = Vec::new();
    for (count, status, one, many) in [
        (counts.failed, Status::Failed, "failure", "failures"),
        (counts.warn, Status::Warn, "warning", "warnings"),
        (counts.ignored, Status::Ignored, "ignored", "ignored"),
    ] {
        if count == 0 {
            continue;
        }
        if !spans.is_empty() {
            spans.push(Span::styled("   ".to_string(), theme.muted()));
        }
        spans.push(ui::gutter(
            glyphs.status(status),
            theme.status(status),
            glyphs,
        ));
        spans.push(Span::styled(
            ui::plural(count, one, many),
            theme.status(status),
        ));
    }
    Line::from(spans)
}

/// A log pane: a heading row, the lines, then a scrollbar. The run screen and the full-screen log
/// draw the same thing at different sizes.
pub(super) fn log_pane(frame: &mut Frame, app: &App, area: Rect, rows: usize) {
    let theme = &app.theme;
    let glyphs = app.glyphs;
    if area.height < 2 || area.width < 4 {
        return;
    }

    let heading = Rect { height: 1, ..area };
    let body = Rect {
        y: area.y + 1,
        height: area.height - 1,
        ..area
    };
    frame.render_widget(Block::default().style(theme.panel()), area);
    let (start, rows) = app.log_window_for(rows);
    let total = app.visible_log().count();
    let overflowing = total > rows;

    let mut badge = vec![Span::styled(" install log".to_string(), theme.muted())];
    badge.push(Span::styled("  ·  ".to_string(), theme.muted()));
    badge.push(Span::styled(
        if overflowing {
            format!("{}-{} of {total}", start + 1, (start + rows).min(total))
        } else {
            ui::plural(total, "line", "lines")
        },
        theme.muted(),
    ));
    if app.log_loud_only {
        badge.push(Span::styled("  ·  ".to_string(), theme.muted()));
        badge.push(Span::styled(
            "warnings and errors only".to_string(),
            theme.warn_text(),
        ));
    }
    let problems = app.problem_rows().count();
    if problems > 0 {
        badge.push(Span::styled("  ·  ".to_string(), theme.muted()));
        badge.push(Span::styled(
            format!("{} to jump to", ui::plural(problems, "problem", "problems")),
            theme.warn_text(),
        ));
    }
    ui::prose(frame, heading, vec![Line::from(badge)], theme.body());

    let follow_width = 7.min(area.width);
    let follow = Rect {
        x: area.x + area.width - follow_width,
        width: follow_width,
        ..heading
    };
    frame.render_widget(
        Paragraph::new(Line::from(Span::styled(
            if app.log_follow { "FOLLOW" } else { "PAUSED" }.to_string(),
            if app.log_follow {
                theme.selected()
            } else {
                theme.warn_text()
            },
        )))
        .alignment(Alignment::Right),
        follow,
    );

    let (text_area, bar_area) = if overflowing {
        (
            Rect {
                width: body.width - 1,
                ..body
            },
            Some(Rect {
                x: body.x + body.width - 1,
                width: 1,
                ..body
            }),
        )
    } else {
        (body, None)
    };
    let mut lines: Vec<Line<'static>> = Vec::new();
    for line in app.visible_log().skip(start).take(rows) {
        lines.push(log_line(line, theme, glyphs));
    }
    ui::prose(frame, text_area, lines, theme.body());
    if let Some(bar_area) = bar_area {
        let marks: Vec<(usize, ratatui::style::Style)> = app
            .visible_log()
            .enumerate()
            .filter(|(_, line)| line.is_loud())
            .map(|(row, line)| (row, line.style(theme)))
            .collect();
        ui::scrollbar(frame, bar_area, start, total, theme, glyphs, &marks);
    }
}

pub(super) fn log_line(line: &LogLine, theme: &Theme, glyphs: Glyphs) -> Line<'static> {
    let style = line.style(theme);
    let gutter = match line.glyph(glyphs) {
        Some(glyph) => ui::gutter(
            glyph,
            if line.kind == Kind::Heading {
                theme.rule()
            } else {
                style
            },
            glyphs,
        ),
        None => Span::styled(" ".repeat(glyphs.status_cell() + 2), theme.body()),
    };
    Line::from(vec![
        gutter,
        Span::styled(
            format!("{}{}", " ".repeat(usize::from(line.depth) * 2), line.text),
            style,
        ),
    ])
}

/// Which steps ended in a given state, in the order they ran.
pub(super) fn steps_with(app: &App, status: Status) -> Vec<usize> {
    app.statuses()
        .iter()
        .enumerate()
        .filter(|(_, each)| **each == status)
        .map(|(index, _)| index)
        .collect()
}
