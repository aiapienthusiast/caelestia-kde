//! Sidebar Subsystem
//! Complete parity with `shell/modules/sidebar/`.
//!
//! Provides the sliding desktop sidebar featuring:
//! - Notifications history and management
//! - AI Assistant conversation interface
//! - News feed integration
//! - Pinned mode and dynamic edge resizing

pub mod ai;
pub mod ai_assistant;
pub mod config;
pub mod news;
pub mod state;
pub mod tabs;
pub mod view;

pub use ai::{
    ChatAttachment, ChatMarkdownParser, ChatSessionData, ChatStoreManager, MarkdownBlock,
    ToolCallCardModel, ToolExecutionStatus,
};
pub use ai_assistant::{AiAssistantModel, ChatMessage, MessageRole};
pub use config::SidebarConfig;
pub use news::{NewsItem, NewsModel};
pub use state::SidebarState;
pub use tabs::{SidebarTab, SidebarTabDefinition};
pub use view::SidebarView;

