//! Nexus Common Dialogs and Inspectors
//! Parity with `shell/modules/nexus/common/UninstallDialog.qml`, `ListEditor.qml`, and `UpdateTimeline.qml`.

use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, rgb, rgba, CursorStyle, Div, Styled};

/// Application Uninstallation Confirmation Dialog
#[derive(Clone, Debug, Default)]
pub struct UninstallDialog {
    pub is_open: bool,
    pub app_name: String,
    pub package_name: String,
    pub is_flatpak: bool,
    pub is_busy: bool,
    pub status_msg: Option<String>,
}

impl UninstallDialog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn open(&mut self, app_name: &str, package_name: &str, is_flatpak: bool) {
        self.is_open = true;
        self.app_name = app_name.to_string();
        self.package_name = package_name.to_string();
        self.is_flatpak = is_flatpak;
        self.is_busy = false;
        self.status_msg = None;
    }

    pub fn close(&mut self) {
        self.is_open = false;
        self.is_busy = false;
        self.status_msg = None;
    }

    pub fn render(&self, colours: &CaelestiaColours) -> Option<Div> {
        if !self.is_open {
            return None;
        }

        let mgr = if self.is_flatpak { "Flatpak" } else { "Pacman / Native" };

        let dialog = div()
            .flex()
            .flex_col()
            .w(px(360.0))
            .p(px(20.0))
            .bg(rgba((colours.surface << 8) | 0xF0))
            .rounded(px(ROUNDING_LARGE))
            .gap(px(16.0))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .child(
                        div()
                            .text_size(px(22.0))
                            .text_color(rgb(colours.error))
                            .child("delete_forever"),
                    )
                    .child(
                        div()
                            .text_size(px(FONT_SIZE_TITLE_MEDIUM))
                            .text_color(rgb(colours.on_surface))
                            .child("Uninstall Application"),
                    ),
            )
            .child(
                div()
                    .text_size(px(FONT_SIZE_BODY_MEDIUM))
                    .text_color(rgb(colours.on_surface_variant))
                    .child(format!(
                        "Are you sure you want to remove '{}' ({}) via {}? All local application data may be deleted.",
                        self.app_name, self.package_name, mgr
                    )),
            )
            .child(
                div()
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
                            .text_color(rgb(colours.on_surface_variant))
                            .cursor(CursorStyle::PointingHand)
                            .child("Cancel"),
                    )
                    .child(
                        div()
                            .p(px(8.0))
                            .rounded(px(ROUNDING_SMALL))
                            .bg(rgb(colours.error))
                            .text_size(px(12.0))
                            .text_color(rgb(colours.on_error))
                            .cursor(CursorStyle::PointingHand)
                            .child("Uninstall"),
                    ),
            );

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

/// Dynamic List Editor for string array preferences
#[derive(Clone, Debug)]
pub struct ListEditor {
    pub title: String,
    pub items: Vec<String>,
    pub new_item_input: String,
}

impl ListEditor {
    pub fn new(title: &str, items: Vec<String>) -> Self {
        Self {
            title: title.to_string(),
            items,
            new_item_input: String::new(),
        }
    }

    pub fn add_item(&mut self, item: &str) {
        let trimmed = item.trim();
        if !trimmed.is_empty() && !self.items.contains(&trimmed.to_string()) {
            self.items.push(trimmed.to_string());
        }
        self.new_item_input.clear();
    }

    pub fn remove_item(&mut self, index: usize) {
        if index < self.items.len() {
            self.items.remove(index);
        }
    }

    pub fn render(&self, colours: &CaelestiaColours) -> Div {
        let mut root = div()
            .flex()
            .flex_col()
            .w_full()
            .p(px(PADDING_MEDIUM))
            .bg(rgba((colours.surface_container << 8) | 0xFF))
            .rounded(px(ROUNDING_MEDIUM))
            .gap(px(SPACING_SMALL));

        let header = div()
            .flex()
            .items_center()
            .justify_between()
            .w_full()
            .child(
                div()
                    .text_size(px(FONT_SIZE_TITLE_SMALL))
                    .text_color(rgb(colours.on_surface))
                    .child(self.title.clone()),
            )
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(rgb(colours.on_surface_variant))
                    .child(format!("{} entries", self.items.len())),
            );

        root = root.child(header);

        // Item List
        let mut items_col = div().flex().flex_col().w_full().gap(px(4.0));
        if self.items.is_empty() {
            items_col = items_col.child(
                div()
                    .p(px(8.0))
                    .text_size(px(12.0))
                    .text_color(rgb(colours.on_surface_variant))
                    .child("No items added"),
            );
        } else {
            for item in &self.items {
                let row = div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .w_full()
                    .p(px(8.0))
                    .bg(rgba((colours.surface_container_high << 8) | 0xFF))
                    .rounded(px(ROUNDING_SMALL))
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(rgb(colours.on_surface))
                            .child(item.clone()),
                    )
                    .child(
                        div()
                            .cursor(CursorStyle::PointingHand)
                            .text_size(px(14.0))
                            .text_color(rgb(colours.error))
                            .child("close"),
                    );
                items_col = items_col.child(row);
            }
        }
        root = root.child(items_col);

        // Add item bar representation
        let add_bar = div()
            .flex()
            .items_center()
            .justify_between()
            .w_full()
            .p(px(6.0))
            .bg(rgba((colours.surface_container_highest << 8) | 0xFF))
            .rounded(px(ROUNDING_SMALL))
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(rgb(colours.on_surface_variant))
                    .child("Add new entry..."),
            )
            .child(
                div()
                    .cursor(CursorStyle::PointingHand)
                    .p(px(4.0))
                    .rounded(px(ROUNDING_EXTRA_SMALL))
                    .bg(rgb(colours.primary))
                    .text_size(px(12.0))
                    .text_color(rgb(colours.on_primary))
                    .child("Add"),
            );
        root = root.child(add_bar);

        root
    }
}

/// Package Update Log Item
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UpdateEntry {
    pub date: String,
    pub package: String,
    pub old_version: String,
    pub new_version: String,
    pub summary: String,
}

/// Update Timeline View for inspection of upgrade history
#[derive(Clone, Debug, Default)]
pub struct UpdateTimeline {
    pub entries: Vec<UpdateEntry>,
    pub filter: String,
}

impl UpdateTimeline {
    pub fn new(entries: Vec<UpdateEntry>) -> Self {
        Self {
            entries,
            filter: String::new(),
        }
    }

    pub fn render(&self, colours: &CaelestiaColours) -> Div {
        let mut root = div()
            .flex()
            .flex_col()
            .w_full()
            .p(px(PADDING_MEDIUM))
            .bg(rgba((colours.surface_container << 8) | 0xFF))
            .rounded(px(ROUNDING_MEDIUM))
            .gap(px(SPACING_SMALL));

        let header = div()
            .flex()
            .items_center()
            .justify_between()
            .w_full()
            .child(
                div()
                    .text_size(px(FONT_SIZE_TITLE_SMALL))
                    .text_color(rgb(colours.on_surface))
                    .child("Package Upgrade History Timeline"),
            )
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(rgb(colours.on_surface_variant))
                    .child(format!("{} recorded upgrades", self.entries.len())),
            );

        root = root.child(header);

        let mut timeline_col = div().flex().flex_col().w_full().gap(px(6.0));
        if self.entries.is_empty() {
            timeline_col = timeline_col.child(
                div()
                    .p(px(8.0))
                    .text_size(px(12.0))
                    .text_color(rgb(colours.on_surface_variant))
                    .child("No package update history available"),
            );
        } else {
            for entry in &self.entries {
                let card = div()
                    .flex()
                    .flex_col()
                    .w_full()
                    .p(px(8.0))
                    .bg(rgba((colours.surface_container_high << 8) | 0xFF))
                    .rounded(px(ROUNDING_SMALL))
                    .gap(px(2.0))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .text_size(px(13.0))
                                    .text_color(rgb(colours.on_surface))
                                    .child(entry.package.clone()),
                            )
                            .child(
                                div()
                                    .text_size(px(10.0))
                                    .text_color(rgb(colours.on_surface_variant))
                                    .child(entry.date.clone()),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(6.0))
                            .child(
                                div()
                                    .p(px(2.0))
                                    .rounded(px(ROUNDING_EXTRA_SMALL))
                                    .bg(rgba((colours.surface_container_highest << 8) | 0xFF))
                                    .text_size(px(11.0))
                                    .text_color(rgb(colours.on_surface_variant))
                                    .child(entry.old_version.clone()),
                            )
                            .child(
                                div()
                                    .text_size(px(10.0))
                                    .text_color(rgb(colours.primary))
                                    .child("arrow_forward"),
                            )
                            .child(
                                div()
                                    .p(px(2.0))
                                    .rounded(px(ROUNDING_EXTRA_SMALL))
                                    .bg(rgba((colours.primary_container << 8) | 0xFF))
                                    .text_size(px(11.0))
                                    .text_color(rgb(colours.on_primary_container))
                                    .child(entry.new_version.clone()),
                            ),
                    );
                timeline_col = timeline_col.child(card);
            }
        }

        root = root.child(timeline_col);
        root
    }
}
