use super::*;

pub(super) fn auth(frame: &mut Frame, app: &App, body: Rect) {
    let theme = &app.theme;
    let glyphs = app.glyphs;
    const MEASURE: u16 = 68;
    let mut lines: Vec<Line<'static>> = Vec::new();

    for (index, method) in app.auth_methods().iter().enumerate() {
        let chosen = index == app.cursor;
        if chosen {
            lines.push(ui::cursor_row(
                theme,
                glyphs,
                &format!("{} {}", glyphs.cursor(), method.title()),
                MEASURE,
            ));
        } else {
            lines.push(Line::from(vec![
                Span::styled("   ".to_string(), theme.body()),
                Span::styled(method.title().to_string(), theme.body()),
            ]));
        }
    }

    if let Some(problem) = &app.problem {
        lines.push(blank());
        for line in wrap(problem, MEASURE as usize) {
            lines.push(Line::from(Span::styled(line, theme.error_text())));
        }
    }
    lines.extend(notice(app));

    ui::card(
        frame,
        body,
        theme,
        glyphs,
        Some("root privileges"),
        MEASURE,
        lines,
    );
}
