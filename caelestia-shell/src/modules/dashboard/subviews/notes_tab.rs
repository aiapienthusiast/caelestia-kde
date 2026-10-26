//! Dashboard Notes Tab
//! Ported with 100% parity from `shell/modules/dashboard/NotesTab.qml`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DashboardNoteEntry {
    pub id: String,
    pub title: String,
    pub body: String,
    pub timestamp_epoch_secs: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotesTabModel {
    pub notes: Vec<DashboardNoteEntry>,
    pub active_note_idx: usize,
    pub is_editing: bool,
    pub search_filter: String,
}

impl Default for NotesTabModel {
    fn default() -> Self {
        Self {
            notes: vec![DashboardNoteEntry {
                id: "welcome".to_string(),
                title: "Scratchpad".to_string(),
                body: "Quick thoughts, todos, and terminal output snippets.".to_string(),
                timestamp_epoch_secs: 0,
            }],
            active_note_idx: 0,
            is_editing: false,
            search_filter: String::new(),
        }
    }
}

impl NotesTabModel {
    pub fn create_note(&mut self, title: impl Into<String>) {
        let entry = DashboardNoteEntry {
            id: format!("note_{}", self.notes.len() + 1),
            title: title.into(),
            body: String::new(),
            timestamp_epoch_secs: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
        };
        self.notes.push(entry);
        self.active_note_idx = self.notes.len() - 1;
        self.is_editing = true;
    }

    pub fn delete_active_note(&mut self) -> bool {
        if self.notes.is_empty() {
            return false;
        }
        self.notes.remove(self.active_note_idx);
        if self.active_note_idx >= self.notes.len() && !self.notes.is_empty() {
            self.active_note_idx = self.notes.len() - 1;
        }
        true
    }

    pub fn active_note(&self) -> Option<&DashboardNoteEntry> {
        self.notes.get(self.active_note_idx)
    }

    pub fn active_note_mut(&mut self) -> Option<&mut DashboardNoteEntry> {
        self.notes.get_mut(self.active_note_idx)
    }

    pub fn filtered_notes(&self) -> Vec<&DashboardNoteEntry> {
        if self.search_filter.is_empty() {
            self.notes.iter().collect()
        } else {
            let q = self.search_filter.to_lowercase();
            self.notes
                .iter()
                .filter(|n| n.title.to_lowercase().contains(&q) || n.body.to_lowercase().contains(&q))
                .collect()
        }
    }
}
