use caelestia_shell::modules::launcher::*;

#[test]
fn test_desktop_entry_parsing() {
    let mock_desktop = r#"
[Desktop Entry]
Version=1.0
Name=Firefox
GenericName=Web Browser
Comment=Browse the World Wide Web
Icon=firefox
Exec=/usr/lib/firefox/firefox %u
Terminal=false
Type=Application
Categories=Network;WebBrowser;
Keywords=internet;browser;web;
"#;

    let entry = DesktopEntry::parse("firefox", mock_desktop, None).expect("Failed to parse desktop entry");
    assert_eq!(entry.id, "firefox");
    assert_eq!(entry.name, "Firefox");
    assert_eq!(entry.generic_name, "Web Browser");
    assert_eq!(entry.comment, "Browse the World Wide Web");
    assert_eq!(entry.icon, "firefox");
    assert_eq!(entry.exec, "/usr/lib/firefox/firefox %u");
    assert!(!entry.run_in_terminal);
    assert!(!entry.no_display);
    assert_eq!(entry.categories, vec!["Network", "WebBrowser"]);
    assert_eq!(entry.keywords, vec!["internet", "browser", "web"]);

    // Test NoDisplay omission
    let mock_hidden = r#"
[Desktop Entry]
Name=Hidden Utility
Exec=false
NoDisplay=true
"#;
    assert!(DesktopEntry::parse("hidden", mock_hidden, None).is_none());
}

#[test]
fn test_categories_mapping_and_filtering() {
    let app1 = DesktopEntry {
        id: "vscode".to_string(),
        name: "Visual Studio Code".to_string(),
        categories: vec!["Development".to_string(), "IDE".to_string()],
        ..Default::default()
    };
    let app2 = DesktopEntry {
        id: "spotify".to_string(),
        name: "Spotify".to_string(),
        categories: vec!["Audio".to_string(), "Music".to_string()],
        ..Default::default()
    };
    let app3 = DesktopEntry {
        id: "vlc".to_string(),
        name: "VLC".to_string(),
        categories: vec!["AudioVideo".to_string(), "Player".to_string()],
        ..Default::default()
    };
    let app4 = DesktopEntry {
        id: "custom".to_string(),
        name: "Custom Script".to_string(),
        categories: vec!["UnknownCategory".to_string()],
        ..Default::default()
    };

    let all_apps = vec![app1.clone(), app2.clone(), app3.clone(), app4.clone()];

    // Category mapping verification
    assert_eq!(Categories::category_for_app(&app1), "development");
    assert_eq!(Categories::category_for_app(&app2), "audio");
    assert_eq!(Categories::category_for_app(&app3), "audioVideo");
    assert_eq!(Categories::category_for_app(&app4), "other");

    // Filtering by category
    let dev_apps = Categories::apps_for("development", &all_apps, &[]);
    assert_eq!(dev_apps.len(), 1);
    assert_eq!(dev_apps[0].id, "vscode");

    let audio_apps = Categories::apps_for("audio", &all_apps, &[]);
    assert_eq!(audio_apps.len(), 1);
    assert_eq!(audio_apps[0].id, "spotify");

    let other_apps = Categories::apps_for("other", &all_apps, &[]);
    assert_eq!(other_apps.len(), 1);
    assert_eq!(other_apps[0].id, "custom");

    // Favorites filtering
    let fav_apps = Categories::apps_for("favorites", &all_apps, &["vscode".to_string()]);
    assert_eq!(fav_apps.len(), 1);
    assert_eq!(fav_apps[0].id, "vscode");

    // All category returns everything
    let all = Categories::apps_for("all", &all_apps, &[]);
    assert_eq!(all.len(), 4);
}

#[test]
fn test_searcher_prefix_modes_and_ranking() {
    // Mode parsing
    assert_eq!(AppSearcher::parse_mode(":i firefox"), (SearchMode::Id, "firefox"));
    assert_eq!(AppSearcher::parse_mode(":c network"), (SearchMode::Category, "network"));
    assert_eq!(AppSearcher::parse_mode(":d browser"), (SearchMode::Description, "browser"));
    assert_eq!(AppSearcher::parse_mode(":e /bin/sh"), (SearchMode::Exec, "/bin/sh"));
    assert_eq!(AppSearcher::parse_mode(":k code"), (SearchMode::Keywords, "code"));
    assert_eq!(AppSearcher::parse_mode(":t htop"), (SearchMode::TerminalOnly, "htop"));
    assert_eq!(AppSearcher::parse_mode(">clipboard query"), (SearchMode::Action(ActionCommand::Clipboard), "query"));
    assert_eq!(AppSearcher::parse_mode("firefox"), (SearchMode::General, "firefox"));

    let app1 = DesktopEntry {
        id: "firefox".to_string(),
        name: "Firefox".to_string(),
        generic_name: "Web Browser".to_string(),
        keywords: vec!["internet".to_string()],
        ..Default::default()
    };
    let app2 = DesktopEntry {
        id: "chromium".to_string(),
        name: "Chromium".to_string(),
        generic_name: "Web Browser".to_string(),
        keywords: vec!["browser".to_string()],
        ..Default::default()
    };
    let app3 = DesktopEntry {
        id: "htop".to_string(),
        name: "Htop".to_string(),
        generic_name: "Process Viewer".to_string(),
        run_in_terminal: true,
        ..Default::default()
    };

    let entries = vec![app1, app2, app3];

    // Search by name
    let res = AppSearcher::search("fire", &entries);
    assert_eq!(res.len(), 1);
    assert_eq!(res[0].id, "firefox");

    // Search by generic name / keywords matches both browsers
    let res_browser = AppSearcher::search("browser", &entries);
    assert_eq!(res_browser.len(), 2);

    // Terminal only filter
    let res_term = AppSearcher::search(":t", &entries);
    assert_eq!(res_term.len(), 1);
    assert_eq!(res_term[0].id, "htop");
}

#[test]
fn test_executor_command_building_and_field_code_stripping() {
    // Test stripping %u, %F, etc.
    let cmd = AppExecutor::build_command("firefox -new-tab %u", false);
    assert_eq!(cmd, vec!["firefox", "-new-tab"]);

    let quoted_cmd = AppExecutor::build_command(r#""/opt/google/chrome" --profile-directory="Default" %U"#, false);
    assert_eq!(quoted_cmd, vec!["/opt/google/chrome", "--profile-directory=Default"]);

    // Test terminal wrapping
    let term_cmd = AppExecutor::build_command("htop", true);
    assert!(term_cmd.len() >= 3);
    assert_eq!(term_cmd[1], "-e");
    assert_eq!(term_cmd[2], "htop");
}
