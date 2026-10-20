use super::*;

pub(super) fn privilege(frame: &mut Frame, app: &App, body: Rect) {
    let theme = &app.theme;
    let glyphs = app.glyphs;
    const MEASURE: u16 = 56;
    let mut lines = if app.auth == Some(AuthMethod::Fingerprint) {
        fingerprint_card(app)
    } else {
        password_card(app)
    };

    lines.push(blank());
    if app.checking.is_some() {
        let waiting = if app.auth == Some(AuthMethod::Fingerprint) {
            "Waiting for your finger..."
        } else {
            "Checking your password..."
        };
        lines.push(Line::from(vec![
            Span::styled(format!("{} ", ui::running_glyph(app)), theme.accent()),
            Span::styled(waiting.to_string(), theme.muted()),
        ]));
    } else if let Some(problem) = &app.problem {
        for line in wrap(problem, MEASURE as usize) {
            lines.push(Line::from(Span::styled(line, theme.error_text())));
        }
    }
    if app.attempts > 0 {
        lines.push(Line::from(Span::styled(
            format!("attempt {}", app.attempts + 1),
            theme.warn_text(),
        )));
    }

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

/// The reader's card: what to do with it, and nothing else. What the reader is for is the name of
/// the route, and the reader answers when the finger arrives rather than when a key does.
pub(super) fn fingerprint_card(app: &App) -> Vec<Line<'static>> {
    vec![Line::from(Span::styled(
        AuthMethod::Fingerprint.title().to_string(),
        app.theme.body(),
    ))]
}

pub(super) fn password_card(app: &App) -> Vec<Line<'static>> {
    let theme = &app.theme;
    let glyphs = app.glyphs;
    const MEASURE: u16 = 56;

    let mut field = vec![
        Span::styled(format!("{:<10}", "password"), theme.muted()),
        Span::styled(
            glyphs
                .mask()
                .repeat(app.password.chars().count().min(MEASURE as usize)),
            theme.body(),
        ),
    ];
    if app.password.is_empty() {
        field.push(Span::styled("(type, then ".to_string(), theme.muted()));
        field.push(Span::styled(glyphs.enter().to_string(), theme.accent()));
        field.push(Span::styled(")".to_string(), theme.muted()));
    }
    vec![Line::from(field)]
}
