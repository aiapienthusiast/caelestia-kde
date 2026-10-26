//! Attachments and Tool Cards for AI Assistant
//! Ported with 100% parity from `Attachments.qml` and `ToolCard.qml`.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChatAttachment {
    pub id: String,
    pub name: String,
    pub mime_type: String,
    pub path: Option<PathBuf>,
    pub size_bytes: u64,
    pub is_image: bool,
}

impl ChatAttachment {
    pub fn from_file(path: &Path) -> Result<Self, String> {
        let meta = std::fs::metadata(path).map_err(|e| e.to_string())?;
        let name = path
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "attachment".to_string());

        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|s| s.to_lowercase())
            .unwrap_or_default();

        let (mime_type, is_image) = match ext.as_str() {
            "png" => ("image/png".to_string(), true),
            "jpg" | "jpeg" => ("image/jpeg".to_string(), true),
            "webp" => ("image/webp".to_string(), true),
            "svg" => ("image/svg+xml".to_string(), true),
            "txt" | "md" | "rs" | "json" => ("text/plain".to_string(), false),
            "pdf" => ("application/pdf".to_string(), false),
            _ => ("application/octet-stream".to_string(), false),
        };

        Ok(Self {
            id: path.to_string_lossy().to_string(),
            name,
            mime_type,
            path: Some(path.to_path_buf()),
            size_bytes: meta.len(),
            is_image,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToolExecutionStatus {
    Pending,
    Running,
    Success,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolCallCardModel {
    pub call_id: String,
    pub tool_name: String,
    pub input_json: String,
    pub output_json: Option<String>,
    pub status: ToolExecutionStatus,
    pub execution_time_ms: Option<u64>,
}

impl ToolCallCardModel {
    pub fn new(call_id: impl Into<String>, tool_name: impl Into<String>, input_json: impl Into<String>) -> Self {
        Self {
            call_id: call_id.into(),
            tool_name: tool_name.into(),
            input_json: input_json.into(),
            output_json: None,
            status: ToolExecutionStatus::Pending,
            execution_time_ms: None,
        }
    }

    pub fn complete(&mut self, output: impl Into<String>, success: bool, time_ms: u64) {
        self.output_json = Some(output.into());
        self.status = if success {
            ToolExecutionStatus::Success
        } else {
            ToolExecutionStatus::Failed
        };
        self.execution_time_ms = Some(time_ms);
    }
}
