//! Sidebar Tabs Specification
//! Complete parity with `shell/modules/sidebar/Content.qml` tab model.

use super::config::SidebarConfig;
use serde::{Deserialize, Serialize};

/// Available tab views in the sidebar
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SidebarTab {
    Notifications,
    AiAssistant,
    News,
    DinoGame,
}

impl Default for SidebarTab {
    fn default() -> Self {
        Self::Notifications
    }
}

/// Metadata definition for rendering tab buttons and indicators
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SidebarTabDefinition {
    pub tab: SidebarTab,
    pub id: &'static str,
    pub label: &'static str,
    pub icon: &'static str,
}

impl SidebarTab {
    pub fn id(&self) -> &'static str {
        match self {
            Self::Notifications => "notifications",
            Self::AiAssistant => "ai",
            Self::News => "news",
            Self::DinoGame => "dino",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Notifications => "Notifications",
            Self::AiAssistant => "AI Assistant",
            Self::News => "News",
            Self::DinoGame => "Dino Game",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Self::Notifications => "notifications",
            Self::AiAssistant => "smart_toy",
            Self::News => "newspaper",
            Self::DinoGame => "sports_esports",
        }
    }

    pub fn from_id(id: &str) -> Self {
        match id {
            "ai" => Self::AiAssistant,
            "news" => Self::News,
            "dino" => Self::DinoGame,
            _ => Self::Notifications,
        }
    }

    /// List active tabs according to sidebar configuration
    pub fn available_tabs(config: &SidebarConfig) -> Vec<SidebarTabDefinition> {
        let mut tabs = vec![SidebarTabDefinition {
            tab: Self::Notifications,
            id: Self::Notifications.id(),
            label: Self::Notifications.label(),
            icon: Self::Notifications.icon(),
        }];

        if config.enable_ai_assistant {
            tabs.push(SidebarTabDefinition {
                tab: Self::AiAssistant,
                id: Self::AiAssistant.id(),
                label: Self::AiAssistant.label(),
                icon: Self::AiAssistant.icon(),
            });
        }

        if config.show_news {
            tabs.push(SidebarTabDefinition {
                tab: Self::News,
                id: Self::News.id(),
                label: Self::News.label(),
                icon: Self::News.icon(),
            });
        }

        if config.show_dino_game {
            tabs.push(SidebarTabDefinition {
                tab: Self::DinoGame,
                id: Self::DinoGame.id(),
                label: Self::DinoGame.label(),
                icon: Self::DinoGame.icon(),
            });
        }

        tabs
    }
}
