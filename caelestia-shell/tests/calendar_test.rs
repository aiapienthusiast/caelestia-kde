use std::collections::HashSet;
use std::time::Duration;
use chrono::NaiveDate;
use gpui::Rgba;
use caelestia_shell::components::calendar::*;
use caelestia_shell::components::drawer_visibilities::DrawerVisibilities;
use caelestia_shell::components::logo::{LogoConfig, LOGO_PATH_TOP_CRESCENT};
use caelestia_shell::components::screen_state::ScreenState;

#[test]
fn test_calendar_month_slide() {
    let date_a = NaiveDate::from_ymd_opt(2026, 10, 1).unwrap();
    let date_b = NaiveDate::from_ymd_opt(2026, 11, 1).unwrap();

    let mut slide = CalendarMonthSlide::new(date_a);
    assert_eq!(slide.anim_translate, 0.0);
    assert_eq!(slide.anim_opacity, 1.0);
    assert!(!slide.is_animating);

    // Slide forward to November
    slide.set_shown_date(date_b);
    assert!(slide.is_animating);
    assert!(slide.anim_translate > 0.0); // positive parked offset
    assert_eq!(slide.anim_opacity, 0.0);

    // Advance animation to completion
    slide.tick(Duration::from_millis(600));
    assert!(!slide.is_animating);
    assert_eq!(slide.anim_translate, 0.0);
    assert_eq!(slide.anim_opacity, 1.0);
}

#[test]
fn test_calendar_month_navigation_and_grid() {
    let oct_2026 = NaiveDate::from_ymd_opt(2026, 10, 26).unwrap();
    let mut state = CalendarMonthState::new(oct_2026);

    assert_eq!(state.curr_month(), 10);
    assert_eq!(state.curr_year(), 2026);
    assert_eq!(state.month_year_title(), "October 2026");

    // Add event on Oct 28
    let oct_28 = NaiveDate::from_ymd_opt(2026, 10, 28).unwrap();
    let mut events = HashSet::new();
    events.insert(oct_28);
    state = state.with_events(events);

    // Set highlighted date to Oct 26
    state = state.with_highlighted_date(Some(oct_2026));

    let cells = state.generate_grid_cells();
    assert_eq!(cells.len(), 42);

    // Check cells: October 2026 starts on Thursday Oct 1st.
    // The grid starts from Monday Sep 28th.
    assert_eq!(cells[0].date, NaiveDate::from_ymd_opt(2026, 9, 28).unwrap());
    assert!(!cells[0].is_in_month);

    // Find Oct 26 in grid
    let cell_26 = cells.iter().find(|c| c.date == oct_2026).expect("Oct 26 cell exists");
    assert!(cell_26.is_in_month);
    assert!(cell_26.is_marked);

    // Find Oct 28 with events
    let cell_28 = cells.iter().find(|c| c.date == oct_28).expect("Oct 28 cell exists");
    assert!(cell_28.has_events);

    // Shift month forward
    state.shift_month(1);
    assert_eq!(state.curr_month(), 11);
    assert_eq!(state.curr_year(), 2026);

    // Shift month backward by 2
    state.shift_month(-2);
    assert_eq!(state.curr_month(), 9);
    assert_eq!(state.curr_year(), 2026);
}

#[test]
fn test_drawer_visibilities() {
    let mut vis = DrawerVisibilities::new();
    assert!(!vis.is_any_drawer_open());

    vis.toggle_launcher();
    assert!(vis.launcher);
    assert!(vis.is_any_drawer_open());

    // Toggling dashboard should close launcher and open dashboard
    vis.toggle_dashboard();
    assert!(!vis.launcher);
    assert!(vis.dashboard);

    vis.close_all();
    assert!(!vis.is_any_drawer_open());
}

#[test]
fn test_screen_state() {
    let mut state = ScreenState::new("DP-1");
    assert_eq!(state.reloadable_id(), "screenState-DP-1");
    assert_eq!(state.screen_name, "DP-1");
    assert_eq!(state.dashboard_tab, 0);

    state.launcher = true;
    state.sidebar = true;
    state.close_all();
    assert!(!state.launcher);
    assert!(!state.sidebar);
}

#[test]
fn test_logo_svg_generation() {
    let top_color = Rgba { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
    let bottom_color = Rgba { r: 0.0, g: 1.0, b: 0.0, a: 1.0 };

    let config = LogoConfig::new().with_colors(top_color, bottom_color);
    let svg = config.to_svg_string(top_color, bottom_color);

    assert!(svg.contains("<svg"));
    assert!(svg.contains(LOGO_PATH_TOP_CRESCENT));
    assert!(svg.contains("#ff0000"));
    assert!(svg.contains("#00ff00"));
}
