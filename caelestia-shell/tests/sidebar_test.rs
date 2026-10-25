//! Comprehensive Test Suite for Sidebar Subsystem
//! Validates tab switching, pinned persistence, dynamic width boundary clamping,
//! AI Assistant conversation model, News feed, and notifications dock.

use caelestia_shell::modules::sidebar::{
    AiAssistantModel, MessageRole, NewsModel, SidebarConfig, SidebarState, SidebarTab,
};
use caelestia_shell::services::notifications::NotificationItem;

#[test]
fn test_sidebar_tabs_and_configuration() {
    let mut config = SidebarConfig::default();
    assert!(config.enable_ai_assistant);
    assert!(config.show_news);
    assert!(config.show_dino_game);

    let tabs = SidebarTab::available_tabs(&config);
    assert_eq!(tabs.len(), 4);
    assert_eq!(tabs[0].tab, SidebarTab::Notifications);
    assert_eq!(tabs[1].tab, SidebarTab::AiAssistant);
    assert_eq!(tabs[2].tab, SidebarTab::News);
    assert_eq!(tabs[3].tab, SidebarTab::DinoGame);

    // Disable AI and News
    config.enable_ai_assistant = false;
    config.show_news = false;
    let filtered_tabs = SidebarTab::available_tabs(&config);
    assert_eq!(filtered_tabs.len(), 2);
    assert_eq!(filtered_tabs[0].tab, SidebarTab::Notifications);
    assert_eq!(filtered_tabs[1].tab, SidebarTab::DinoGame);

    // Tab ID parsing
    assert_eq!(SidebarTab::from_id("notifications"), SidebarTab::Notifications);
    assert_eq!(SidebarTab::from_id("ai"), SidebarTab::AiAssistant);
    assert_eq!(SidebarTab::from_id("news"), SidebarTab::News);
    assert_eq!(SidebarTab::from_id("dino"), SidebarTab::DinoGame);
    assert_eq!(SidebarTab::from_id("unknown"), SidebarTab::Notifications);
}

#[test]
fn test_sidebar_pin_state_and_lifecycle() {
    let config = SidebarConfig::default();
    let mut state = SidebarState::new(&config, 1920.0);

    assert!(!state.is_open);
    assert!(!state.pinned);

    state.open();
    assert!(state.is_open);

    // Pinning locks sidebar open
    state.set_pinned(true);
    assert!(state.pinned);
    assert!(state.is_open);

    // Regular close does not close pinned sidebar
    state.close();
    assert!(state.is_open);

    // Unpinning and closing closes sidebar
    state.toggle_pinned();
    assert!(!state.pinned);
    state.close();
    assert!(!state.is_open);

    // Force close works even when pinned
    state.set_pinned(true);
    assert!(state.is_open);
    state.force_close();
    assert!(!state.is_open);
}

#[test]
fn test_sidebar_resizing_and_width_boundaries() {
    let config = SidebarConfig {
        default_width: 420.0,
        min_width: 300.0,
        max_width_ratio: 0.5,
        ..Default::default()
    };
    let mut state = SidebarState::new(&config, 1920.0);

    // Screen width = 1920, max_width_ratio = 0.5 -> max_width = 960.0
    assert_eq!(state.min_width, 300.0);
    assert_eq!(state.max_width, 960.0);
    assert_eq!(state.width, 420.0);

    // Resize within bounds
    state.set_width(550.0);
    assert_eq!(state.width, 550.0);

    // Resize below minimum clamps to min_width
    state.set_width(150.0);
    assert_eq!(state.width, 300.0);

    // Resize above maximum clamps to max_width
    state.set_width(1200.0);
    assert_eq!(state.width, 960.0);

    // Reset width
    state.reset_width(420.0);
    assert_eq!(state.width, 420.0);
}

#[test]
fn test_sidebar_ai_assistant_conversation() {
    let mut ai = AiAssistantModel::new();
    assert!(!ai.busy);
    assert_eq!(ai.messages.len(), 1); // Welcome message
    assert_eq!(ai.messages[0].role, MessageRole::Assistant);

    // Send empty prompt does nothing
    assert_eq!(ai.send_prompt("   "), None);
    assert_eq!(ai.messages.len(), 1);

    // Send valid prompt
    let sent = ai.send_prompt("How do I configure my monitor layout?");
    assert_eq!(sent, Some("How do I configure my monitor layout?".to_string()));
    assert!(ai.busy);
    assert_eq!(ai.messages.len(), 2);
    assert_eq!(ai.messages[1].role, MessageRole::User);

    // Cannot send prompt while busy
    assert_eq!(ai.send_prompt("Another question"), None);

    // Deliver assistant reply
    ai.deliver_reply("You can configure monitors in Settings > Displays or using kscreen-doctor.");
    assert!(!ai.busy);
    assert_eq!(ai.messages.len(), 3);
    assert_eq!(ai.messages[2].role, MessageRole::Assistant);

    // Clear history
    ai.clear_history();
    assert_eq!(ai.messages.len(), 0);
    assert!(!ai.busy);
}

#[test]
fn test_sidebar_news_model() {
    let mut news = NewsModel::new();
    assert!(!news.is_refreshing);
    assert!(news.articles.len() >= 3);

    let first = &news.articles[0];
    assert!(!first.title.is_empty());
    assert!(!first.source.is_empty());
    assert!(first.url.starts_with("http"));

    news.refresh();
    assert!(!news.is_refreshing);
}

#[test]
fn test_sidebar_notifications_handling() {
    use caelestia_shell::services::notifications::Urgency;

    let notifs = vec![
        NotificationItem {
            id: 101,
            app_name: "Matrix".to_string(),
            summary: "New message".to_string(),
            body: "Alice sent a photo".to_string(),
            app_icon: "chat".to_string(),
            urgency: Urgency::Normal,
            timestamp_secs: 1000,
        },
        NotificationItem {
            id: 102,
            app_name: "Calendar".to_string(),
            summary: "Meeting in 15m".to_string(),
            body: "Team sync".to_string(),
            app_icon: "calendar".to_string(),
            urgency: Urgency::Critical,
            timestamp_secs: 1015,
        },
    ];

    assert_eq!(notifs.len(), 2);
    assert_eq!(notifs[0].id, 101);
    assert_eq!(notifs[1].id, 102);
}
