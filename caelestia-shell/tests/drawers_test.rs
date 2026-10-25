use caelestia_shell::modules::drawers::*;

#[test]
fn test_exclusions_struts_and_usable_area() {
    // 1. Standard Top Bar configuration
    let config_top = ExclusionConfig {
        screen_width: 1920.0,
        screen_height: 1080.0,
        bar_position: BarPosition::Top,
        bar_exclusive_zone: 48.0,
        border_thickness: 8.0,
        sidebar_enabled: true,
        sidebar_pinned: false,
        sidebar_visible: false,
        sidebar_suspended: false,
        sidebar_width: 380.0,
    };
    let ex_top = ExclusionZones::compute(&config_top);
    assert_eq!(ex_top.top, 48.0);
    assert_eq!(ex_top.bottom, 8.0);
    assert_eq!(ex_top.left, 8.0);
    assert_eq!(ex_top.right, 8.0);
    assert_eq!(ex_top.usable_area.x, 8.0);
    assert_eq!(ex_top.usable_area.y, 48.0);
    assert_eq!(ex_top.usable_area.width, 1920.0 - 16.0);
    assert_eq!(ex_top.usable_area.height, 1080.0 - 48.0 - 8.0);

    // 2. Right Bar with Pinned Sidebar
    let config_right_pinned = ExclusionConfig {
        screen_width: 2560.0,
        screen_height: 1440.0,
        bar_position: BarPosition::Right,
        bar_exclusive_zone: 54.0,
        border_thickness: 10.0,
        sidebar_enabled: true,
        sidebar_pinned: true,
        sidebar_visible: true,
        sidebar_suspended: false,
        sidebar_width: 400.0,
    };
    let ex_right = ExclusionZones::compute(&config_right_pinned);
    // When bar is on right and sidebar is pinned, sidebar reserves on the left
    assert_eq!(ex_right.right, 54.0);
    assert_eq!(ex_right.left, 10.0 + 400.0);
    assert_eq!(ex_right.top, 10.0);
    assert_eq!(ex_right.bottom, 10.0);

    // 3. Minimum clamping test
    let config_tiny = ExclusionConfig {
        screen_width: 120.0,
        screen_height: 120.0,
        bar_position: BarPosition::Top,
        bar_exclusive_zone: 80.0,
        border_thickness: 40.0,
        sidebar_enabled: false,
        sidebar_pinned: false,
        sidebar_visible: false,
        sidebar_suspended: false,
        sidebar_width: 0.0,
    };
    let ex_tiny = ExclusionZones::compute(&config_tiny);
    assert!(ex_tiny.usable_area.width >= 100.0);
    assert!(ex_tiny.usable_area.height >= 100.0);
}

#[test]
fn test_panels_layout_and_margins() {
    let config = PanelsConfig {
        bar_position: BarPosition::Top,
        bar_width: 1920.0,
        bar_height: 48.0,
        border_thickness: 8.0,
        islands: true,
        island_spacing: 16.0,
        notif_position: NotifPosition::Auto,
        notifs_width: 360.0,
        notifs_height: 120.0,
        sidebar_width: 380.0,
        sidebar_height: 800.0,
    };

    let layout = PanelsLayout::compute(&config);
    // Top bar with islands: topMargin = barHeight + islandSpacing * 2
    assert_eq!(layout.margins.top, 48.0 + 32.0);
    assert_eq!(layout.margins.bottom, 8.0 + 16.0);
    assert_eq!(layout.margins.left, 8.0 + 16.0);
    assert_eq!(layout.margins.right, 8.0 + 16.0);

    // Auto notification position with Top Bar resolves to TopRight
    assert_eq!(layout.resolved_notif_position, NotifPosition::TopRight);
    assert!(layout.notif_at_top);
    assert!(!layout.notif_at_bottom);
    assert_eq!(layout.notif_reserved_height, 120.0 + 16.0);

    // Auto notification position with Bottom Bar resolves to BottomRight
    let config_bottom = PanelsConfig {
        bar_position: BarPosition::Bottom,
        ..config
    };
    let layout_bottom = PanelsLayout::compute(&config_bottom);
    assert_eq!(layout_bottom.resolved_notif_position, NotifPosition::BottomRight);
    assert!(layout_bottom.notif_at_bottom);
}

#[test]
fn test_panels_popout_intersections() {
    let popout = RectBounds {
        x: 1500.0,
        y: 60.0,
        width: 300.0,
        height: 400.0,
    };
    let notifs = RectBounds {
        x: 1600.0,
        y: 60.0,
        width: 300.0,
        height: 200.0,
    };
    let sidebar = RectBounds {
        x: 100.0,
        y: 60.0,
        width: 380.0,
        height: 900.0,
    };

    // Horizontal bar popout intersects notifications
    let intersects_notifs = PanelsLayout::popout_intersects_right(
        true,
        &popout,
        true,
        0.0,
        &notifs,
    );
    assert!(intersects_notifs);

    // Popout does not intersect sidebar when on opposite side
    let intersects_side = PanelsLayout::popout_intersects_sidebar(
        true,
        &popout,
        true,
        0.0,
        &sidebar,
        true,
    );
    assert!(!intersects_side);

    // If popout is closed (offset_scale >= 1.0), it should report false
    let closed_intersects = PanelsLayout::popout_intersects_right(
        true,
        &popout,
        true,
        1.0,
        &notifs,
    );
    assert!(!closed_intersects);
}

#[test]
fn test_input_regions_and_click_passthrough() {
    let mut regions = InputRegions::new(1920.0, 1080.0, BarPosition::Top, 48.0);
    regions.hot_corners = OverviewHotCorners {
        enabled: true,
        hover_thickness: 20.0,
        top_left: true,
        top_right: false,
        bottom_left: false,
        bottom_right: false,
    };

    // 1. Point on the top bar should capture input
    assert!(regions.should_capture_input(500.0, 20.0));
    assert!(regions.in_bar_area(500.0, 20.0));

    // 2. Point in Top-Left hot corner should capture input
    assert!(regions.should_capture_input(10.0, 10.0));
    assert!(regions.in_hot_corner(10.0, 10.0));

    // 3. Point in empty desktop should PASS THROUGH (return false)
    assert!(!regions.should_capture_input(800.0, 600.0));

    // 4. Open dashboard drawer should capture input within its extent
    regions.offsets.dashboard_height = 500.0;
    regions.offsets.dashboard_offset = 0.0; // 0.0 = fully open
    assert!(regions.should_capture_input(800.0, 400.0));
    assert!(regions.in_dashboard_region(800.0, 400.0));

    // 5. Point beyond open dashboard should pass through
    assert!(!regions.in_dashboard_region(800.0, 700.0));
}

#[test]
fn test_interactions_span_bounds_and_triggers() {
    let (near, far) = InteractionsTracker::span_bounds(100.0, 500.0, 100.0);
    assert_eq!(near, 100.0);
    assert_eq!(far, 500.0);

    let (near_50, far_50) = InteractionsTracker::span_bounds(100.0, 500.0, 50.0);
    // Span of 50% centers around 300 with width 200 (200 to 400)
    assert_eq!(near_50, 200.0);
    assert_eq!(far_50, 400.0);

    let config = InteractionsConfig {
        screen_width: 1920.0,
        screen_height: 1080.0,
        bar_position: BarPosition::Top,
        bar_thickness: 48.0,
        border_thickness: 8.0,
        hover_thickness: 16.0,
        swipe_threshold: 40.0,
    };
    let tracker = InteractionsTracker::new(config);

    // Top edge trigger test
    assert!(tracker.is_top_edge_trigger(500.0, 30.0));
    assert!(!tracker.is_top_edge_trigger(500.0, 100.0));

    // Bottom edge trigger test
    assert!(tracker.is_bottom_edge_trigger(500.0, 1070.0));
    assert!(!tracker.is_bottom_edge_trigger(500.0, 900.0));

    // Sidebar trigger test (right edge)
    assert!(tracker.is_sidebar_edge_trigger(1910.0, 500.0));
    assert!(!tracker.is_sidebar_edge_trigger(1800.0, 500.0));
}

#[test]
fn test_interactions_drag_gesture_threshold() {
    let config = InteractionsConfig {
        screen_width: 1920.0,
        screen_height: 1080.0,
        bar_position: BarPosition::Top,
        bar_thickness: 48.0,
        border_thickness: 8.0,
        hover_thickness: 16.0,
        swipe_threshold: 40.0,
    };
    let mut tracker = InteractionsTracker::new(config);

    // Start drag at top edge
    let target = tracker.start_drag(600.0, 10.0);
    assert_eq!(target, Some(DrawerTarget::Dashboard));

    // Drag downward past threshold (e.g. +60px)
    tracker.update_drag(600.0, 70.0);
    let result = tracker.end_drag();
    assert_eq!(result, Some((DrawerTarget::Dashboard, true)));

    // Drag downward insufficient distance (e.g. +20px < 40px threshold)
    tracker.start_drag(600.0, 10.0);
    tracker.update_drag(600.0, 30.0);
    let result_small = tracker.end_drag();
    assert_eq!(result_small, Some((DrawerTarget::Dashboard, false)));
}

#[test]
fn test_content_window_coordinator_lifecycle() {
    let mut coord = ContentWindowCoordinator::new("eDP-1", 1920.0, 1080.0, BarPosition::Top);

    // Initially no overlays are open
    assert!(!coord.visibilities.has_open_overlay());

    // Toggle dashboard
    coord.visibilities.toggle(DrawerTarget::Dashboard);
    assert!(coord.visibilities.dashboard);
    assert!(coord.visibilities.has_open_overlay());
    assert!(coord.visibilities.captures_overlay_input());

    // Clicking in empty desktop outside active regions dismisses the open overlay
    let consumed = coord.handle_pointer_down(900.0, 800.0);
    assert!(consumed);
    assert!(!coord.visibilities.has_open_overlay());
    assert!(!coord.visibilities.dashboard);

    // Edge drag opens drawer via pointer up
    coord.handle_pointer_down(500.0, 20.0); // start drag at top edge
    coord.tracker.update_drag(500.0, 100.0); // drag 80px down
    let opened = coord.handle_pointer_up();
    assert_eq!(opened, Some(DrawerTarget::Dashboard));
    assert!(coord.visibilities.dashboard);
}
