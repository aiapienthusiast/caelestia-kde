//! Primary Dash Tab
//! Parity with `shell/modules/dashboard/Dash.qml`.
//! Composed cleanly of reusable components from `crate::components::*`:
//! 1. Weather Card (top left)
//! 2. User & System Card (top middle) using `Logo`
//! 3. DateTime vertical clock (bottom left)
//! 4. CalendarMonth Card (bottom middle) using `CalendarMonth` & `CalendarMonthState`
//! 5. Resources Card (bottom right of left column) using `CircularProgress`
//! 6. Media & Bongo Cat Card (right column) using `CoverArtView`, `CircularProgress`, and `IconButton`

use crate::components::calendar::calendar_month::{CalendarMonth, CalendarMonthState};
use crate::components::controls::button_base::{ButtonStyle, ButtonType};
use crate::components::controls::circular_progress::CircularProgress;
use crate::components::controls::icon_button::IconButton;
use crate::components::icon::MaterialIcon;
use crate::components::logo::Logo;
use crate::components::styled_text::{StyledText, StyledTextExt};
use crate::components::widgets::cover_art::{CoverArtState, CoverArtView};
use crate::infrastructure::paths::CaelestiaPaths;
use crate::infrastructure::sysinfo::SystemInfo;
use crate::infrastructure::theme::CaelestiaColours;
use crate::services::mpris::{MprisService, PlaybackStatus};
use crate::services::system_monitor::SystemMonitorService;
use crate::services::weather::WeatherData;
use chrono::{Local, NaiveDate};
use gpui::{div, img, prelude::*, px, rgba, CursorStyle, Div, MouseButton, Styled};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct QuickActionsState {
    pub wifi_active: bool,
    pub bluetooth_active: bool,
    pub night_active: bool,
    pub notifs_active: bool,
}

impl Default for QuickActionsState {
    fn default() -> Self {
        Self {
            wifi_active: true,
            bluetooth_active: true,
            night_active: false,
            notifs_active: true,
        }
    }
}

pub struct DashView;

impl DashView {
    pub fn render<V: 'static>(
        weather: &WeatherData,
        sysinfo: &SystemInfo,
        paths: &CaelestiaPaths,
        monitor: &SystemMonitorService,
        mpris: &MprisService,
        calendar_date: NaiveDate,
        c: &CaelestiaColours,
        cx: &mut gpui::Context<V>,
        on_shift_month: impl Fn(&mut V, i32, &mut gpui::Context<V>) + 'static + Copy,
        on_today: impl Fn(&mut V, &mut gpui::Context<V>) + 'static + Copy,
        on_play_pause: impl Fn(&mut V, &mut gpui::Context<V>) + 'static + Copy,
        on_prev: impl Fn(&mut V, &mut gpui::Context<V>) + 'static + Copy,
        on_next: impl Fn(&mut V, &mut gpui::Context<V>) + 'static + Copy,
    ) -> Div {
        let uptime_str = SystemInfo::uptime_formatted();

        div()
            .flex()
            .gap(px(14.0))
            .w_full()
            .h(px(474.0))
            .child(
                // Left Column (spans 2 rows: Top Row and Bottom Row)
                div()
                    .flex()
                    .flex_col()
                    .gap(px(14.0))
                    .flex_1()
                    .h_full()
                    .child(
                        // Top Row: Weather (275px) + User (flex_1)
                        div()
                            .flex()
                            .gap(px(14.0))
                            .w_full()
                            .h(px(140.0))
                            .child(Self::render_weather_card(weather, c))
                            .child(Self::render_user_card(sysinfo, &uptime_str, paths, c)),
                    )
                    .child(
                        // Bottom Row: DateTime (110px) + Calendar (flex_1) + Resources (95px)
                        div()
                            .flex()
                            .gap(px(14.0))
                            .w_full()
                            .h(px(320.0))
                            .child(Self::render_datetime_card(c))
                            .child(Self::render_calendar_card(
                                calendar_date,
                                c,
                                cx,
                                on_shift_month,
                                on_today,
                            ))
                            .child(Self::render_resources_card(monitor, c)),
                    ),
            )
            .child(
                // Right Column: Media & Bongo Cat Card (spans full height 474px, width 200px)
                Self::render_media_card(mpris, c, cx, on_play_pause, on_prev, on_next),
            )
    }

    /// 1. Weather Card matching `SmallWeather.qml`
    fn render_weather_card(weather: &WeatherData, c: &CaelestiaColours) -> Div {
        let icon_name = if weather.icon.is_empty() {
            "cloud"
        } else {
            weather.icon.as_str()
        };
        let temp = if weather.temp.is_empty() {
            "30°C".to_string()
        } else {
            weather.temp.clone()
        };
        let desc = if weather.condition.is_empty() {
            "Overcast".to_string()
        } else {
            weather.condition.clone()
        };

        div()
            .w(px(275.0))
            .h(px(140.0))
            .p(px(20.0))
            .rounded(px(28.0))
            .bg(rgba(c.surface_container))
            .flex()
            .items_center()
            .justify_center()
            .gap(px(20.0))
            .child(
                MaterialIcon::new(icon_name)
                    .color(c.secondary)
                    .text_size(px(52.0)),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap(px(4.0))
                    .child(
                        StyledText::new(&temp)
                            .color(c.primary)
                            .font_size(28.0)
                            .font_weight(gpui::FontWeight::BOLD),
                    )
                    .child(
                        StyledText::new(&desc)
                            .color(c.on_surface_variant)
                            .font_size(13.0),
                    ),
            )
    }

    /// 2. User & System Card matching `User.qml` using `components::logo::Logo`
    fn render_user_card(
        _sysinfo: &SystemInfo,
        uptime: &str,
        _paths: &CaelestiaPaths,
        c: &CaelestiaColours,
    ) -> Div {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let face_path = format!("{}/.face", home);
        let has_face = std::path::Path::new(&face_path).exists();

        div()
            .flex_1()
            .h(px(140.0))
            .p(px(16.0))
            .rounded(px(24.0))
            .bg(rgba(c.surface_container))
            .flex()
            .items_center()
            .justify_between()
            .child(
                // Left: Logo badge overlapping avatar
                div()
                    .flex()
                    .items_center()
                    .gap(px(12.0))
                    .child(
                        // Caelestia Logo badge using components::logo::Logo
                        div()
                            .size(px(40.0))
                            .rounded(px(14.0))
                            .bg(rgba(c.primary_container))
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(Logo::new().with_size(24.0, 24.0)),
                    )
                    .child(
                        // Avatar container
                        div()
                            .size(px(68.0))
                            .rounded(px(22.0))
                            .bg(rgba(c.surface_container_highest))
                            .flex()
                            .items_center()
                            .justify_center()
                            .overflow_hidden()
                            .child(if has_face {
                                img(std::path::PathBuf::from(face_path))
                                    .size_full()
                                    .into_any_element()
                            } else {
                                MaterialIcon::new("person_add")
                                    .color(c.on_surface_variant)
                                    .text_size(px(32.0))
                                    .into_any_element()
                            }),
                    ),
            )
            .child(
                // Right: WM badge top and Uptime bottom
                div()
                    .flex()
                    .flex_col()
                    .items_end()
                    .justify_between()
                    .h_full()
                    .child(
                        // Top: KDE Speech bubble
                        div()
                            .flex()
                            .items_center()
                            .gap(px(4.0))
                            .child(
                                div()
                                    .size(px(6.0))
                                    .rounded_full()
                                    .bg(rgba(c.secondary_container)),
                            )
                            .child(
                                div()
                                    .size(px(9.0))
                                    .rounded_full()
                                    .bg(rgba(c.secondary_container)),
                            )
                            .child(
                                div()
                                    .px(px(10.0))
                                    .py(px(4.0))
                                    .rounded(px(12.0))
                                    .bg(rgba(c.secondary_container))
                                    .flex()
                                    .items_center()
                                    .gap(px(6.0))
                                    .child(
                                        MaterialIcon::new("desktop_windows")
                                            .color(c.on_secondary_container)
                                            .text_size(px(13.0)),
                                    )
                                    .child(
                                        StyledText::new("KDE...")
                                            .color(c.on_secondary_container)
                                            .font_size(12.0)
                                            .font_weight(gpui::FontWeight::SEMIBOLD),
                                    ),
                            ),
                    )
                    .child(
                        // Bottom: Uptime badge
                        div()
                            .flex()
                            .items_center()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .size(px(28.0))
                                    .rounded_full()
                                    .bg(rgba(c.secondary_container))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .child(
                                        MaterialIcon::new("update")
                                            .color(c.on_secondary_container)
                                            .text_size(px(15.0)),
                                    ),
                            )
                            .child(
                                StyledText::new(&format!("up {}", uptime))
                                    .color(c.on_surface_variant)
                                    .font_size(13.0),
                            ),
                    ),
            )
    }

    /// 3. DateTime Vertical Clock Card matching `DateTime.qml`
    fn render_datetime_card(c: &CaelestiaColours) -> Div {
        let now = Local::now();
        let hour = now.format("%H").to_string();
        let minute = now.format("%M").to_string();

        div()
            .w(px(110.0))
            .h(px(320.0))
            .rounded(px(20.0))
            .bg(rgba(c.surface_container))
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap(px(2.0))
            .child(
                StyledText::new(&hour)
                    .color(c.secondary)
                    .font_size(36.0)
                    .font_weight(gpui::FontWeight::BOLD),
            )
            .child(
                StyledText::new("•••")
                    .color(c.primary)
                    .font_size(24.0)
                    .font_weight(gpui::FontWeight::BOLD),
            )
            .child(
                StyledText::new(&minute)
                    .color(c.secondary)
                    .font_size(36.0)
                    .font_weight(gpui::FontWeight::BOLD),
            )
    }

    /// 4. Calendar Month Card matching `Calendar.qml` using `components::calendar::calendar_month::CalendarMonth`
    fn render_calendar_card<V: 'static>(
        view_date: NaiveDate,
        c: &CaelestiaColours,
        cx: &mut gpui::Context<V>,
        on_shift_month: impl Fn(&mut V, i32, &mut gpui::Context<V>) + 'static + Copy,
        on_today: impl Fn(&mut V, &mut gpui::Context<V>) + 'static + Copy,
    ) -> Div {
        let cal_state = CalendarMonthState::new(view_date)
            .with_start_on_sunday(true)
            .with_selectable(true);

        let calendar_component = CalendarMonth::new(cal_state)
            .with_colours(c.clone())
            .on_prev(cx.listener(move |view, _, _, cx| on_shift_month(view, -1, cx)))
            .on_next(cx.listener(move |view, _, _, cx| on_shift_month(view, 1, cx)))
            .on_today(cx.listener(move |view, _, _, cx| on_today(view, cx)));

        div()
            .flex_1()
            .h(px(320.0))
            .p(px(16.0))
            .rounded(px(24.0))
            .bg(rgba(c.surface_container))
            .child(calendar_component)
    }

    /// 5. Resources Card matching `Resources.qml` using `components::controls::circular_progress::CircularProgress`
    fn render_resources_card(monitor: &SystemMonitorService, c: &CaelestiaColours) -> Div {
        let cpu_val = (monitor.state.cpu_usage / 100.0).clamp(0.0, 1.0);
        let ram_used = monitor.ram_used_gib();
        let ram_total = monitor.ram_total_gib();
        let ram_val = if ram_total > 0.0 {
            (ram_used / ram_total).clamp(0.0, 1.0) as f32
        } else {
            0.0
        };
        let disk_val = (monitor.disk_percent() / 100.0).clamp(0.0, 1.0) as f32;

        div()
            .w(px(95.0))
            .h(px(320.0))
            .p(px(12.0))
            .rounded(px(20.0))
            .bg(rgba(c.surface_container))
            .flex()
            .flex_col()
            .items_center()
            .justify_around()
            .child(
                CircularProgress::new(cpu_val)
                    .with_size(64.0)
                    .with_stroke_width(5.0)
                    .with_colors(c.primary, c.surface_container_highest)
                    .with_child(
                        MaterialIcon::new("memory")
                            .color(c.primary)
                            .text_size(px(22.0)),
                    ),
            )
            .child(
                CircularProgress::new(ram_val)
                    .with_size(64.0)
                    .with_stroke_width(5.0)
                    .with_colors(c.secondary, c.surface_container_highest)
                    .with_child(
                        MaterialIcon::new("memory_alt")
                            .color(c.secondary)
                            .text_size(px(22.0)),
                    ),
            )
            .child(
                CircularProgress::new(disk_val)
                    .with_size(64.0)
                    .with_stroke_width(5.0)
                    .with_colors(c.primary, c.surface_container_highest)
                    .with_child(
                        MaterialIcon::new("hard_disk")
                            .color(c.primary)
                            .text_size(px(22.0)),
                    ),
            )
    }

    /// 6. Media & Bongo Cat Card matching `dash/Media.qml` using `CoverArtView`, `CircularProgress`, and `IconButton`
    fn render_media_card<V: 'static>(
        mpris: &MprisService,
        c: &CaelestiaColours,
        cx: &mut gpui::Context<V>,
        on_play_pause: impl Fn(&mut V, &mut gpui::Context<V>) + 'static + Copy,
        on_prev: impl Fn(&mut V, &mut gpui::Context<V>) + 'static + Copy,
        on_next: impl Fn(&mut V, &mut gpui::Context<V>) + 'static + Copy,
    ) -> Div {
        let is_playing = mpris.current_track.status == PlaybackStatus::Playing;
        let title = if mpris.current_track.title.is_empty() {
            "No media"
        } else {
            &mpris.current_track.title
        };
        let artist = if mpris.current_track.artist.is_empty() {
            "Unknown artist"
        } else {
            &mpris.current_track.artist
        };

        let bongocat_path = "/home/ladybug-me/caelestia-kde/shell/assets/bongocat.gif";
        let has_bongo = std::path::Path::new(bongocat_path).exists();

        let progress = if mpris.current_track.duration_secs > 0.0 {
            (mpris.current_track.position_secs / mpris.current_track.duration_secs).clamp(0.0, 1.0) as f32
        } else {
            0.0
        };

        let cover_state = CoverArtState::new()
            .with_art(mpris.current_track.art_url.clone())
            .with_playing(is_playing)
            .with_size(104.0);

        let tonal_style = ButtonStyle {
            button_type: ButtonType::Tonal,
            is_round: true,
            ..Default::default()
        };
        let filled_style = ButtonStyle {
            button_type: ButtonType::Filled,
            is_round: true,
            ..Default::default()
        };

        let prev_btn = IconButton::render("skip_previous", &tonal_style, c, Some(32.0))
            .cursor(CursorStyle::PointingHand)
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |view, _, _, cx| on_prev(view, cx)),
            );

        let play_btn = IconButton::render(
            if is_playing { "pause" } else { "play_arrow" },
            &filled_style,
            c,
            Some(40.0),
        )
        .cursor(CursorStyle::PointingHand)
        .on_mouse_down(
            MouseButton::Left,
            cx.listener(move |view, _, _, cx| on_play_pause(view, cx)),
        );

        let next_btn = IconButton::render("skip_next", &tonal_style, c, Some(32.0))
            .cursor(CursorStyle::PointingHand)
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |view, _, _, cx| on_next(view, cx)),
            );

        div()
            .w(px(200.0))
            .h(px(474.0))
            .p(px(16.0))
            .rounded(px(32.0))
            .bg(rgba(c.surface_container))
            .flex()
            .flex_col()
            .items_center()
            .justify_between()
            .child(
                // Top Section: CircularProgress with CoverArt + Title + Controls
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap(px(10.0))
                    .w_full()
                    .child(
                        // CircularProgress wrapping CoverArtView
                        CircularProgress::new(progress)
                            .with_size(120.0)
                            .with_stroke_width(4.0)
                            .with_colors(c.primary, c.surface_container_highest)
                            .with_child(CoverArtView::new(cover_state)),
                    )
                    .child(
                        // Track Title & Artist
                        div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .gap(px(2.0))
                            .child(
                                StyledText::new(title)
                                    .color(c.primary)
                                    .font_size(14.0)
                                    .font_weight(gpui::FontWeight::BOLD),
                            )
                            .child(
                                StyledText::new(artist)
                                    .color(c.secondary)
                                    .font_size(12.0),
                            ),
                    )
                    .child(
                        // Player Controls Row using IconButtons
                        div()
                            .flex()
                            .items_center()
                            .justify_center()
                            .gap(px(8.0))
                            .child(prev_btn)
                            .child(play_btn)
                            .child(next_btn),
                    ),
            )
            .child(
                // Bottom Section: Bongo Cat
                div()
                    .w_full()
                    .h(px(100.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .overflow_hidden()
                    .child(if has_bongo {
                        img(std::path::PathBuf::from(bongocat_path))
                            .w(px(150.0))
                            .h(px(85.0))
                            .into_any_element()
                    } else {
                        div().text_size(px(42.0)).child("🐱").into_any_element()
                    }),
            )
    }
}
