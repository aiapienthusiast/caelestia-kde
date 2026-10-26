//! Modal dialogs, confirmations, and scrim backdrop models for Utilities
//! Ported with 100% parity from `DrawerConfirmModal.qml`, `DrawerModal.qml`, and `DrawerScrim.qml`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModalActionType {
    Confirm,
    Cancel,
    Destructive,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModalDialogModel {
    pub is_open: bool,
    pub title: String,
    pub message: String,
    pub confirm_label: String,
    pub cancel_label: String,
    pub is_destructive: bool,
    pub target_id: Option<String>,
}

impl Default for ModalDialogModel {
    fn default() -> Self {
        Self {
            is_open: false,
            title: String::new(),
            message: String::new(),
            confirm_label: "Confirm".to_string(),
            cancel_label: "Cancel".to_string(),
            is_destructive: false,
            target_id: None,
        }
    }
}

impl ModalDialogModel {
    pub fn open_confirm(
        title: impl Into<String>,
        message: impl Into<String>,
        confirm_label: impl Into<String>,
        target_id: Option<String>,
    ) -> Self {
        Self {
            is_open: true,
            title: title.into(),
            message: message.into(),
            confirm_label: confirm_label.into(),
            cancel_label: "Cancel".to_string(),
            is_destructive: false,
            target_id,
        }
    }

    pub fn open_destructive(
        title: impl Into<String>,
        message: impl Into<String>,
        confirm_label: impl Into<String>,
        target_id: Option<String>,
    ) -> Self {
        Self {
            is_open: true,
            title: title.into(),
            message: message.into(),
            confirm_label: confirm_label.into(),
            cancel_label: "Cancel".to_string(),
            is_destructive: true,
            target_id,
        }
    }

    pub fn close(&mut self) {
        self.is_open = false;
        self.target_id = None;
    }
}
