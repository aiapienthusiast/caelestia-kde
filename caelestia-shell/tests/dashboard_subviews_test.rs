use caelestia_shell::modules::dashboard::subviews::*;
use caelestia_shell::modules::dashboard::tabs::DashboardTab;

#[test]
fn test_dashboard_tabs_enum_and_labels() {
    assert_eq!(DashboardTab::ALL.len(), 7);
    assert_eq!(DashboardTab::Notes.label(), "Notes");
    assert_eq!(DashboardTab::Terminal.label(), "Terminal");
    assert_eq!(DashboardTab::Wallhaven.label(), "Wallhaven");
    assert_eq!(DashboardTab::Notes.icon(), "edit_note");
}

#[test]
fn test_notes_tab_lifecycle_and_search() {
    let mut model = NotesTabModel::default();
    assert_eq!(model.notes.len(), 1);

    model.create_note("Rust Porting Tasks");
    assert_eq!(model.notes.len(), 2);
    assert_eq!(model.active_note().map(|n| n.title.as_str()), Some("Rust Porting Tasks"));

    if let Some(note) = model.active_note_mut() {
        note.body = "Finish dashboard subviews and write unit tests.".to_string();
    }

    model.search_filter = "subviews".to_string();
    let hits = model.filtered_notes();
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].title, "Rust Porting Tasks");

    assert!(model.delete_active_note());
    assert_eq!(model.notes.len(), 1);
}

#[test]
fn test_terminal_tab_bridge() {
    let mut model = TerminalTabModel::default();
    model.current_command = "cargo test".to_string();
    let executed = model.submit_command();
    assert_eq!(executed, Some("cargo test".to_string()));
    assert_eq!(model.history, vec!["cargo test".to_string()]);

    model.push_output("running 5 tests");
    assert!(model.output_lines.iter().any(|l| l.contains("running 5 tests")));
}

#[test]
fn test_wallhaven_tab_url_builder() {
    let mut model = WallhavenTabModel::default();
    model.search_query = "nature".to_string();
    model.current_page = 2;
    let url = model.build_api_url();
    assert!(url.contains("q=nature"));
    assert!(url.contains("page=2"));
    assert!(url.contains("purity=100"));
}

#[test]
fn test_battery_tank_and_hero_card() {
    let mut tank = BatteryTankModel::default();
    tank.is_charging = false;
    tank.time_to_empty_secs = Some(7200); // 2 hours
    assert_eq!(tank.formatted_time_remaining(), "2h 0m remaining");

    tank.is_charging = true;
    tank.time_to_full_secs = Some(3600); // 1 hour
    assert_eq!(tank.formatted_time_remaining(), "1h 0m until full");

    let mut hero = HeroPerformanceCardModel::default();
    hero.push_sample(88.0);
    assert_eq!(hero.sparkline_samples.last(), Some(&88.0));

    let mut lyrics_sel = LyricsSelectorModel::default();
    lyrics_sel.adjust_offset(500);
    assert_eq!(lyrics_sel.offset_ms, 500);
    lyrics_sel.adjust_offset(-200);
    assert_eq!(lyrics_sel.offset_ms, 300);
}
