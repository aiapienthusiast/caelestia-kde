//! Primary Dash Tab
//! Parity with `shell/modules/dashboard/Dash.qml`.
//! Contains 6 cards matching upstream:
//! 1. SmallWeather (top left)
//! 2. User & System (top middle)
//! 3. DateTime vertical clock (bottom left)
//! 4. CalendarMonth 42-day grid (bottom middle)
//! 5. Resources 3-ring circular gauges for CPU/RAM/Disk (bottom right of left column)
//! 6. Media & Bongo Cat (right column spanning full height)

use crate::components::icon::MaterialIcon;
use crate::components::styled_text::{StyledText, StyledTextExt};
use crate::infrastructure::paths::CaelestiaPaths;
use crate::infrastructure::sysinfo::SystemInfo;
use crate::infrastructure::theme::CaelestiaColours;
use crate::services::mpris::{MprisService, PlaybackStatus};
use crate::services::system_monitor::SystemMonitorService;
use crate::services::weather::WeatherData;
use chrono::{Datelike, Days, Local, NaiveDate};
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

    /// 2. User & System Card matching `User.qml`
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
                        // Gem logo badge
                        div()
                            .size(px(36.0))
                            .rounded(px(12.0))
                            .bg(rgba(c.primary_container))
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(
                                MaterialIcon::new("auto_awesome")
                                    .color(c.on_primary_container)
                                    .text_size(px(18.0)),
                            ),
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

    /// 4. Calendar Month Card matching `Calendar.qml` and `CalendarMonth.qml`
    fn render_calendar_card<V: 'static>(
        view_date: NaiveDate,
        c: &CaelestiaColours,
        cx: &mut gpui::Context<V>,
        on_shift_month: impl Fn(&mut V, i32, &mut gpui::Context<V>) + 'static + Copy,
        on_today: impl Fn(&mut V, &mut gpui::Context<V>) + 'static + Copy,
    ) -> Div {
        let curr_year = view_date.year();
        let curr_month = view_date.month();
        let today = Local::now().date_naive();

        let first_of_month =
            NaiveDate::from_ymd_opt(curr_year, curr_month, 1).unwrap_or(view_date);
        let first_weekday_idx = first_of_month.weekday().num_days_from_sunday();
        let start_date = first_of_month
            .checked_sub_days(Days::new(first_weekday_idx as u64))
            .unwrap_or(first_of_month);

        let mut days_cells = Vec::with_capacity(42);
        for i in 0..42 {
            let cur = start_date.checked_add_days(Days::new(i)).unwrap_or(start_date);
            let in_month = cur.month() == curr_month;
            let is_today = cur == today;
            days_cells.push((cur.day(), in_month, is_today));
        }

        let weekdays = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];

        div()
            .flex_1()
            .h(px(320.0))
            .p(px(16.0))
            .rounded(px(24.0))
            .bg(rgba(c.surface_container))
            .flex()
            .flex_col()
            .justify_between()
            .child(
                // Month Navigation Header Row
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .w_full()
                    .child(
                        div()
                            .p(px(4.0))
                            .cursor(CursorStyle::PointingHand)
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |view, _, _, cx| on_shift_month(view, -1, cx)),
                            )
                            .child(
                                MaterialIcon::new("chevron_left")
                                    .color(c.primary)
                                    .text_size(px(18.0)),
                            ),
                    )
                    .child(
                        div()
                            .cursor(CursorStyle::PointingHand)
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |view, _, _, cx| on_today(view, cx)),
                            )
                            .child(
                                StyledText::new(&view_date.format("%B %Y").to_string())
                                    .color(c.primary)
                                    .font_size(16.0)
                                    .font_weight(gpui::FontWeight::BOLD),
                            ),
                    )
                    .child(
                        div()
                            .p(px(4.0))
                            .cursor(CursorStyle::PointingHand)
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |view, _, _, cx| on_shift_month(view, 1, cx)),
                            )
                            .child(
                                MaterialIcon::new("chevron_right")
                                    .color(c.primary)
                                    .text_size(px(18.0)),
                            ),
                    ),
            )
            .child(
                // Weekday Header Row
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .w_full()
                    .children(weekdays.iter().map(|&name| {
                        div()
                            .flex_1()
                            .flex()
                            .justify_center()
                            .child(
                                StyledText::new(name)
                                    .color(c.primary)
                                    .font_size(12.0)
                                    .font_weight(gpui::FontWeight::SEMIBOLD),
                            )
                    })),
            )
            .child(
                // 42-day Grid (6 rows of 7 days)
                div()
                    .flex()
                    .flex_col()
                    .gap(px(4.0))
                    .w_full()
                    .children((0..6).map(|row| {
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .w_full()
                            .children((0..7).map(|col| {
                                let (day_num, in_month, is_today) = days_cells[row * 7 + col];
                                let cell_content = if is_today {
                                    div()
                                        .size(px(26.0))
                                        .rounded(px(10.0))
                                        .bg(rgba(c.secondary_container))
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .child(
                                            StyledText::new(&day_num.to_string())
                                                .color(c.on_secondary_container)
                                                .font_size(13.0)
                                                .font_weight(gpui::FontWeight::BOLD),
                                        )
                                } else if in_month {
                                    div()
                                        .size(px(26.0))
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .child(
                                            StyledText::new(&day_num.to_string())
                                                .color(c.on_surface_variant)
                                                .font_size(13.0),
                                        )
                                } else {
                                    div()
                                        .size(px(26.0))
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .child(
                                            StyledText::new(&day_num.to_string())
                                                .color(c.outline)
                                                .font_size(13.0)
                                                .opacity(0.4),
                                        )
                                };
                                div()
                                    .flex_1()
                                    .flex()
                                    .justify_center()
                                    .child(cell_content)
                            }))
                    })),
            )
    }

    /// 5. Resources Card matching `Resources.qml`
    fn render_resources_card(monitor: &SystemMonitorService, c: &CaelestiaColours) -> Div {
        let cpu_pct = monitor.state.cpu_usage.round() as u32;
        let ram_used = monitor.ram_used_gib();
        let ram_total = monitor.ram_total_gib();
        let ram_pct = if ram_total > 0.0 {
            ((ram_used / ram_total) * 100.0).round() as u32
        } else {
            0
        };
        let disk_pct = monitor.disk_percent().round() as u32;

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
            .child(Self::render_circular_gauge("memory", cpu_pct, c.primary, c))
            .child(Self::render_circular_gauge(
                "memory_alt",
                ram_pct,
                c.secondary,
                c,
            ))
            .child(Self::render_circular_gauge(
                "hard_disk",
                disk_pct,
                c.primary,
                c,
            ))
    }

    fn render_circular_gauge(
        icon: &'static str,
        pct: u32,
        fg_color: u32,
        c: &CaelestiaColours,
    ) -> Div {
        div()
            .size(px(62.0))
            .rounded_full()
            .bg(rgba(c.surface_container_low))
            .border_3()
            .border_color(rgba(if pct > 0 { fg_color } else { c.surface_container_highest }))
            .flex()
            .items_center()
            .justify_center()
            .child(
                MaterialIcon::new(icon)
                    .color(fg_color)
                    .text_size(px(22.0)),
            )
    }

    /// 6. Media & Bongo Cat Card matching `dash/Media.qml`
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

        let bongocat_path = "/home/ladybug-me/caelestia-kde/shell/assets/bongocat.gif";
        let has_bongo = std::path::Path::new(bongocat_path).exists();

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
                // Top Section: Cover Art + Title + Controls
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap(px(12.0))
                    .w_full()
                    .child(
                        // Cover Art Circular Placeholder
                        div()
                            .size(px(104.0))
                            .rounded_full()
                            .bg(rgba(c.surface_container_high))
                            .border_2()
                            .border_color(rgba(c.outline_variant))
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(
                                MaterialIcon::new("photo_size_select_actual")
                                    .color(c.outline)
                                    .text_size(px(36.0)),
                            ),
                    )
                    .child(
                        // Track Title
                        StyledText::new(title)
                            .color(c.primary)
                            .font_size(14.0)
                            .font_weight(gpui::FontWeight::BOLD),
                    )
                    .child(
                        // Player Controls Row
                        div()
                            .flex()
                            .items_center()
                            .justify_center()
                            .gap(px(10.0))
                            .child(
                                div()
                                    .size(px(32.0))
                                    .rounded_full()
                                    .bg(rgba(c.surface_container_high))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .cursor(CursorStyle::PointingHand)
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(move |view, _, _, cx| on_prev(view, cx)),
                                    )
                                    .child(
                                        MaterialIcon::new("skip_previous")
                                            .color(c.on_surface_variant)
                                            .text_size(px(18.0)),
                                    ),
                            )
                            .child(
                                div()
                                    .size(px(40.0))
                                    .rounded_full()
                                    .bg(rgba(c.surface_container_highest))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .cursor(CursorStyle::PointingHand)
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(move |view, _, _, cx| on_play_pause(view, cx)),
                                    )
                                    .child(
                                        MaterialIcon::new(if is_playing {
                                            "pause"
                                        } else {
                                            "play_arrow"
                                        })
                                        .color(c.primary)
                                        .text_size(px(22.0)),
                                    ),
                            )
                            .child(
                                div()
                                    .size(px(32.0))
                                    .rounded_full()
                                    .bg(rgba(c.surface_container_high))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .cursor(CursorStyle::PointingHand)
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(move |view, _, _, cx| on_next(view, cx)),
                                    )
                                    .child(
                                        MaterialIcon::new("skip_next")
                                            .color(c.on_surface_variant)
                                            .text_size(px(18.0)),
                                    ),
                            ),
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
