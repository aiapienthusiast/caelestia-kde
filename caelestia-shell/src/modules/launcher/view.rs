//! Launcher View Component
//! Parity with `shell/modules/launcher/Content.qml` and `AppList.qml`.
//!
//! Renders search bar, category chips, scrollable application grid, and session controls.

use super::categories::{Categories, CATEGORY_DEFINITIONS};
use super::entry::DesktopEntry;
use super::executor::AppExecutor;
use super::searcher::AppSearcher;
use crate::components::icon::MaterialIcon;
use crate::components::styled_text::StyledTextExt;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{
    div, prelude::*, px, rgba, CursorStyle, FocusHandle, IntoElement, KeyDownEvent,
    Render, Styled, Window,
};

pub struct LauncherView {
    pub entries: Vec<DesktopEntry>,
    pub filtered_entries: Vec<DesktopEntry>,
    pub search_query: String,
    pub selected_category: String,
    pub focused_index: usize,
    pub colours: CaelestiaColours,
    pub is_open: bool,
    pub show_power_menu: bool,
    focus_handle: FocusHandle,
}

impl LauncherView {
    pub fn new(cx: &mut gpui::Context<Self>, colours: CaelestiaColours) -> Self {
        let entries = DesktopEntry::scan_system_applications();
        let mut launcher = Self {
            entries: entries.clone(),
            filtered_entries: entries,
            search_query: String::new(),
            selected_category: "all".to_string(),
            focused_index: 0,
            colours,
            is_open: false,
            show_power_menu: true,
            focus_handle: cx.focus_handle(),
        };
        launcher.refresh_results();
        launcher
    }

    /// Update search query and recalculate matching apps
    pub fn set_search_query(&mut self, query: &str) {
        self.search_query = query.to_string();
        self.refresh_results();
    }

    /// Select category filter
    pub fn select_category(&mut self, category_id: &str) {
        self.selected_category = category_id.to_string();
        self.refresh_results();
    }

    /// Filter applications based on category and search query
    pub fn refresh_results(&mut self) {
        let base = Categories::apps_for(&self.selected_category, &self.entries, &[]);
        let base_owned: Vec<DesktopEntry> = base.into_iter().cloned().collect();

        if self.search_query.is_empty() {
            self.filtered_entries = base_owned;
        } else {
            let matches = AppSearcher::search(&self.search_query, &base_owned);
            self.filtered_entries = matches.into_iter().cloned().collect();
        }

        if self.focused_index >= self.filtered_entries.len() {
            self.focused_index = 0;
        }
    }

    /// Launch currently focused application
    pub fn launch_focused(&mut self) -> Result<(), String> {
        if let Some(entry) = self.filtered_entries.get(self.focused_index) {
            self.is_open = false;
            AppExecutor::launch(entry)
        } else {
            Err("No application focused".to_string())
        }
    }

    /// Keyboard navigation
    pub fn handle_key_down(&mut self, event: &KeyDownEvent) {
        match event.keystroke.key.as_str() {
            "down" => {
                if self.focused_index + 1 < self.filtered_entries.len() {
                    self.focused_index += 1;
                }
            }
            "up" => {
                if self.focused_index > 0 {
                    self.focused_index -= 1;
                }
            }
            "enter" => {
                let _ = self.launch_focused();
            }
            "escape" => {
                self.is_open = false;
            }
            _ => {}
        }
    }
}

impl Render for LauncherView {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        let c = &self.colours;

        div()
            .track_focus(&self.focus_handle)
            .on_key_down(cx.listener(|this, event, _window, _cx| {
                this.handle_key_down(event);
            }))
            .w(px(640.0))
            .max_h(px(680.0))
            .rounded_3xl()
            .bg(rgba(c.surface_container_low))
            .border_1()
            .border_color(rgba(c.outline_variant))
            .p_4()
            .flex()
            .flex_col()
            .gap_3()
            // 1. Search Bar Header
            .child(
                div()
                    .w_full()
                    .h(px(48.0))
                    .rounded_2xl()
                    .bg(rgba(c.surface_container_highest))
                    .border_1()
                    .border_color(rgba(c.outline_variant))
                    .px_4()
                    .flex()
                    .items_center()
                    .gap_3()
                    .child(
                        MaterialIcon::new("search")
                            .text_color(rgba(c.on_surface_variant))
                            .text_size(px(20.0)),
                    )
                    .child(
                        div()
                            .text_color(if self.search_query.is_empty() {
                                rgba(c.on_surface_variant)
                            } else {
                                rgba(c.on_surface)
                            })
                            .text_sm()
                            .child(if self.search_query.is_empty() {
                                "Search applications...".to_string()
                            } else {
                                self.search_query.clone()
                            }),
                    ),
            )
            // 2. Category Chips Bar
            .child(
                div()
                    .w_full()
                    .flex()
                    .items_center()
                    .gap_2()
                    .overflow_hidden()
                    .children(
                        CATEGORY_DEFINITIONS
                            .iter()
                            .take(6)
                            .map(|cat| {
                                let is_active = self.selected_category == cat.id;
                                let cat_id = cat.id.to_string();
                                div()
                                    .px_3()
                                    .py_1()
                                    .rounded_full()
                                    .cursor(CursorStyle::PointingHand)
                                    .bg(if is_active {
                                        rgba(c.primary)
                                    } else {
                                        rgba(c.surface_container)
                                    })
                                    .text_color(if is_active {
                                        rgba(c.on_primary)
                                    } else {
                                        rgba(c.on_surface)
                                    })
                                    .text_xs()
                                    .child(cat.name)
                                    .on_mouse_down(
                                        gpui::MouseButton::Left,
                                        cx.listener(move |this, _, _, _cx| {
                                            this.select_category(&cat_id);
                                        }),
                                    )
                            }),
                    ),
            )
            // 3. Scrollable Applications List / Grid
            .child(
                div()
                    .w_full()
                    .flex_1()
                    .overflow_hidden()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .children(
                        self.filtered_entries
                            .iter()
                            .take(7)
                            .enumerate()
                            .map(|(idx, entry)| {
                                let is_focused = idx == self.focused_index;
                                let entry_id = entry.id.clone();
                                div()
                                    .w_full()
                                    .px_3()
                                    .py_2()
                                    .rounded_xl()
                                    .cursor(CursorStyle::PointingHand)
                                    .bg(if is_focused {
                                        rgba(c.surface_container_highest)
                                    } else {
                                        rgba(0x00000000)
                                    })
                                    .flex()
                                    .items_center()
                                    .gap_3()
                                    .child(
                                        div()
                                            .w(px(36.0))
                                            .h(px(36.0))
                                            .rounded_lg()
                                            .bg(rgba(c.primary_container))
                                            .text_color(rgba(c.on_primary_container))
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .child(
                                                MaterialIcon::new("apps")
                                                    .text_size(px(20.0)),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .flex_1()
                                            .flex()
                                            .flex_col()
                                            .child(
                                                div()
                                                    .text_color(rgba(c.on_surface))
                                                    .text_sm()
                                                    .font_weight(gpui::FontWeight::SEMIBOLD)
                                                    .child(entry.name.clone()),
                                            )
                                            .child(
                                                div()
                                                    .text_color(rgba(c.on_surface_variant))
                                                    .text_xs()
                                                    .child(if !entry.generic_name.is_empty() {
                                                        entry.generic_name.clone()
                                                    } else {
                                                        entry.comment.clone()
                                                    }),
                                            ),
                                    )
                                    .on_mouse_down(
                                        gpui::MouseButton::Left,
                                        cx.listener(move |this, _, _, _cx| {
                                            if let Some(pos) = this.filtered_entries.iter().position(|e| e.id == entry_id) {
                                                this.focused_index = pos;
                                                let _ = this.launch_focused();
                                            }
                                        }),
                                    )
                            }),
                    ),
            )
            // 4. Quick Session Controls Footer matching Content.qml
            .child(
                div()
                    .w_full()
                    .pt_2()
                    .border_t_1()
                    .border_color(rgba(c.outline_variant))
                    .flex()
                    .items_center()
                    .justify_end()
                    .gap_2()
                    .child(
                        div()
                            .px_3()
                            .py_1()
                            .rounded_lg()
                            .bg(rgba(c.surface_container))
                            .text_color(rgba(c.on_surface))
                            .text_xs()
                            .child("Lock"),
                    )
                    .child(
                        div()
                            .px_3()
                            .py_1()
                            .rounded_lg()
                            .bg(rgba(c.surface_container))
                            .text_color(rgba(c.on_surface))
                            .text_xs()
                            .child("Logout"),
                    )
                    .child(
                        div()
                            .px_3()
                            .py_1()
                            .rounded_lg()
                            .bg(rgba(c.error_container))
                            .text_color(rgba(c.on_error_container))
                            .text_xs()
                            .child("Power"),
                    ),
            )
    }
}
