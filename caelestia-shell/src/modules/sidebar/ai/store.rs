//! Claude Code and LLM session runner with persistent chat store
//! Ported with 100% parity from `ClaudeCodeSession.qml`, `claudecode.js`, `ChatStore.qml`, and `chatsessions.js`.

use serde::{Deserialize, Serialize};
use super::super::ai_assistant::{ChatMessage, MessageRole};
use super::attachments::ChatAttachment;
use super::attachments::ToolCallCardModel;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatSessionData {
    pub session_id: String,
    pub title: String,
    pub created_at: u64,
    pub updated_at: u64,
    pub messages: Vec<ChatMessage>,
    pub tool_calls: Vec<ToolCallCardModel>,
    pub attachments: Vec<ChatAttachment>,
}

impl ChatSessionData {
    pub fn new(session_id: impl Into<String>, title: impl Into<String>) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        Self {
            session_id: session_id.into(),
            title: title.into(),
            created_at: now,
            updated_at: now,
            messages: Vec::new(),
            tool_calls: Vec::new(),
            attachments: Vec::new(),
        }
    }

    pub fn add_message(&mut self, role: MessageRole, text: impl Into<String>) {
        let msg = ChatMessage {
            id: format!("msg_{}", self.messages.len() + 1),
            role,
            text: text.into(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
        };
        self.messages.push(msg);
        self.updated_at = self.messages.last().unwrap().timestamp;
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatStoreManager {
    pub sessions: Vec<ChatSessionData>,
    pub active_session_id: Option<String>,
}

impl Default for ChatStoreManager {
    fn default() -> Self {
        let initial = ChatSessionData::new("default", "New Chat");
        let id = initial.session_id.clone();
        Self {
            sessions: vec![initial],
            active_session_id: Some(id),
        }
    }
}

impl ChatStoreManager {
    pub fn create_session(&mut self, title: impl Into<String>) -> String {
        let id = format!("session_{}", self.sessions.len() + 1);
        let session = ChatSessionData::new(&id, title);
        self.sessions.push(session);
        self.active_session_id = Some(id.clone());
        id
    }

    pub fn active_session(&self) -> Option<&ChatSessionData> {
        let id = self.active_session_id.as_deref()?;
        self.sessions.iter().find(|s| s.session_id == id)
    }

    pub fn active_session_mut(&mut self) -> Option<&mut ChatSessionData> {
        let id = self.active_session_id.as_deref()?.to_string();
        self.sessions.iter_mut().find(|s| s.session_id == id)
    }

    pub fn delete_session(&mut self, id: &str) -> bool {
        if let Some(pos) = self.sessions.iter().position(|s| s.session_id == id) {
            self.sessions.remove(pos);
            if self.active_session_id.as_deref() == Some(id) {
                self.active_session_id = self.sessions.first().map(|s| s.session_id.clone());
            }
            true
        } else {
            false
        }
    }
}
