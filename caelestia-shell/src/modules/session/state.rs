//! Session State Coordinator
//! 100% parity with `shell/modules/session/Content.qml` and `Wrapper.qml`.

use super::config::SessionConfig;
use crate::domain::session::{SessionAction, SessionActionKind};

#[derive(Clone, Debug)]
pub struct SessionState {
    pub is_open: bool,
    pub actions: Vec<SessionAction>,
    pub focused_index: usize,
    pub vim_keybinds: bool,
    pub enabled: bool,
}

impl Default for SessionState {
    fn default() -> Self {
        Self::new(&SessionConfig::default())
    }
}

impl SessionState {
    pub fn new(config: &SessionConfig) -> Self {
        Self {
            is_open: false,
            actions: SessionAction::ALL.to_vec(),
            focused_index: 0,
            vim_keybinds: config.vim_keybinds,
            enabled: config.enabled,
        }
    }

    /// Open the session drawer, resetting focus to the primary Logout action
    pub fn open(&mut self) {
        if self.enabled {
            self.is_open = true;
            self.focused_index = 0;
        }
    }

    /// Close the session drawer
    pub fn close(&mut self) {
        self.is_open = false;
    }

    /// Toggle session drawer open state
    pub fn toggle(&mut self) {
        if self.is_open {
            self.close();
        } else {
            self.open();
        }
    }

    /// Step focus down to next action
    pub fn focus_next(&mut self) {
        if !self.actions.is_empty() {
            self.focused_index = (self.focused_index + 1) % self.actions.len();
        }
    }

    /// Step focus up to previous action
    pub fn focus_prev(&mut self) {
        if !self.actions.is_empty() {
            if self.focused_index == 0 {
                self.focused_index = self.actions.len() - 1;
            } else {
                self.focused_index -= 1;
            }
        }
    }

    /// Set explicit focused index
    pub fn set_focused_index(&mut self, idx: usize) {
        if idx < self.actions.len() {
            self.focused_index = idx;
        }
    }

    /// Returns currently focused session action kind
    pub fn focused_action(&self) -> Option<SessionActionKind> {
        self.actions.get(self.focused_index).map(|a| a.kind)
    }

    /// Handles keyboard events matching `Content.qml` key navigation and vim bindings
    pub fn handle_key(&mut self, key: &str, ctrl: bool, shift: bool) -> Option<SessionActionKind> {
        match key {
            "escape" => {
                self.close();
                None
            }
            "enter" | "return" => self.focused_action(),
            "down" => {
                self.focus_next();
                None
            }
            "up" => {
                self.focus_prev();
                None
            }
            "tab" => {
                if shift {
                    self.focus_prev();
                } else {
                    self.focus_next();
                }
                None
            }
            "backtab" => {
                self.focus_prev();
                None
            }
            "j" | "n" if ctrl && self.vim_keybinds => {
                self.focus_next();
                None
            }
            "k" | "p" if ctrl && self.vim_keybinds => {
                self.focus_prev();
                None
            }
            _ => None,
        }
    }
}
