use caelestia_shell::modules::launcher::entry::DesktopEntry;
use caelestia_shell::modules::launcher::items::*;
use caelestia_shell::modules::launcher::views::*;

#[test]
fn test_app_browser_navigation_grid() {
    let mut model = AppBrowserModel::new(BrowserLayoutMode::Grid);
    assert_eq!(model.columns, 5);

    let mut apps = Vec::new();
    for i in 0..12 {
        apps.push(DesktopEntry {
            id: format!("app{i}.desktop"),
            name: format!("App {i}"),
            generic_name: String::new(),
            comment: format!("Description {i}"),
            icon: "application-x-executable".to_string(),
            exec: format!("app{i}"),
            categories: vec!["Utility".to_string()],
            keywords: vec![],
            run_in_terminal: false,
            no_display: false,
            path: None,
        });
    }

    model.set_apps(apps);
    assert_eq!(model.selected_index, 0);

    // Navigate right
    model.select_next();
    assert_eq!(model.selected_index, 1);

    // Navigate down (down jumps by columns = 5)
    model.select_down();
    assert_eq!(model.selected_index, 6);

    // Navigate up
    model.select_up();
    assert_eq!(model.selected_index, 1);

    // Navigate prev with wrap-around
    model.select_prev();
    assert_eq!(model.selected_index, 0);
    model.select_prev();
    assert_eq!(model.selected_index, 11);
}

#[test]
fn test_keybinds_view_filter() {
    let mut model = KeybindsViewModel::default();
    assert!(model.filtered_keybinds().len() >= 4);

    model.query = "terminal".to_string();
    let res = model.filtered_keybinds();
    assert_eq!(res.len(), 1);
    assert_eq!(res[0].keybind_id, "terminal");

    model.query = "Super".to_string();
    let res2 = model.filtered_keybinds();
    assert!(res2.len() >= 3);
}

#[test]
fn test_window_switcher_view_filter() {
    let mut model = WindowSwitcherViewModel::default();
    model.windows = vec![
        WindowSwitcherItemDef {
            window_id: 1,
            title: "Cargo Build - Konsole".to_string(),
            app_id: "org.kde.konsole".to_string(),
            workspace_id: 1,
            is_active: true,
            icon_name: "utilities-terminal".to_string(),
        },
        WindowSwitcherItemDef {
            window_id: 2,
            title: "Firefox Nightly".to_string(),
            app_id: "firefox".to_string(),
            workspace_id: 2,
            is_active: false,
            icon_name: "firefox".to_string(),
        },
    ];

    model.query = "firefox".to_string();
    let res = model.filtered_windows();
    assert_eq!(res.len(), 1);
    assert_eq!(res[0].window_id, 2);
}

#[test]
fn test_launcher_search_result_formatting() {
    let calc = LauncherSearchResult::Calc(CalcResultItem {
        expression: "2 + 2".to_string(),
        result: "4".to_string(),
        copy_payload: "4".to_string(),
    });
    assert_eq!(calc.title(), "4");
    assert_eq!(calc.subtitle(), "2 + 2");

    let emoji = LauncherSearchResult::Emoji(EmojiResultItem {
        emoji: "🚀".to_string(),
        name: "Rocket".to_string(),
        keywords: vec!["space".to_string(), "launch".to_string()],
    });
    assert_eq!(emoji.title(), "Rocket");
    assert_eq!(emoji.subtitle(), "🚀");

    let action = LauncherSearchResult::Action(ActionLauncherItem {
        action_id: "lock".to_string(),
        title: "Lock Screen".to_string(),
        description: "Lock the current session".to_string(),
        icon_name: "system-lock-screen".to_string(),
        command: "loginctl lock-session".to_string(),
    });
    assert_eq!(action.title(), "Lock Screen");
    assert_eq!(action.subtitle(), "Lock the current session");
}
