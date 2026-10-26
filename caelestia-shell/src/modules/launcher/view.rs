//! Launcher View Component
//! Parity with `shell/modules/launcher/Content.qml`, `ContentList.qml`, and `AppBrowserGrid.qml`.
//!
//! Renders category sidebar on the left, app grid on the right, and the bottom notched search bar.

use super::categories::Categories;
use super::entry::DesktopEntry;
use super::executor::AppExecutor;
use super::searcher::AppSearcher;
use crate::components::icon::{IconSize, MaterialIcon};
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{
    div, img, prelude::*, px, rgba, svg, CursorStyle, FocusHandle, FontWeight, IntoElement,
    KeyDownEvent, MouseButton, Render, Styled, Window,
};
use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub enum ResolvedAppIcon {
    Png(PathBuf),
    Svg(PathBuf),
    Glyph(&'static str),
}

/// Resolves an application icon name to a local filesystem PNG/SVG, or fallback glyph
pub fn resolve_app_icon(icon_name: &str, app_id: &str) -> ResolvedAppIcon {
    let p = Path::new(icon_name);
    if p.is_absolute() && p.exists() {
        if icon_name.ends_with(".svg") {
            return ResolvedAppIcon::Svg(p.to_path_buf());
        } else {
            return ResolvedAppIcon::Png(p.to_path_buf());
        }
    }

    let base_name = app_id.trim_end_matches(".desktop");
    let candidates = [icon_name, app_id, base_name];

    for name in candidates {
        if name.is_empty() {
            continue;
        }

        // Check PNGs first
        let png_paths = [
            format!("/usr/share/icons/hicolor/48x48/apps/{}.png", name),
            format!("/usr/share/icons/hicolor/64x64/apps/{}.png", name),
            format!("/usr/share/icons/hicolor/128x128/apps/{}.png", name),
            format!("/usr/share/pixmaps/{}.png", name),
            format!("/usr/share/icons/hicolor/256x256/apps/{}.png", name),
        ];
        for p_str in &png_paths {
            let path = PathBuf::from(p_str);
            if path.exists() {
                return ResolvedAppIcon::Png(path);
            }
        }

        // Check SVGs (check Papirus theme first as standard on Caelestia)
        let svg_paths = [
            format!("/usr/share/icons/Papirus/48x48/apps/{}.svg", name),
            format!("/usr/share/icons/Papirus/64x64/apps/{}.svg", name),
            format!("/usr/share/icons/hicolor/scalable/apps/{}.svg", name),
            format!("/usr/share/icons/breeze/apps/48/{}.svg", name),
            format!("/usr/share/pixmaps/{}.svg", name),
        ];
        for s_str in &svg_paths {
            let path = PathBuf::from(s_str);
            if path.exists() {
                return ResolvedAppIcon::Svg(path);
            }
        }
    }

    let lower_icon = icon_name.to_ascii_lowercase();
    let lower_id = app_id.to_ascii_lowercase();
    if lower_icon.contains("firefox") || lower_id.contains("firefox") {
        ResolvedAppIcon::Glyph("public")
    } else if lower_icon.contains("dolphin") || lower_id.contains("dolphin") || lower_icon.contains("folder") {
        ResolvedAppIcon::Glyph("folder")
    } else if lower_icon.contains("term") || lower_id.contains("alacritty") {
        ResolvedAppIcon::Glyph("terminal")
    } else if lower_icon.contains("code") || lower_id.contains("code") {
        ResolvedAppIcon::Glyph("code")
    } else {
        ResolvedAppIcon::Glyph("apps")
    }
}

pub struct LauncherView {
    pub entries: Vec<DesktopEntry>,
    pub filtered_entries: Vec<DesktopEntry>,
    pub search_query: String,
    pub selected_category: String,
    pub focused_index: usize,
    pub colours: CaelestiaColours,
    pub is_open: bool,
    pub show_power_menu: bool,
    pub favourite_ids: Vec<String>,
    focus_handle: FocusHandle,
}

impl LauncherView {
    pub fn new(cx: &mut gpui::Context<Self>, colours: CaelestiaColours) -> Self {
        let entries = DesktopEntry::scan_system_applications();
        let mut launcher = Self {
            entries: entries.clone(),
            filtered_entries: Vec::new(),
            search_query: String::new(),
            selected_category: "favorites".to_string(),
            focused_index: 0,
            colours,
            is_open: true,
            show_power_menu: false,
            favourite_ids: vec![
                "firefox".to_string(),
                "org.kde.dolphin".to_string(),
                "dolphin".to_string(),
            ],
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
        self.search_query.clear();
        self.refresh_results();
    }

    /// Filter applications based on category and search query
    pub fn refresh_results(&mut self) {
        if self.search_query.is_empty() {
            let base = Categories::apps_for(&self.selected_category, &self.entries, &self.favourite_ids);
            let mut base_owned: Vec<DesktopEntry> = base.into_iter().cloned().collect();

            // Fallback for favorites if explicit ID match yielded nothing
            if self.selected_category == "favorites" && base_owned.is_empty() {
                for entry in &self.entries {
                    let id_l = entry.id.to_ascii_lowercase();
                    let name_l = entry.name.to_ascii_lowercase();
                    if id_l.contains("firefox") || name_l == "firefox" || id_l.contains("dolphin") || name_l == "dolphin" {
                        base_owned.push(entry.clone());
                    }
                }
            }

            if self.selected_category == "favorites" {
                base_owned.sort_by_key(|e| {
                    let base_id = e.id.trim_end_matches(".desktop");
                    self.favourite_ids
                        .iter()
                        .position(|f| f == &e.id || f == &e.name || f == base_id || e.name.eq_ignore_ascii_case(f))
                        .unwrap_or(999)
                });
            }

            self.filtered_entries = base_owned;
        } else {
            let matches = AppSearcher::search(&self.search_query, &self.entries);
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
            "backspace" => {
                self.search_query.pop();
                self.refresh_results();
            }
            key if key.len() == 1 => {
                self.search_query.push_str(key);
                self.refresh_results();
            }
            _ => {}
        }
    }
}

impl Render for LauncherView {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        let c = &self.colours;
        let visible_categories = Categories::visible_categories(&self.entries);

        // Outermost layer-shell backdrop overlay
        div()
            .id("launcher-backdrop")
            .track_focus(&self.focus_handle)
            .on_key_down(cx.listener(|this, event, _window, _cx| {
                this.handle_key_down(event);
            }))
            .w_full()
            .h_full()
            .flex()
            .flex_col()
            .justify_end()
            .items_center()
            .pb(px(52.0))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, _, _| {
                    this.is_open = false;
                }),
            )
            .child(
                // Floating Launcher Card Dialog
                div()
                    .id("launcher-card")
                    .on_mouse_down(MouseButton::Left, |_, _, cx| {
                        cx.stop_propagation();
                    })
                    .w(px(640.0))
                    .h(px(460.0))
                    .rounded(px(24.0))
                    .bg(rgba(c.surface_container_low))
                    .border_1()
                    .border_color(rgba(c.outline_variant))
                    .p(px(16.0))
                    .flex()
                    .flex_col()
                    .justify_between()
                    // 1. Top Section: Sidebar + App Grid
                    .child(
                        div()
                            .flex_1()
                            .w_full()
                            .flex()
                            .flex_row()
                            .gap(px(16.0))
                            .overflow_hidden()
                            .mb(px(12.0))
                            // Left Category Sidebar
                            .child(
                                div()
                                    .id("categories-sidebar")
                                    .w(px(175.0))
                                    .h_full()
                                    .flex()
                                    .flex_col()
                                    .gap(px(3.0))
                                    .overflow_y_scroll()
                                    .children(
                                        visible_categories.iter().map(|cat| {
                                            let is_active = self.selected_category == cat.id;
                                            let cat_id = cat.id.to_string();
                                            div()
                                                .w_full()
                                                .h(px(38.0))
                                                .px(px(12.0))
                                                .rounded(px(12.0))
                                                .cursor(CursorStyle::PointingHand)
                                                .flex()
                                                .items_center()
                                                .gap(px(12.0))
                                                .bg(if is_active {
                                                    rgba(c.surface_container_high)
                                                } else {
                                                    rgba(0x00000000)
                                                })
                                                .child(
                                                    MaterialIcon::with_size(
                                                        cat.icon,
                                                        IconSize::Small,
                                                        if is_active { c.primary } else { c.on_surface_variant },
                                                    ),
                                                )
                                                .child(
                                                    div()
                                                        .text_sm()
                                                        .font_weight(if is_active {
                                                            FontWeight::MEDIUM
                                                        } else {
                                                            FontWeight::NORMAL
                                                        })
                                                        .text_color(rgba(if is_active {
                                                            c.on_surface
                                                        } else {
                                                            c.on_surface_variant
                                                        }))
                                                        .overflow_hidden()
                                                        .truncate()
                                                        .child(cat.name),
                                                )
                                                .on_mouse_down(
                                                    MouseButton::Left,
                                                    cx.listener(move |this, _, _, _cx| {
                                                        this.select_category(&cat_id);
                                                    }),
                                                )
                                        }),
                                    ),
                            )
                            // Right Applications Grid Area
                            .child(
                                div()
                                    .id("apps-grid-container")
                                    .flex_1()
                                    .h_full()
                                    .flex()
                                    .flex_col()
                                    .overflow_y_scroll()
                                    .child(if self.filtered_entries.is_empty() {
                                        div()
                                            .w_full()
                                            .h_full()
                                            .flex()
                                            .flex_col()
                                            .items_center()
                                            .justify_center()
                                            .gap(px(8.0))
                                            .child(MaterialIcon::with_size("apps", IconSize::Large, c.outline))
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgba(c.outline))
                                                    .child("No apps in this category"),
                                            )
                                    } else {
                                        div()
                                            .w_full()
                                            .flex()
                                            .flex_row()
                                            .flex_wrap()
                                            .gap(px(10.0))
                                            .p(px(2.0))
                                            .children(
                                                self.filtered_entries.iter().enumerate().map(|(idx, entry)| {
                                                    let is_focused = idx == self.focused_index;
                                                    let is_fav = self.favourite_ids.iter().any(|fav| {
                                                        fav == &entry.id || fav == &entry.name
                                                    }) || self.selected_category == "favorites";
                                                    let resolved_icon = resolve_app_icon(&entry.icon, &entry.id);
                                                    let entry_id = entry.id.clone();

                                                    div()
                                                        .w(px(84.0))
                                                        .h(px(96.0))
                                                        .relative()
                                                        .rounded(px(14.0))
                                                        .p(px(6.0))
                                                        .cursor(CursorStyle::PointingHand)
                                                        .bg(if is_focused {
                                                            rgba(c.surface_container_highest)
                                                        } else {
                                                            rgba(0x00000000)
                                                        })
                                                        .flex()
                                                        .flex_col()
                                                        .items_center()
                                                        .justify_between()
                                                        // Favorite badge at top-right
                                                        .when(is_fav, |d| {
                                                            d.child(
                                                                div()
                                                                    .absolute()
                                                                    .top(px(4.0))
                                                                    .right(px(6.0))
                                                                    .child(MaterialIcon::with_size(
                                                                        "favorite",
                                                                        IconSize::Custom(12.0),
                                                                        c.primary,
                                                                    )),
                                                            )
                                                        })
                                                        // Large App Icon
                                                        .child(
                                                            div()
                                                                .w(px(48.0))
                                                                .h(px(48.0))
                                                                .flex()
                                                                .items_center()
                                                                .justify_center()
                                                                .child(match resolved_icon {
                                                                    ResolvedAppIcon::Png(ref path) => {
                                                                        img(path.as_path()).w(px(44.0)).h(px(44.0)).into_any_element()
                                                                    }
                                                                    ResolvedAppIcon::Svg(ref path) => {
                                                                        svg()
                                                                            .external_path(path.to_string_lossy().to_string())
                                                                            .w(px(44.0))
                                                                            .h(px(44.0))
                                                                            .into_any_element()
                                                                    }
                                                                    ResolvedAppIcon::Glyph(glyph) => {
                                                                        MaterialIcon::with_size(glyph, IconSize::Large, c.primary)
                                                                            .into_any_element()
                                                                    }
                                                                }),
                                                        )
                                                        // App Name Label
                                                        .child(
                                                            div()
                                                                .text_xs()
                                                                .text_color(rgba(c.on_surface))
                                                                .text_center()
                                                                .line_height(px(13.0))
                                                                .max_w_full()
                                                                .overflow_hidden()
                                                                .truncate()
                                                                .child(entry.name.clone()),
                                                        )
                                                        .on_mouse_down(
                                                            MouseButton::Left,
                                                            cx.listener(move |this, _, _, _cx| {
                                                                if let Some(pos) = this
                                                                    .filtered_entries
                                                                    .iter()
                                                                    .position(|e| e.id == entry_id)
                                                                {
                                                                    this.focused_index = pos;
                                                                    let _ = this.launch_focused();
                                                                }
                                                            }),
                                                        )
                                                }),
                                            )
                                    }),
                            ),
                    )
                    // 2. Bottom Section: Outlined Notched Search Bar Row
                    .child(
                        div()
                            .w_full()
                            .h(px(44.0))
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap(px(10.0))
                            // Search Icon Button
                            .child(
                                div()
                                    .w(px(40.0))
                                    .h(px(40.0))
                                    .rounded_full()
                                    .bg(rgba(c.surface_container_highest))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .child(MaterialIcon::with_size(
                                        "search",
                                        IconSize::Custom(20.0),
                                        c.on_surface_variant,
                                    )),
                            )
                            // Outlined Text Box with Notched Cutout Label
                            .child(
                                div()
                                    .flex_1()
                                    .h(px(42.0))
                                    .relative()
                                    .rounded(px(14.0))
                                    .border_1()
                                    .border_color(rgba(if self.search_query.is_empty() {
                                        c.outline_variant
                                    } else {
                                        c.primary
                                    }))
                                    .px(px(12.0))
                                    .flex()
                                    .items_center()
                                    // Floating Notch Label
                                    .child(
                                        div()
                                            .absolute()
                                            .top(px(-9.0))
                                            .left(px(10.0))
                                            .px(px(4.0))
                                            .bg(rgba(c.surface_container_low))
                                            .text_xs()
                                            .text_color(rgba(if self.search_query.is_empty() {
                                                c.on_surface_variant
                                            } else {
                                                c.primary
                                            }))
                                            .child("Type \">\" for commands"),
                                    )
                                    // Current Query
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgba(c.on_surface))
                                            .child(if self.search_query.is_empty() {
                                                "".to_string()
                                            } else {
                                                self.search_query.clone()
                                            }),
                                    ),
                            )
                            // Hamburger Menu Button
                            .child(
                                div()
                                    .w(px(40.0))
                                    .h(px(40.0))
                                    .rounded_full()
                                    .bg(rgba(c.surface_container_highest))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .cursor(CursorStyle::PointingHand)
                                    .child(MaterialIcon::with_size(
                                        "menu",
                                        IconSize::Custom(20.0),
                                        c.on_surface_variant,
                                    )),
                            ),
                    ),
            )
    }
}

