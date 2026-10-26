use caelestia_shell::modules::background::desktopicons::*;
use caelestia_shell::modules::background::widgets::*;
use std::collections::HashMap;

#[test]
fn test_layout_engine_grid_occupancy_and_first_free() {
    let mut positions = HashMap::new();
    positions.insert("item1".to_string(), GridPos::new(0, 0));
    positions.insert("item2".to_string(), GridPos::new(0, 1));
    positions.insert("item3".to_string(), GridPos::new(0, 2));

    let mut spans = HashMap::new();
    spans.insert("item1".to_string(), Span { w: 1, h: 1 });

    let occ = LayoutEngine::occupancy(&positions, None, Some(&spans));
    assert_eq!(occ.len(), 3);
    assert_eq!(occ.get("0,0").map(|s| s.as_str()), Some("item1"));
    assert_eq!(occ.get("0,1").map(|s| s.as_str()), Some("item2"));
    assert_eq!(occ.get("0,2").map(|s| s.as_str()), Some("item3"));

    // In a 4-row desktop, next free in col 0 is row 3
    let next_free = LayoutEngine::first_free(&occ, 10, 4, Span { w: 1, h: 1 });
    assert_eq!(next_free, GridPos::new(0, 3));

    // If col 0 is full, next free wraps to col 1, row 0
    positions.insert("item4".to_string(), GridPos::new(0, 3));
    let occ2 = LayoutEngine::occupancy(&positions, None, Some(&spans));
    let next_col_free = LayoutEngine::first_free(&occ2, 10, 4, Span { w: 1, h: 1 });
    assert_eq!(next_col_free, GridPos::new(1, 0));
}

#[test]
fn test_layout_engine_nearest_free_and_spans() {
    let mut positions = HashMap::new();
    positions.insert("w1".to_string(), GridPos::new(1, 1));
    let mut spans = HashMap::new();
    spans.insert("w1".to_string(), Span { w: 2, h: 2 }); // Occupies (1,1), (1,2), (2,1), (2,2)

    let occ = LayoutEngine::occupancy(&positions, None, Some(&spans));
    assert_eq!(occ.len(), 4);

    // Nearest free to (1, 1) for 1x1
    let near = LayoutEngine::nearest_free(&occ, 1, 1, 5, 5, Span { w: 1, h: 1 });
    // Must not be in the occupied 2x2 box
    assert!(!LayoutEngine::rects_overlap(
        &near,
        Span { w: 1, h: 1 },
        &GridPos::new(1, 1),
        Span { w: 2, h: 2 }
    ));
}

#[test]
fn test_layout_engine_plan_move_and_displacement() {
    let mut positions = HashMap::new();
    positions.insert("a".to_string(), GridPos::new(0, 0));
    positions.insert("b".to_string(), GridPos::new(1, 0));

    let moving = vec!["a".to_string()];
    let target = GridPos::new(1, 0); // "a" lands on "b"'s spot

    let new_pos = LayoutEngine::plan_move(&positions, &moving, "a", &target, 10, 10, None);
    assert_eq!(new_pos.get("a"), Some(&GridPos::new(1, 0)));
    // "b" is displaced to a nearest free cell
    assert_ne!(new_pos.get("b"), Some(&GridPos::new(1, 0)));
}

#[test]
fn test_layout_engine_compact_and_insert() {
    let keys = vec!["x".to_string(), "y".to_string(), "z".to_string()];
    let compact_res = LayoutEngine::compact(&keys, 2, None);
    // Rows = 2:
    // x -> (0, 0)
    // y -> (0, 1)
    // z -> (1, 0)
    assert_eq!(compact_res.get("x"), Some(&GridPos::new(0, 0)));
    assert_eq!(compact_res.get("y"), Some(&GridPos::new(0, 1)));
    assert_eq!(compact_res.get("z"), Some(&GridPos::new(1, 0)));

    let moving = vec!["z".to_string()];
    let target = GridPos::new(0, 0);
    let inserted = LayoutEngine::plan_insert(&compact_res, &moving, &target, 2, None);
    assert_eq!(inserted.get("z"), Some(&GridPos::new(0, 0)));
}

#[test]
fn test_layout_engine_sort_and_navigation() {
    let items = vec![
        DesktopItemMeta {
            key: "f1".to_string(),
            name: "Zeta".to_string(),
            kind: "file".to_string(),
            item_type: None,
            modified: Some(10),
            size: Some(500),
        },
        DesktopItemMeta {
            key: "w1".to_string(),
            name: "Alpha Widget".to_string(),
            kind: "widget".to_string(),
            item_type: None,
            modified: None,
            size: None,
        },
        DesktopItemMeta {
            key: "d1".to_string(),
            name: "Beta Dir".to_string(),
            kind: "dir".to_string(),
            item_type: None,
            modified: Some(20),
            size: None,
        },
    ];

    let sorted = LayoutEngine::sort_items(items, "name");
    // Rank 0: widget, Rank 1: dir, Rank 2: file
    assert_eq!(sorted[0].key, "w1");
    assert_eq!(sorted[1].key, "d1");
    assert_eq!(sorted[2].key, "f1");

    // Spatial navigation test
    let mut positions = HashMap::new();
    positions.insert("center".to_string(), GridPos::new(2, 2));
    positions.insert("right".to_string(), GridPos::new(3, 2));
    positions.insert("left".to_string(), GridPos::new(1, 2));
    positions.insert("up".to_string(), GridPos::new(2, 1));
    positions.insert("down".to_string(), GridPos::new(2, 3));

    assert_eq!(
        LayoutEngine::navigate(&positions, "center", NavDirection::Right, None),
        Some("right".to_string())
    );
    assert_eq!(
        LayoutEngine::navigate(&positions, "center", NavDirection::Left, None),
        Some("left".to_string())
    );
    assert_eq!(
        LayoutEngine::navigate(&positions, "center", NavDirection::Up, None),
        Some("up".to_string())
    );
    assert_eq!(
        LayoutEngine::navigate(&positions, "center", NavDirection::Down, None),
        Some("down".to_string())
    );
}

#[test]
fn test_layout_engine_rubberband_rect_and_ranges() {
    let mut positions = HashMap::new();
    positions.insert("a".to_string(), GridPos::new(0, 0));
    positions.insert("b".to_string(), GridPos::new(1, 0));
    positions.insert("c".to_string(), GridPos::new(2, 0));

    let rect = PixelRect {
        x: 50.0,
        y: 0.0,
        width: 120.0,
        height: 100.0,
    };

    // Cell size: 100x100
    // "a" is at 0..100 -> overlaps 50..100
    // "b" is at 100..200 -> overlaps 100..170
    // "c" is at 200..300 -> outside
    let hit_keys = LayoutEngine::keys_in_rect(&positions, &rect, 100.0, 100.0, 0.0, None);
    assert!(hit_keys.contains(&"a".to_string()));
    assert!(hit_keys.contains(&"b".to_string()));
    assert!(!hit_keys.contains(&"c".to_string()));

    let range = LayoutEngine::range_between(&positions, "a", "c", 10);
    assert_eq!(range.len(), 3);
}

#[test]
fn test_desktop_entry_icon_command_parsing() {
    let icon = DesktopEntryIcon::new("kwrite", "KWrite", "accessories-text-editor", "kwrite %U");
    assert_eq!(icon.build_launch_command(), "kwrite");

    let terminal_icon = DesktopEntryIcon {
        key: "htop".to_string(),
        name: "Htop".to_string(),
        icon_name: "htop".to_string(),
        exec_cmd: "htop".to_string(),
        desktop_file_path: None,
        comment: None,
        terminal: true,
        is_pinned: false,
    };
    assert_eq!(terminal_icon.build_launch_command(), "konsole -e htop");
}

#[test]
fn test_big_folder_paging_and_children() {
    let mut folder = BigFolderItem::new("fld", "Documents", std::path::PathBuf::from("/tmp"));
    for i in 1..=20 {
        folder.add_child(format!("item_{i}"));
    }
    assert_eq!(folder.total_pages(), 3); // 9 items per page: 9 + 9 + 2 = 20
    assert_eq!(folder.current_page_items().len(), 9);

    folder.page_index = 2;
    assert_eq!(folder.current_page_items().len(), 2);

    assert!(folder.remove_child("item_1"));
    assert_eq!(folder.children_keys.len(), 19);
}

#[test]
fn test_widget_catalog_and_models() {
    let widgets = DesktopWidgetCatalog::all_widgets();
    assert!(widgets.len() >= 7);

    let clock_def = DesktopWidgetCatalog::find_widget("clock").unwrap();
    assert_eq!(clock_def.default_span, Span { w: 2, h: 2 });

    let mut clock_model = DesktopClockWidgetModel::default();
    clock_model.update_time(15, 30, 45, "Mon, Oct 26");
    assert_eq!(clock_model.time_str, "15:30");
    let (h_deg, _m_deg, s_deg) = clock_model.analog_angles();
    assert_eq!(h_deg, 3.0 * 30.0 + 30.0 * 0.5); // 105 deg
    assert_eq!(s_deg, 45.0 * 6.0); // 270 deg

    let mut lyrics = DesktopLyricsWidgetModel::default();
    lyrics.set_lyrics(vec![
        DesktopLyricLine {
            timestamp_ms: 1000,
            text: "Hello".to_string(),
            translation: None,
        },
        DesktopLyricLine {
            timestamp_ms: 3000,
            text: "World".to_string(),
            translation: None,
        },
    ]);
    lyrics.update_position(2500);
    assert_eq!(lyrics.current_line().map(|l| l.text.as_str()), Some("Hello"));
    lyrics.update_position(3500);
    assert_eq!(lyrics.current_line().map(|l| l.text.as_str()), Some("World"));

    let mut media = DesktopMediaWidgetModel::default();
    media.duration_ms = 200_000;
    media.position_ms = 50_000;
    assert_eq!(media.progress_ratio(), 0.25);
    assert_eq!(media.formatted_position(), "0:50");
    assert_eq!(media.formatted_duration(), "3:20");

    let mut sys = DesktopSystemWidgetModel::default();
    sys.update_metrics(45.2, 8.0, 16.0, Some(85.0), true, 60.0, Some(52.0));
    assert_eq!(sys.cpu_usage_percent, 45.2);
    assert_eq!(sys.ram_usage_percent, 50.0);
    assert_eq!(sys.battery_percent, Some(85.0));
    assert!(sys.is_charging);
}
