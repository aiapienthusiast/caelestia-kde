use super::home::column_gap;
use super::*;

pub(super) fn options(frame: &mut Frame, app: &App, body: Rect) {
    let theme = &app.theme;

    let rail_width = ui::panel_width(question_rail_width(), body.width);
    let gap = column_gap(body.width);
    let pane_width = PANE_WIDTH.min(body.width.saturating_sub(rail_width + gap));
    if pane_width >= PANE_MIN {
        let listed = option_rows(app, pane_width).len() as u16 + 2;
        let width = rail_width + gap + pane_width;
        let height = (listed + HELP_ROWS + 2).max(question_rail(app).len() as u16 + 2);
        let block = ui::middle(body, width, height);

        let rail = question_rail(app);
        let left = Rect {
            width: rail_width,
            ..block
        };
        let inner = ui::section(
            frame,
            ui::panel_rect(left, rail_width, rail.len() as u16),
            theme,
            app.glyphs,
            None,
            Role::Rule,
        );
        ui::prose(frame, inner, rail, theme.body());

        let right = Rect {
            x: block.x + rail_width + gap,
            width: pane_width,
            ..block
        };
        question_pane(frame, app, right);
        return;
    }

    question_pane(
        frame,
        app,
        ui::middle(body, body.width.min(PANE_WIDE), body.height),
    );
}

/// The room the question pane asks for, and the least it will be drawn at before the map is given
/// up instead. The heading and the help are rows the pane keeps whatever is in the list.
pub(super) const PANE_WIDTH: u16 = 74;

pub(super) const PANE_WIDE: u16 = 96;

pub(super) const PANE_MIN: u16 = 44;

pub(super) const HELP_ROWS: u16 = 2;

pub(super) fn question_rail(app: &App) -> Vec<Line<'static>> {
    let theme = &app.theme;
    let glyphs = app.glyphs;
    let mut lines: Vec<Line<'static>> = Vec::new();
    for index in 0..menu::question_count() {
        let title = menu::question(index);
        let here = index == app.question;
        let name_style = if here { theme.selected() } else { theme.body() };
        let changed = if app.question_changed(index) {
            format!("  {}", glyphs.changed())
        } else {
            String::new()
        };
        lines.push(Line::from(vec![
            Span::styled(
                if here { "  · " } else { "    " },
                if here { theme.accent() } else { theme.muted() },
            ),
            Span::styled(title.to_string(), name_style),
            Span::styled(changed, theme.accent()),
        ]));
    }
    lines
}

/// How wide the map has to be for its longest name plus its change marker.
pub(super) fn question_rail_width() -> u16 {
    let mut width = 0;
    for index in 0..menu::question_count() {
        let title = menu::question(index);
        width = width.max(2 + title.len() + 2 + 2);
    }
    width as u16
}

/// One question and its answers.
pub(super) fn question_pane(frame: &mut Frame, app: &App, area: Rect) {
    let theme = &app.theme;
    let mut listed = option_rows(app, area.width);
    listed.push(blank());
    listed.push(continue_row(app));
    let rows = Layout::vertical([
        Constraint::Length(2),
        Constraint::Length(listed.len() as u16),
        Constraint::Min(0),
    ])
    .areas::<3>(area);

    let mut heading_spans = vec![Span::styled(
        menu::question(app.question).to_string(),
        theme.heading(),
    )];
    if app.question_changed(app.question) {
        heading_spans.push(Span::styled(
            format!("  {} changed", app.glyphs.changed()),
            theme.accent(),
        ));
    }
    ui::prose(
        frame,
        rows[0],
        vec![Line::from(heading_spans)],
        theme.body(),
    );

    ui::prose(frame, rows[1], listed, theme.body());

    let mut help_lines: Vec<Line<'static>> = Vec::new();
    if let Some(text) = app.selected().and_then(Entry::help) {
        for line in wrap(text, (rows[2].width as usize).saturating_sub(2)) {
            help_lines.push(Line::from(Span::styled(line, theme.muted())));
        }
    }
    help_lines.extend(notice(app));
    ui::prose(frame, rows[2], help_lines, theme.body());
}

/// The last row of a question: the way on, drawn like every other row so the keys can reach it.
pub(super) fn continue_row(app: &App) -> Line<'static> {
    let theme = &app.theme;
    let chosen = app.on_continue();
    let style = if chosen {
        theme.selected()
    } else {
        theme.body()
    };
    Line::from(vec![
        Span::styled(
            format!(" {} ", if chosen { app.glyphs.cursor() } else { " " }),
            style,
        ),
        Span::styled("  continue".to_string(), style),
    ])
}

/// One row per option: cursor, changed marker, name, then the value in its own column.
pub(crate) fn option_rows(app: &App, width: u16) -> Vec<Line<'static>> {
    let theme = &app.theme;
    let glyphs = app.glyphs;
    let name_width = (width as usize)
        .saturating_sub(VALUE_COLUMN + 5)
        .clamp(8, NAME_CELL);
    let entries = app.current_entries();
    if entries.is_empty() {
        return vec![Line::from(Span::styled(
            "Nothing on this machine to theme.".to_string(),
            theme.muted(),
        ))];
    }
    entries
        .into_iter()
        .enumerate()
        .map(|(index, entry)| {
            let chosen = index == app.cursor;
            let changed = if app.is_changed(entry) {
                format!("{} ", glyphs.changed())
            } else {
                "  ".to_string()
            };
            let room = name_width;
            let name = format!(
                "{:<room$}",
                ui::fit(entry.title(), room, glyphs.ellipsis()),
                room = room
            );
            let (value, value_style) = value_cell(entry, app);
            let value = format!("{:>width$}", value, width = VALUE_COLUMN);
            let pointer = format!(" {} ", if chosen { glyphs.cursor() } else { " " });
            let label = if chosen {
                theme.selected()
            } else {
                theme.body()
            };

            Line::from(vec![
                Span::styled(
                    pointer,
                    if chosen {
                        theme.selected()
                    } else {
                        theme.body()
                    },
                ),
                Span::styled(changed, theme.accent()),
                Span::styled(name, label),
                Span::styled(value, value_style),
            ])
        })
        .collect()
}

/// What the row will do, with the colour of its consequence. An app row has three states rather
/// than two, because "on" means "install it" and the answer depends on whether it is already
/// there; DMS draws the same three with the same markers.
pub(super) fn value_cell(entry: &'static Entry, app: &App) -> (String, Style) {
    let glyphs = app.glyphs;
    let theme = &app.theme;
    match entry {
        Entry::Preset { .. } => (String::new(), theme.muted()),
        Entry::Toggle { id, .. } => {
            let on = menu::value(entry, &app.answers) == Some("true");
            if !on {
                return (format!("{} skip", glyphs.cross()), theme.muted());
            }
            if app.apps.has(id) {
                (
                    format!("{} installed", glyphs.tick()),
                    theme.status(Status::Ok),
                )
            } else if app.apps.is_app_row(id) {
                (
                    format!("{} will install", glyphs.circle()),
                    theme.status(Status::Warn),
                )
            } else {
                (format!("{} on", glyphs.tick()), theme.status(Status::Ok))
            }
        }
        Entry::Choice { .. } => (
            format!(
                "{} {} {}",
                glyphs.open(),
                menu::value(entry, &app.answers).unwrap_or_default(),
                glyphs.close()
            ),
            theme.muted(),
        ),
        Entry::Submenu { .. } => (glyphs.submenu().to_string(), theme.muted()),
    }
}
