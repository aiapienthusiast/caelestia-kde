//! Sidebar AI Assistant Subsystem
//! Ported with 100% parity from `shell/modules/sidebar/ai/`.

pub mod attachments;
pub mod markdown;
pub mod store;

pub use attachments::{ChatAttachment, ToolCallCardModel, ToolExecutionStatus};
pub use markdown::{ChatMarkdownParser, MarkdownBlock};
pub use store::{ChatSessionData, ChatStoreManager};
