//! Searcher Subsystem
//! Complete parity with `shell/modules/launcher/services/Apps.qml` prefix search and query ranking.

use super::entry::DesktopEntry;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SearchMode {
    General,
    Id,
    Category,
    Description,
    Exec,
    Keywords,
    TerminalOnly,
    Action(ActionCommand),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionCommand {
    Clipboard,
    Calculator,
    Session,
}

pub struct AppSearcher;

impl AppSearcher {
    /// Detect search mode based on prefix (e.g. `:i`, `:c`, `:t`, `>clipboard`)
    pub fn parse_mode(query: &str) -> (SearchMode, &str) {
        let trimmed = query.trim_start();
        if let Some(rest) = trimmed.strip_prefix(":i ") {
            (SearchMode::Id, rest.trim())
        } else if trimmed == ":i" {
            (SearchMode::Id, "")
        } else if let Some(rest) = trimmed.strip_prefix(":c ") {
            (SearchMode::Category, rest.trim())
        } else if trimmed == ":c" {
            (SearchMode::Category, "")
        } else if let Some(rest) = trimmed.strip_prefix(":d ") {
            (SearchMode::Description, rest.trim())
        } else if trimmed == ":d" {
            (SearchMode::Description, "")
        } else if let Some(rest) = trimmed.strip_prefix(":e ") {
            (SearchMode::Exec, rest.trim())
        } else if trimmed == ":e" {
            (SearchMode::Exec, "")
        } else if let Some(rest) = trimmed.strip_prefix(":k ") {
            (SearchMode::Keywords, rest.trim())
        } else if trimmed == ":k" {
            (SearchMode::Keywords, "")
        } else if let Some(rest) = trimmed.strip_prefix(":t ") {
            (SearchMode::TerminalOnly, rest.trim())
        } else if trimmed == ":t" {
            (SearchMode::TerminalOnly, "")
        } else if let Some(rest) = trimmed.strip_prefix(">clipboard ") {
            (SearchMode::Action(ActionCommand::Clipboard), rest.trim())
        } else if trimmed == ">clipboard" {
            (SearchMode::Action(ActionCommand::Clipboard), "")
        } else if let Some(rest) = trimmed.strip_prefix(">calc ") {
            (SearchMode::Action(ActionCommand::Calculator), rest.trim())
        } else if trimmed == ">calc" {
            (SearchMode::Action(ActionCommand::Calculator), "")
        } else if trimmed.starts_with(">session") {
            (SearchMode::Action(ActionCommand::Session), "")
        } else {
            (SearchMode::General, trimmed)
        }
    }

    /// Search and rank desktop entries
    pub fn search<'a>(query: &str, entries: &'a [DesktopEntry]) -> Vec<&'a DesktopEntry> {
        let (mode, clean_query) = Self::parse_mode(query);
        let q_lower = clean_query.to_lowercase();

        if clean_query.is_empty() {
            if mode == SearchMode::TerminalOnly {
                return entries.iter().filter(|e| e.run_in_terminal).collect();
            }
            return entries.iter().collect();
        }

        let mut scored: Vec<(&'a DesktopEntry, i32)> = entries
            .iter()
            .filter_map(|entry| {
                let score = match mode {
                    SearchMode::Id => Self::score_field(&entry.id, &q_lower),
                    SearchMode::Category => {
                        let mut max_cat = 0;
                        for cat in &entry.categories {
                            max_cat = max_cat.max(Self::score_field(cat, &q_lower));
                        }
                        max_cat
                    }
                    SearchMode::Description => {
                        Self::score_field(&entry.comment, &q_lower)
                            .max(Self::score_field(&entry.generic_name, &q_lower))
                    }
                    SearchMode::Exec => Self::score_field(&entry.exec, &q_lower),
                    SearchMode::Keywords => {
                        let mut max_kw = 0;
                        for kw in &entry.keywords {
                            max_kw = max_kw.max(Self::score_field(kw, &q_lower));
                        }
                        max_kw
                    }
                    SearchMode::TerminalOnly => {
                        if !entry.run_in_terminal {
                            0
                        } else {
                            Self::score_general(entry, &q_lower)
                        }
                    }
                    SearchMode::General => Self::score_general(entry, &q_lower),
                    SearchMode::Action(_) => 0,
                };

                if score > 0 {
                    Some((entry, score))
                } else {
                    None
                }
            })
            .collect();

        scored.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.name.cmp(&b.0.name)));
        scored.into_iter().map(|(e, _)| e).collect()
    }

    fn score_general(entry: &DesktopEntry, q: &str) -> i32 {
        let mut score = 0;
        let name_score = Self::score_field(&entry.name, q);
        if name_score > 0 {
            score += name_score * 3; // Name is primary weight
        }

        let id_score = Self::score_field(&entry.id, q);
        if id_score > 0 {
            score += id_score * 2;
        }

        let generic_score = Self::score_field(&entry.generic_name, q);
        if generic_score > 0 {
            score += generic_score;
        }

        for kw in &entry.keywords {
            let kw_score = Self::score_field(kw, q);
            if kw_score > 0 {
                score += kw_score;
            }
        }

        score
    }

    fn score_field(field: &str, q: &str) -> i32 {
        let field_lower = field.to_lowercase();
        if field_lower == q {
            100 // Exact match
        } else if field_lower.starts_with(q) {
            80 // Prefix match
        } else if field_lower.contains(q) {
            50 // Substring match
        } else {
            0
        }
    }
}
