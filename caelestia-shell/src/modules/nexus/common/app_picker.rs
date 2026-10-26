//! Application and Window Picker Rows for Nexus Settings
//! Parity with `shell/modules/nexus/common/AppPickerRow.qml` and `WindowPickerRow.qml`.

use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use crate::modules::launcher::entry::DesktopEntry;
use gpui::{div, prelude::*, px, rgb, rgba, CursorStyle, Div, Styled};

/// Searchable Application Picker Row
#[derive(Clone, Debug, Default)]
pub struct AppPickerRow {
    pub filter: String,
    pub exclude: Vec<String>,
    pub is_open: bool,
    pub selected_app: Option<String>,
}

impl AppPickerRow {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_filter(&mut self, text: &str) {
        self.filter = text.trim().to_string();
    }

    pub fn matches<'a>(&self, apps: &'a [DesktopEntry]) -> Vec<&'a DesktopEntry> {
        let needle = self.filter.to_lowercase();
        apps.iter()
            .filter(|entry| {
                if self.exclude.iter().any(|ex| ex.eq_ignore_ascii_case(&entry.name)) {
                    return false;
                }
                if needle.is_empty() {
                    return true;
                }
                entry.name.to_lowercase().contains(&needle)
                    || entry.generic_name.to_lowercase().contains(&needle)
                    || entry.comment.to_lowercase().contains(&needle)
            })
            .collect()
    }

    pub fn select(&mut self, name: &str) {
        self.selected_app = Some(name.to_string());
        self.is_open = false;
    }

    pub fn render(&self, apps: &[DesktopEntry], colours: &CaelestiaColours) -> Div {
        let matches = self.matches(apps);
        let selected_text = self.selected_app.as_deref().unwrap_or("Select application...");

        let mut root = div()
            .flex()
            .flex_col()
            .w_full()
            .gap(px(8.0));

        // Header button row
        let header = div()
            .flex()
            .items_center()
            .justify_between()
            .w_full()
            .p(px(PADDING_MEDIUM))
            .bg(rgba((colours.surface_container_high << 8) | 0xFF))
            .rounded(px(ROUNDING_MEDIUM))
            .cursor(CursorStyle::PointingHand)
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(SPACING_SMALL))
                    .child(
                        div()
                            .text_size(px(16.0))
                            .text_color(rgb(colours.primary))
                            .child("apps"),
                    )
                    .child(
                        div()
                            .text_size(px(FONT_SIZE_LABEL_MEDIUM))
                            .text_color(rgb(colours.on_surface))
                            .child(selected_text.to_string()),
                    ),
            )
            .child(
                div()
                    .text_size(px(14.0))
                    .text_color(rgb(colours.on_surface_variant))
                    .child(if self.is_open { "expand_less" } else { "expand_more" }),
            );

        root = root.child(header);

        // Dropdown list if open
        if self.is_open {
            let mut dropdown = div()
                .flex()
                .flex_col()
                .w_full()
                .max_h(px(240.0))
                .overflow_hidden()
                .p(px(8.0))
                .bg(rgba((colours.surface_container << 8) | 0xFF))
                .rounded(px(ROUNDING_MEDIUM))
                .gap(px(4.0));

            // Search input box representation
            let search_input = div()
                .flex()
                .items_center()
                .w_full()
                .p(px(6.0))
                .bg(rgba((colours.surface_container_highest << 8) | 0xFF))
                .rounded(px(ROUNDING_SMALL))
                .gap(px(8.0))
                .child(
                    div()
                        .text_size(px(14.0))
                        .text_color(rgb(colours.on_surface_variant))
                        .child("search"),
                )
                .child(
                    div()
                        .text_size(px(13.0))
                        .text_color(rgb(colours.on_surface))
                        .child(if self.filter.is_empty() {
                            "Search applications...".to_string()
                        } else {
                            self.filter.clone()
                        }),
                );

            dropdown = dropdown.child(search_input);

            if matches.is_empty() {
                dropdown = dropdown.child(
                    div()
                        .p(px(12.0))
                        .text_size(px(12.0))
                        .text_color(rgb(colours.on_surface_variant))
                        .child("No matching applications"),
                );
            } else {
                for app in matches.iter().take(15) {
                    let is_active = self.selected_app.as_deref() == Some(&app.name);
                    let item = div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .w_full()
                        .p(px(6.0))
                        .rounded(px(ROUNDING_SMALL))
                        .bg(if is_active {
                            rgba((colours.secondary_container << 8) | 0xFF)
                        } else {
                            rgba((colours.surface_container_high << 8) | 0xFF)
                        })
                        .cursor(CursorStyle::PointingHand)
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap(px(8.0))
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(rgb(colours.primary))
                                        .child("widgets"),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .flex_col()
                                        .child(
                                            div()
                                                .text_size(px(12.0))
                                                .text_color(rgb(colours.on_surface))
                                                .child(app.name.clone()),
                                        )
                                        .child(
                                            div()
                                                .text_size(px(10.0))
                                                .text_color(rgb(colours.on_surface_variant))
                                                .child(app.generic_name.clone()),
                                        ),
                                ),
                        )
                        .child(
                            if is_active {
                                div()
                                    .text_size(px(14.0))
                                    .text_color(rgb(colours.primary))
                                    .child("check")
                            } else {
                                div()
                            },
                        );
                    dropdown = dropdown.child(item);
                }
            }

            root = root.child(dropdown);
        }

        root
    }
}

/// Window Picker Row for window rule overrides and picture-in-picture selection
#[derive(Clone, Debug, Default)]
pub struct WindowPickerRow {
    pub is_open: bool,
    pub selected_window_id: Option<String>,
    pub selected_title: Option<String>,
}

impl WindowPickerRow {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn select(&mut self, id: &str, title: &str) {
        self.selected_window_id = Some(id.to_string());
        self.selected_title = Some(title.to_string());
        self.is_open = false;
    }

    pub fn render(
        &self,
        windows: &[(String, String, String)], // (id, title, app_id)
        colours: &CaelestiaColours,
    ) -> Div {
        let label = self.selected_title.as_deref().unwrap_or("Select target window...");

        let mut root = div()
            .flex()
            .flex_col()
            .w_full()
            .gap(px(8.0));

        let header = div()
            .flex()
            .items_center()
            .justify_between()
            .w_full()
            .p(px(PADDING_MEDIUM))
            .bg(rgba((colours.surface_container_high << 8) | 0xFF))
            .rounded(px(ROUNDING_MEDIUM))
            .cursor(CursorStyle::PointingHand)
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(SPACING_SMALL))
                    .child(
                        div()
                            .text_size(px(16.0))
                            .text_color(rgb(colours.secondary))
                            .child("desktop_windows"),
                    )
                    .child(
                        div()
                            .text_size(px(FONT_SIZE_LABEL_MEDIUM))
                            .text_color(rgb(colours.on_surface))
                            .child(label.to_string()),
                    ),
            )
            .child(
                div()
                    .text_size(px(14.0))
                    .text_color(rgb(colours.on_surface_variant))
                    .child(if self.is_open { "expand_less" } else { "expand_more" }),
            );

        root = root.child(header);

        if self.is_open {
            let mut list = div()
                .flex()
                .flex_col()
                .w_full()
                .max_h(px(200.0))
                .overflow_hidden()
                .p(px(8.0))
                .bg(rgba((colours.surface_container << 8) | 0xFF))
                .rounded(px(ROUNDING_MEDIUM))
                .gap(px(4.0));

            if windows.is_empty() {
                list = list.child(
                    div()
                        .p(px(8.0))
                        .text_size(px(12.0))
                        .text_color(rgb(colours.on_surface_variant))
                        .child("No open windows found"),
                );
            } else {
                for (id, title, app_id) in windows {
                    let is_active = self.selected_window_id.as_deref() == Some(id.as_str());
                    let row = div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .w_full()
                        .p(px(6.0))
                        .rounded(px(ROUNDING_SMALL))
                        .bg(if is_active {
                            rgba((colours.secondary_container << 8) | 0xFF)
                        } else {
                            rgba((colours.surface_container_high << 8) | 0xFF)
                        })
                        .cursor(CursorStyle::PointingHand)
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .child(
                                    div()
                                        .text_size(px(12.0))
                                        .text_color(rgb(colours.on_surface))
                                        .child(title.clone()),
                                )
                                .child(
                                    div()
                                        .text_size(px(10.0))
                                        .text_color(rgb(colours.on_surface_variant))
                                        .child(app_id.clone()),
                                ),
                        )
                        .child(
                            if is_active {
                                div()
                                    .text_size(px(14.0))
                                    .text_color(rgb(colours.secondary))
                                    .child("check")
                            } else {
                                div()
                            },
                        );
                    list = list.child(row);
                }
            }

            root = root.child(list);
        }

        root
    }
}
