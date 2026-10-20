use super::*;

pub(super) fn home(frame: &mut Frame, app: &App, body: Rect) {
    let theme = &app.theme;
    let mark = banner(body);
    let named = mark.is_none();
    let credits = credits(app);
    let rest = notice(app);

    let identity_lines = identity(app, named);
    let gap = column_gap(body.width);
    let panel_width = ui::panel_width(actions_width(app), body.width);
    let mark_rows = if named {
        0
    } else {
        theme::SPLASH.len() as u16 + 1
    };
    let mark_width = theme::SPLASH[0].chars().count() as u16;
    let text_width = identity_lines
        .iter()
        .chain(credits.iter())
        .map(|line| line.width() as u16)
        .max()
        .unwrap_or(0);
    let left_width = if body.width >= mark_width + gap + panel_width {
        HOME_LEFT.max(mark_width)
    } else {
        text_width
    };
    let two_columns = body.width >= left_width + gap + panel_width;
    let left_rows = identity_lines.len() + 2 + credits.len();

    let (width, height) = if two_columns {
        (
            left_width + gap + panel_width,
            mark_rows + left_rows.max(ACTIONS_PANEL as usize + rest.len()) as u16,
        )
    } else {
        (
            ui::PROSE_WIDTH.min(body.width),
            mark_rows
                + identity_lines.len() as u16
                + Action::ALL.len() as u16
                + rest.len() as u16
                + credits.len() as u16
                + 3,
        )
    };
    let block = ui::middle(body, width, height);

    let below = match mark {
        Some(area) => {
            ui::prose(
                frame,
                Rect {
                    height: area.height,
                    ..block
                },
                mark_lines(app),
                theme.accent(),
            );
            Rect {
                y: block.y + mark_rows,
                height: block.height.saturating_sub(mark_rows),
                ..block
            }
        }
        None => block,
    };

    if two_columns {
        let left = Rect {
            width: left_width.min(below.width),
            ..below
        };
        let right = Rect {
            x: below.x + left.width + gap,
            width: below.width.saturating_sub(left.width + gap),
            ..below
        };
        let mut lines = identity_lines.clone();
        lines.push(blank());
        lines.extend(credits.clone());
        ui::prose(frame, left, lines, theme.body());
        actions_panel(frame, app, right);
        return;
    }

    let mut lines = identity_lines;
    lines.extend(actions(app, below.width));
    lines.extend(rest);
    lines.push(blank());
    lines.extend(credits);
    ui::prose(frame, below, lines, theme.body());
}

/// The left column of the home screen, and the least air beside it.
pub(super) const HOME_LEFT: u16 = 54;

pub(super) const HOME_GAP: u16 = 4;

pub(super) fn column_gap(width: u16) -> u16 {
    HOME_GAP.max((width / 20).min(12))
}

pub(super) fn actions_panel(frame: &mut Frame, app: &App, area: Rect) {
    let theme = &app.theme;
    let width = ui::panel_width(actions_width(app), area.width);
    let inner = ui::section(
        frame,
        ui::panel_rect(area, width, Action::ALL.len() as u16),
        theme,
        app.glyphs,
        None,
        Role::Rule,
    );
    ui::prose(frame, inner, actions(app, inner.width), theme.body());
    let rest = notice(app);
    if !rest.is_empty() {
        ui::prose(
            frame,
            ui::dropped(area, ACTIONS_PANEL + 1),
            rest,
            theme.body(),
        );
    }
}

/// The action panel: three rows and its borders.
pub(super) const ACTIONS_PANEL: u16 = 5;

pub(super) const MARK_MIN_HEIGHT: u16 = 22;

pub(super) fn banner(body: Rect) -> Option<Rect> {
    let width = theme::SPLASH[0].chars().count() as u16;
    if body.width < width || body.height < MARK_MIN_HEIGHT {
        return None;
    }
    Some(Rect {
        height: theme::SPLASH.len() as u16,
        ..body
    })
}

/// The mark, one row per line.
pub(super) fn mark_lines(app: &App) -> Vec<Line<'static>> {
    theme::SPLASH
        .iter()
        .map(|row| Line::from(Span::styled(row.to_string(), app.theme.accent())))
        .collect()
}

/// The name, where the mark has not already said it, and what this machine turned out to be.
pub(super) fn identity(app: &App, named: bool) -> Vec<Line<'static>> {
    let theme = &app.theme;
    let mut lines: Vec<Line<'static>> = Vec::new();
    if named {
        lines.push(Line::from(Span::styled(
            theme::NAME.to_string(),
            theme.title(),
        )));
    }
    lines.push(Line::from(Span::styled(
        theme::TAGLINE.to_string(),
        theme.muted(),
    )));
    lines.push(blank());
    lines.push(state_line(app));
    lines.extend(finish_line(app));
    lines
}

/// What was found, in the line the actions beside it depend on.
pub(super) fn state_line(app: &App) -> Line<'static> {
    let theme = &app.theme;
    if !app.installed() {
        return Line::from(Span::styled("nothing installed yet", theme.muted()));
    }
    if !app.unfinished.is_empty() {
        return Line::from(Span::styled(
            "Caelestia is installed, but the last run did not finish",
            theme.warn_text(),
        ));
    }
    Line::from(Span::styled("Caelestia is installed", theme.body()))
}

/// Where the last run stopped, and what to do about it: the steps that finished are kept, so
/// running the install again is the whole of the repair.
pub(super) fn finish_line(app: &App) -> Option<Line<'static>> {
    let (step, _) = app.unfinished.last()?;
    Some(Line::from(Span::styled(
        format!("stopped at {step} \u{00b7} install again to finish it"),
        app.theme.muted(),
    )))
}

/// The three actions. The label is the whole row: what each one does is the verb, and anything
/// that needs saying about it belongs on the confirmation that asks for it. A row that is not on
/// offer carries the reason it is grey.
pub(super) fn actions(app: &App, width: u16) -> Vec<Line<'static>> {
    let theme = &app.theme;
    let glyphs = app.glyphs;
    let column = name_column();
    Action::ALL
        .iter()
        .enumerate()
        .map(|(index, action)| {
            let enabled = app.action_enabled(*action);
            let name = format!("{:<column$}", action.title(), column = column);
            if !enabled {
                return Line::from(vec![
                    Span::styled("  ", theme.muted()),
                    Span::styled(name, theme.muted()),
                    Span::styled(format!("  {}", Action::unavailable(*action)), theme.muted()),
                ]);
            }
            if index == app.home {
                return ui::cursor_row(
                    theme,
                    glyphs,
                    &format!("{} {name}", glyphs.cursor()),
                    width,
                );
            }
            Line::from(vec![
                Span::styled("  ", theme.body()),
                Span::styled(name, theme.body()),
            ])
        })
        .collect()
}

/// The column the action names are padded to, so the reasons line up under each other.
pub(super) fn name_column() -> usize {
    Action::ALL
        .iter()
        .map(|action| action.title().len())
        .max()
        .unwrap_or(0)
}

/// How wide the action list has to be.
pub(super) fn actions_width(app: &App) -> u16 {
    let column = name_column();
    Action::ALL
        .iter()
        .map(|action| {
            if app.action_enabled(*action) {
                2 + column
            } else {
                2 + column + 2 + Action::unavailable(*action).chars().count()
            }
        })
        .max()
        .unwrap_or(0) as u16
}

pub(super) fn credits(app: &App) -> Vec<Line<'static>> {
    let theme = &app.theme;
    vec![
        Line::from(Span::styled(theme::AUTHOR.to_string(), theme.muted())),
        Line::from(Span::styled(theme::CO_AUTHOR.to_string(), theme.muted())),
    ]
}
