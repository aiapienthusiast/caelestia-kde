//! AI Assistant Tab Subsystem
//! Complete parity with `shell/modules/sidebar/AiAssistant.qml` and `ai/` backend.

use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::info;

/// Message role for conversation tracking
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    User,
    Assistant,
    System,
}

/// Chat message entry in the assistant stream
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: String,
    pub role: MessageRole,
    pub text: String,
    pub timestamp: u64,
}

/// AI Assistant conversation manager
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiAssistantModel {
    pub model_name: String,
    pub messages: Vec<ChatMessage>,
    pub busy: bool,
    pub current_prompt: String,
}

impl Default for AiAssistantModel {
    fn default() -> Self {
        Self {
            model_name: "Local Assistant".to_string(),
            messages: vec![ChatMessage {
                id: "welcome-1".to_string(),
                role: MessageRole::Assistant,
                text: "Hello! How can I assist you today on Caelestia Shell?".to_string(),
                timestamp: 0,
            }],
            busy: false,
            current_prompt: String::new(),
        }
    }
}

impl AiAssistantModel {
    pub fn new() -> Self {
        Self::default()
    }

    /// Submit a user prompt to the assistant
    pub fn send_prompt(&mut self, text: &str) -> Option<String> {
        let trimmed = text.trim();
        if trimmed.is_empty() || self.busy {
            return None;
        }

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        // Push user message
        let user_msg = ChatMessage {
            id: format!("usr-{}", now),
            role: MessageRole::User,
            text: trimmed.to_string(),
            timestamp: now,
        };
        self.messages.push(user_msg);
        self.current_prompt.clear();
        self.busy = true;

        info!("Submitted AI prompt: {}", trimmed);
        Some(trimmed.to_string())
    }

    /// Deliver mock or streaming assistant reply
    pub fn deliver_reply(&mut self, reply: &str) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        self.messages.push(ChatMessage {
            id: format!("ast-{}", now),
            role: MessageRole::Assistant,
            text: reply.to_string(),
            timestamp: now,
        });
        self.busy = false;
    }

    /// Clear conversation history
    pub fn clear_history(&mut self) {
        self.messages.clear();
        self.busy = false;
    }
}
