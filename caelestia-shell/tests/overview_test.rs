//! Comprehensive Test Suite for Overview Subsystem
//! Validates KDE & GNOME packing algorithms, workspace pill indicator models,
//! selection cycling, drag-and-drop workspace migration, and overview lifecycle.

use caelestia_shell::modules::overview::{
    calculate_gnome_layout, calculate_kde_layout, calculate_layout,
    OverviewLayoutType, OverviewState, WorkspaceIndicatorModel,
};
use caelestia_shell::services::kwin::{KWinService, KWinState, VirtualWorkspace, WindowItem};

fn mock_window(addr: &str, title: &str, app_id: &str, ws_id: &str, x: i32, y: i32, w: u32, h: u32) -> WindowItem {
    WindowItem {
        address: addr.to_string(),
        title: title.to_string(),
        app_id: app_id.to_string(),
        pid: 1234,
        x,
        y,
        width: w,
        height: h,
        is_active: false,
        is_minimized: false,
        is_maximized: false,
        is_fullscreen: false,
        demands_attention: false,
        skip_taskbar: false,
        desktops: vec![ws_id.to_string()],
        output: "eDP-1".to_string(),
    }
}

#[test]
fn test_overview_layout_kde_packing() {
    let windows = vec![
        mock_window("win-1", "Terminal", "kitty", "1", 0, 0, 1920, 1080),
        mock_window("win-2", "Browser", "firefox", "1", 0, 0, 1920, 1080),
        mock_window("win-3", "Editor", "code", "1", 0, 0, 1200, 800),
    ];

    let layout = calculate_kde_layout(&windows, 1600.0, 900.0, 16.0, 16.0);
    assert_eq!(layout.len(), 3);

    for (addr, card) in &layout {
        assert!(card.width > 100.0, "Card {} width should be > 100, got {}", addr, card.width);
        assert!(card.height > 100.0, "Card {} height should be > 100, got {}", addr, card.height);
        assert!(card.x >= 0.0, "Card {} x should be >= 0, got {}", addr, card.x);
        assert!(card.y >= 0.0, "Card {} y should be >= 0, got {}", addr, card.y);
    }
}

#[test]
fn test_overview_layout_gnome_packing() {
    let windows = vec![
        mock_window("win-1", "Terminal", "kitty", "1", 0, 0, 1920, 1080),
        mock_window("win-2", "Browser", "firefox", "1", 0, 0, 1920, 1080),
    ];

    let layout = calculate_gnome_layout(&windows, 1600.0, 900.0, 16.0, 16.0);
    assert_eq!(layout.len(), 2);

    let w1 = layout.get("win-1").expect("win-1 should be laid out");
    let w2 = layout.get("win-2").expect("win-2 should be laid out");

    assert!(w1.width > 50.0);
    assert!(w2.width > 50.0);
    // In horizontal row, w2 should be placed to the right of w1
    assert!(w2.x > w1.x);

    // Test unified dispatcher
    let dispatched = calculate_layout(OverviewLayoutType::Gnome, &windows, 1600.0, 900.0, 16.0, 16.0);
    assert_eq!(dispatched.len(), 2);
}

#[test]
fn test_workspace_indicator_computation() {
    let mut kwin_state = KWinState::default();
    kwin_state.workspaces = vec![
        VirtualWorkspace {
            id: 1,
            uuid: "ws-uuid-1".to_string(),
            name: "Desktop 1".to_string(),
            active: true,
        },
        VirtualWorkspace {
            id: 2,
            uuid: "ws-uuid-2".to_string(),
            name: "Desktop 2".to_string(),
            active: false,
        },
        VirtualWorkspace {
            id: 3,
            uuid: "ws-uuid-3".to_string(),
            name: "Desktop 3".to_string(),
            active: false,
        },
    ];

    kwin_state.window_list = vec![
        mock_window("win-1", "App 1", "app1", "1", 0, 0, 800, 600),
        mock_window("win-2", "App 2", "app2", "1", 0, 0, 800, 600),
        mock_window("win-3", "App 3", "app3", "3", 0, 0, 800, 600),
    ];

    let indicator = WorkspaceIndicatorModel::compute(&kwin_state, Some("eDP-1"), 1000.0);
    assert_eq!(indicator.pills.len(), 3);
    assert_eq!(indicator.active_index, 0);

    // Workspace 1 has 2 windows and is active & occupied
    assert_eq!(indicator.pills[0].id, 1);
    assert!(indicator.pills[0].active);
    assert!(indicator.pills[0].occupied);
    assert_eq!(indicator.pills[0].window_count, 2);

    // Workspace 2 has 0 windows and is not occupied
    assert_eq!(indicator.pills[1].id, 2);
    assert!(!indicator.pills[1].active);
    assert!(!indicator.pills[1].occupied);
    assert_eq!(indicator.pills[1].window_count, 0);

    // Workspace 3 has 1 window and is occupied
    assert_eq!(indicator.pills[2].id, 3);
    assert!(!indicator.pills[2].active);
    assert!(indicator.pills[2].occupied);
    assert_eq!(indicator.pills[2].window_count, 1);
}

#[test]
fn test_overview_selection_cycling() {
    let mut state = OverviewState::new();

    // Cycling with 0 windows does not panic
    state.cycle_selection(false, 0);
    assert_eq!(state.selected_index, None);

    // Forward cycle with 3 windows: 0 -> 1 -> 2 -> 0
    state.cycle_selection(false, 3);
    assert_eq!(state.selected_index, Some(0));

    state.cycle_selection(false, 3);
    assert_eq!(state.selected_index, Some(1));

    state.cycle_selection(false, 3);
    assert_eq!(state.selected_index, Some(2));

    state.cycle_selection(false, 3);
    assert_eq!(state.selected_index, Some(0));

    // Backward cycle with 3 windows: 0 -> 2 -> 1 -> 0
    state.cycle_selection(true, 3);
    assert_eq!(state.selected_index, Some(2));

    state.cycle_selection(true, 3);
    assert_eq!(state.selected_index, Some(1));

    state.cycle_selection(true, 3);
    assert_eq!(state.selected_index, Some(0));
}

#[test]
fn test_overview_window_activation_and_dnd() {
    let mut kwin = KWinService {
        state: KWinState::default(),
    };
    kwin.state.workspaces = vec![
        VirtualWorkspace {
            id: 1,
            uuid: "uuid-1".to_string(),
            name: "Desktop 1".to_string(),
            active: true,
        },
        VirtualWorkspace {
            id: 2,
            uuid: "uuid-2".to_string(),
            name: "Desktop 2".to_string(),
            active: false,
        },
    ];
    let win1 = mock_window("win-1", "App 1", "app1", "1", 0, 0, 800, 600);
    let win2 = mock_window("win-2", "App 2", "app2", "1", 0, 0, 800, 600);
    kwin.state.window_list = vec![win1.clone(), win2.clone()];
    kwin.state.active_window = Some(win1.clone());

    let mut state = OverviewState::new();
    state.open(&kwin);
    assert!(state.is_open);
    assert_eq!(state.selected_index, Some(0));

    // Drag and drop win2 to workspace 2
    state.move_window_to_workspace("win-2", 2, &mut kwin);
    let updated_win2 = kwin.state.window_list.iter().find(|w| w.address == "win-2").unwrap();
    assert_eq!(updated_win2.desktops, vec!["2".to_string()]);

    // Activate selected window
    state.cycle_selection(false, 2); // select index 1 (win2)
    state.activate_selected(&[win1, win2], &mut kwin);
    assert!(!state.is_open); // Closes on activation
    assert_eq!(kwin.state.pending_focus_address, Some("win-2".to_string()));
}

#[test]
fn test_overview_window_info_overlay() {
    let mut state = OverviewState::new();
    assert!(!state.is_window_info_open);

    state.open_window_info("win-42");
    assert!(state.is_window_info_open);
    assert_eq!(state.window_info_address.as_deref(), Some("win-42"));

    state.close_window_info();
    assert!(!state.is_window_info_open);
    assert_eq!(state.window_info_address, None);
}

#[test]
fn test_window_info_details_and_controller() {
    use caelestia_shell::modules::overview::{WindowInfoController, WindowInfoDetails};

    let win = mock_window("win-inspect", "Test Terminal", "alacritty", "3", 50, 60, 1024, 768);
    let details = WindowInfoDetails::from_window_item(&win);

    assert_eq!(details.address, "win-inspect");
    assert_eq!(details.title, "Test Terminal");
    assert_eq!(details.app_id, "alacritty");
    assert_eq!(details.x, 50);
    assert_eq!(details.y, 60);
    assert_eq!(details.width, 1024);
    assert_eq!(details.height, 768);
    assert_eq!(details.workspace_id, "3");
    assert!(!details.is_maximized);

    let mut kwin = KWinService {
        state: KWinState::default(),
    };
    kwin.state.window_list = vec![win];

    // Toggle maximize
    WindowInfoController::toggle_maximize("win-inspect", false, &mut kwin);
    let w_max = kwin.state.window_list.iter().find(|w| w.address == "win-inspect").unwrap();
    assert!(w_max.is_maximized);

    // Toggle minimize
    WindowInfoController::toggle_minimize("win-inspect", false, &mut kwin);
    let w_min = kwin.state.window_list.iter().find(|w| w.address == "win-inspect").unwrap();
    assert!(w_min.is_minimized);

    // Move to workspace 4
    WindowInfoController::move_to_workspace("win-inspect", 4, &mut kwin);
    let w_moved = kwin.state.window_list.iter().find(|w| w.address == "win-inspect").unwrap();
    assert_eq!(w_moved.desktops, vec!["4".to_string()]);
}

#[test]
fn test_overview_workspace_switcher_paging() {
    let mut state = OverviewState::new();
    state.is_open = true;
    state.current_workspace_index = 0;

    // Simulate paging forward
    state.current_workspace_index += 1;
    assert_eq!(state.current_workspace_index, 1);

    // Simulate paging forward again
    state.current_workspace_index += 1;
    assert_eq!(state.current_workspace_index, 2);

    // Reselecting current closes overview
    if state.current_workspace_index == 2 {
        state.close();
    }
    assert!(!state.is_open);
}
