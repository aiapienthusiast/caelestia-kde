//! Screenshot Subsystem Configuration
//! Complete parity with `shell/modules/screenshot/ScreenshotAction.qml` and `Config.screenshot`.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Screenshot configuration parameters
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScreenshotConfig {
    pub screenshots_dir: PathBuf,
    pub save_format: String,
    pub image_search_url: String,
    pub file_upload_endpoint: String,
    pub ocr_engine: String,
    pub copy_to_clipboard: bool,
    pub show_notifications: bool,
}

impl Default for ScreenshotConfig {
    fn default() -> Self {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
        Self {
            screenshots_dir: PathBuf::from(format!("{}/Pictures/Screenshots", home)),
            save_format: "screenshot-%Y-%m-%d_%H.%M.%S.png".to_string(),
            image_search_url: "https://lens.google.com/uploadbyurl?url=".to_string(),
            file_upload_endpoint: "https://uguu.se/upload".to_string(),
            ocr_engine: "tesseract".to_string(),
            copy_to_clipboard: true,
            show_notifications: true,
        }
    }
}
