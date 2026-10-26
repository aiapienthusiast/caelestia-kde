//! Key Capture Dialog & Shortcut Configuration Rows
//! Parity with `shell/modules/nexus/common/KeyCaptureDialog.qml` and `ShortcutRow.qml`.

use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use crate::services::global_shortcut::RegisteredShortcut;
use gpui::{div, prelude::*, px, rgb, rgba, CursorStyle, Div, Styled};

/// Key Capture Dialog for interactive shortcut recording and collision detection
#[derive(Clone, Debug, Default)]
pub struct KeyCaptureDialog {
    pub is_open: bool,
    pub shortcut_name: String,
    pub current_key: String,
    pub captured_key: String,
    pub conflict_info: Option<(String, String)>, // (conflicting_shortcut_name, friendly_label)
}

impl KeyCaptureDialog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn open(&mut self, name: &str, current_key: &str) {
        self.is_open = true;
        self.shortcut_name = name.to_string();
        self.current_key = current_key.to_string();
        self.captured_key.clear();
        self.conflict_info = None;
    }

    pub fn close(&mut self) {
        self.is_open = false;
        self.captured_key.clear();
        self.conflict_info = None;
    }

    pub fn capture_key(&mut self, key: &str, all_shortcuts: &[RegisteredShortcut]) {
        self.captured_key = key.trim().to_string();

        if self.captured_key.is_empty() {
            self.conflict_info = None;
            return;
        }

        // Check for conflicts across other registered shortcuts
        let mut collision = None;
        for sc in all_shortcuts {
            if sc.name == self.shortcut_name {
                continue;
            }
            let parts: Vec<&str> = sc.key.split(';').map(|s| s.trim()).collect();
            if parts.contains(&self.captured_key.as_str()) {
                collision = Some((sc.name.clone(), sc.description.clone()));
                break;
            }
        }
        self.conflict_info = collision;
    }

    pub fn confirm(&mut self) -> Option<(String, String)> {
        if self.captured_key.is_empty() {
            return None;
        }
        let res = Some((self.shortcut_name.clone(), self.captured_key.clone()));
        self.close();
        res
    }

    pub fn clear(&mut self) -> String {
        let name = self.shortcut_name.clone();
        self.close();
        name
    }

    pub fn render(&self, colours: &CaelestiaColours) -> Option<Div> {
        if !self.is_open {
            return None;
        }

        let display_key = if self.captured_key.is_empty() {
            "Press keys on keyboard..."
        } else {
            &self.captured_key
        };

        let mut dialog = div()
            .flex()
            .flex_col()
            .w(px(360.0))
            .p(px(20.0))
            .bg(rgba((colours.surface << 8) | 0xF0))
            .rounded(px(ROUNDING_LARGE))
            .gap(px(16.0));

        // Title and description
        dialog = dialog
            .child(
                div()
                    .text_size(px(FONT_SIZE_TITLE_MEDIUM))
                    .text_color(rgb(colours.on_surface))
                    .child(format!("Edit Shortcut: {}", self.shortcut_name)),
            )
            .child(
                div()
                    .text_size(px(FONT_SIZE_BODY_MEDIUM))
                    .text_color(rgb(colours.on_surface_variant))
                    .child("Press the key combination you want to assign to this action."),
            );

        // Captured Key Box
        let key_box = div()
            .flex()
            .items_center()
            .justify_center()
            .w_full()
            .h(px(54.0))
            .bg(rgba((colours.surface_container_highest << 8) | 0xFF))
            .rounded(px(ROUNDING_MEDIUM))
            .child(
                div()
                    .text_size(px(16.0))
                    .text_color(if self.captured_key.is_empty() {
                        rgb(colours.on_surface_variant)
                    } else {
                        rgb(colours.primary)
                    })
                    .child(display_key.to_string()),
            );

        dialog = dialog.child(key_box);

        // Collision Warning if conflict detected
        if let Some((conflict_name, label)) = &self.conflict_info {
            let warn = div()
                .flex()
                .items_center()
                .gap(px(8.0))
                .p(px(8.0))
                .bg(rgba((colours.error_container << 8) | 0xFF))
                .rounded(px(ROUNDING_SMALL))
                .child(
                    div()
                        .text_size(px(16.0))
                        .text_color(rgb(colours.error))
                        .child("warning"),
                )
                .child(
                    div()
                        .text_size(px(11.0))
                        .text_color(rgb(colours.on_error_container))
                        .child(format!("Shortcut already assigned to: {} ({})", label, conflict_name)),
                );
            dialog = dialog.child(warn);
        }

        // Action Buttons: Clear, Cancel, Save
        let buttons = div()
            .flex()
            .items_center()
            .justify_end()
            .w_full()
            .gap(px(8.0))
            .child(
                div()
                    .p(px(8.0))
                    .rounded(px(ROUNDING_SMALL))
                    .bg(rgba((colours.surface_container_high << 8) | 0xFF))
                    .text_size(px(12.0))
                    .text_color(rgb(colours.error))
                    .cursor(CursorStyle::PointingHand)
                    .child("Clear"),
            )
            .child(
                div()
                    .p(px(8.0))
                    .rounded(px(ROUNDING_SMALL))
                    .bg(rgba((colours.surface_container_high << 8) | 0xFF))
                    .text_size(px(12.0))
                    .text_color(rgb(colours.on_surface_variant))
                    .cursor(CursorStyle::PointingHand)
                    .child("Cancel"),
            )
            .child(
                div()
                    .p(px(8.0))
                    .rounded(px(ROUNDING_SMALL))
                    .bg(rgb(colours.primary))
                    .text_size(px(12.0))
                    .text_color(rgb(colours.on_primary))
                    .cursor(CursorStyle::PointingHand)
                    .child("Save"),
            );

        dialog = dialog.child(buttons);

        // Scrim wrapper
        Some(
            div()
                .flex()
                .items_center()
                .justify_center()
                .w_full()
                .h_full()
                .bg(rgba(0x00000088))
                .child(dialog),
        )
    }
}

/// Shortcut Display & Configuration Row
#[derive(Clone, Debug)]
pub struct ShortcutRow {
    pub name: String,
    pub description: String,
    pub key: String,
}

impl ShortcutRow {
    pub fn new(name: &str, description: &str, key: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            key: key.to_string(),
        }
    }

    pub fn render(&self, colours: &CaelestiaColours) -> Div {
        div()
            .flex()
            .items_center()
            .justify_between()
            .w_full()
            .p(px(10.0))
            .bg(rgba((colours.surface_container_high << 8) | 0xFF))
            .rounded(px(ROUNDING_SMALL))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .text_size(px(13.0))
                            .text_color(rgb(colours.on_surface))
                            .child(self.description.clone()),
                    )
                    .child(
                        div()
                            .text_size(px(11.0))
                            .text_color(rgb(colours.on_surface_variant))
                            .child(self.name.clone()),
                    ),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .child(
                        div()
                            .p(px(6.0))
                            .rounded(px(ROUNDING_EXTRA_SMALL))
                            .bg(rgba((colours.surface_container_highest << 8) | 0xFF))
                            .text_size(px(12.0))
                            .text_color(rgb(colours.primary))
                            .child(if self.key.is_empty() { "Disabled".to_string() } else { self.key.clone() }),
                    )
                    .child(
                        div()
                            .cursor(CursorStyle::PointingHand)
                            .text_size(px(14.0))
                            .text_color(rgb(colours.on_surface_variant))
                            .child("edit"),
                    ),
            )
    }
}
