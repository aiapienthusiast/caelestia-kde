use super::run::steps_with;
use super::*;

pub(super) fn done(frame: &mut Frame, app: &App, body: Rect) {
    let theme = &app.theme;
    let glyphs = app.glyphs;
    let tally = app.tally();

    let (status, ending, style) = if tally.failed > 0 {
        (
            Status::Failed,
            format!(
                "finished with {}",
                ui::plural(tally.failed, "failed step", "failed steps")
            ),
            theme.error_text(),
        )
    } else if tally.ignored > 0 {
        (
            Status::Ignored,
            format!(
                "finished with {}",
                ui::plural(tally.ignored, "ignored step", "ignored steps")
            ),
            theme.ignored_text(),
        )
    } else if tally.warn > 0 {
        (
            Status::Warn,
            format!(
                "finished with {}",
                ui::plural(tally.warn, "warning", "warnings")
            ),
            theme.warn_text(),
        )
    } else {
        (Status::Ok, "finished".to_string(), theme.ok_text())
    };

    let mut lines: Vec<Line<'static>> = vec![Line::from(vec![
        Span::styled(format!(" {} ", glyphs.status(status)), style),
        Span::styled("The install ".to_string(), theme.body()),
        Span::styled(ending, theme.title()),
        Span::styled(".".to_string(), theme.body()),
    ])];
    lines.extend(notice(app));

    let failed = steps_with(app, Status::Failed);
    let ignored = steps_with(app, Status::Ignored);
    let mut trouble: Vec<Line<'static>> = Vec::new();
    for index in steps_with(app, Status::Warn) {
        trouble.push(ui::labeled(
            "warned",
            STEPS[index].name.to_string(),
            theme,
            LABEL_COLUMN,
        ));
    }
    for (label, indexes) in [("failed", &failed), ("ignored", &ignored)] {
        for index in indexes {
            trouble.push(ui::labeled(
                label,
                format!(
                    "{}  ·  caelestia-install step {}",
                    STEPS[*index].name, STEPS[*index].id
                ),
                theme,
                LABEL_COLUMN,
            ));
        }
    }
    if !failed.is_empty() || !ignored.is_empty() {
        trouble.push(ui::labeled(
            "retry",
            "r here, or that command in a terminal".to_string(),
            theme,
            LABEL_COLUMN,
        ));
    }
    if !trouble.is_empty() {
        lines.push(blank());
        lines.extend(trouble);
    }

    lines.push(blank());
    if let Some(landed) = &app.landed {
        if let Some(version) = &landed.version {
            lines.push(ui::labeled(
                "version",
                version_line(version, landed),
                theme,
                LABEL_COLUMN,
            ));
        }
        lines.push(ui::labeled(
            "shell",
            crate::paths::shorten(&landed.shell),
            theme,
            LABEL_COLUMN,
        ));
        lines.push(ui::labeled(
            "settings",
            crate::paths::shorten(&landed.configs),
            theme,
            LABEL_COLUMN,
        ));
        if let Some(backup) = &landed.backup {
            lines.push(ui::labeled(
                "backup",
                crate::paths::shorten(backup),
                theme,
                LABEL_COLUMN,
            ));
        }
    }
    lines.push(ui::labeled(
        "log",
        crate::paths::shorten(&app.log_path),
        theme,
        LABEL_COLUMN,
    ));

    lines.push(blank());
    if tally.unresolved() > 0 {
        lines.push(ui::labeled(
            "kept",
            "nothing was rolled back, so the steps that finished are still installed".to_string(),
            theme,
            LABEL_COLUMN,
        ));
    } else {
        lines.push(ui::labeled(
            "next",
            "log out and back in so the new session settings apply".to_string(),
            theme,
            LABEL_COLUMN,
        ));
        lines.push(blank());
        lines.push(Line::from(Span::styled(
            format!("  {} (o) ", LOGOUT_BUTTON),
            theme.bar(),
        )));
        lines.push(blank());
    }
    lines.push(ui::labeled(
        "update",
        "caelestia-install --update".to_string(),
        theme,
        LABEL_COLUMN,
    ));
    lines.push(ui::labeled(
        "remove",
        "caelestia-install --uninstall".to_string(),
        theme,
        LABEL_COLUMN,
    ));

    ui::prose_middle(frame, body, lines, theme.body());
}

/// The version, and whichever of the revision and the channel this install can actually name: a
/// tarball has no revision to show, and a blank after a separator is worse than not saying it.
pub(super) fn version_line(version: &str, landed: &crate::app::Landed) -> String {
    let mut value = version.to_string();
    if let Some(commit) = &landed.commit {
        let short: String = commit.chars().take(7).collect();
        value.push_str(&format!(" \u{00b7} {short}"));
    }
    if let Some(branch) = &landed.branch {
        value.push_str(&format!(" \u{00b7} {branch}"));
    }
    value
}
