//! Screenshot Action Builder
//! Complete parity with `shell/modules/screenshot/ScreenshotAction.qml`.

use super::config::ScreenshotConfig;
use super::region::SelectionRect;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Actions available after selecting a region or window
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SnipAction {
    Copy,
    Edit,
    Search,
    CharRecognition,
    Record,
    RecordWithSound,
}

impl Default for SnipAction {
    fn default() -> Self {
        Self::Copy
    }
}

/// Helper to safely escape single-quoted shell strings
pub fn escape_shell_str(s: &str) -> String {
    s.replace('\'', "'\\''")
}

/// Generate pipeline command script matching `ScreenshotAction.qml:getScript`
pub fn build_action_script(
    rect: &SelectionRect,
    screenshot_path: &Path,
    action: SnipAction,
    config: &ScreenshotConfig,
) -> String {
    let rx = rect.x.round() as i32;
    let ry = rect.y.round() as i32;
    let rw = rect.width.round() as u32;
    let rh = rect.height.round() as u32;

    let shot_str = escape_shell_str(&screenshot_path.to_string_lossy());
    let crop_base = format!("magick '{}' -crop {}x{}+{}+{} +repage", shot_str, rw, rh, rx, ry);
    let cleanup = format!("rm -f '{}'", shot_str);
    let save_dir = escape_shell_str(&config.screenshots_dir.to_string_lossy());

    match action {
        SnipAction::Copy => {
            format!(
                "set -euo pipefail; \
                SAVE_DIR='{save_dir}'; \
                SAVE_DIR=\"${{SAVE_DIR/#\\~/$HOME}}\"; \
                mkdir -p \"$SAVE_DIR\" && \
                saveFile=\"$SAVE_DIR/screenshot-$(date +%Y-%m-%d_%H.%M.%S).png\" && \
                {crop_base} \"$saveFile\" && \
                wl-copy -t image/png < \"$saveFile\"; \
                {cleanup}; \
                (notify-send \"Screenshot Captured\" \"Saved to $saveFile\" -i \"$saveFile\" -a \"Screenshot\" || true) &"
            )
        }
        SnipAction::Edit => {
            format!(
                "set -euo pipefail; \
                SAVE_DIR='{save_dir}'; \
                SAVE_DIR=\"${{SAVE_DIR/#\\~/$HOME}}\"; \
                mkdir -p \"$SAVE_DIR\" && \
                saveFile=\"$SAVE_DIR/screenshot-$(date +%Y-%m-%d_%H.%M.%S).png\" && \
                TMPF=$(mktemp /tmp/qs-snip-XXXXXX.png); \
                {crop_base} \"$TMPF\" && \
                swappy -f \"$TMPF\" -o \"$saveFile\" || true; \
                wl-copy -t image/png < \"$saveFile\"; \
                rm -f \"$TMPF\"; {cleanup}"
            )
        }
        SnipAction::Search => {
            let tmp_file = "/tmp/snip-search.png";
            let endpoint = &config.file_upload_endpoint;
            let search_url = &config.image_search_url;
            format!(
                "set -euo pipefail; \
                {crop_base} '{tmp_file}' && \
                URL=$(curl -sF files[]=@'{tmp_file}' {endpoint} | jq -r '.files[0].url') && \
                xdg-open \"{search_url}$URL\"; \
                rm -f '{tmp_file}'; {cleanup}"
            )
        }
        SnipAction::CharRecognition => {
            let tmp_file = "/tmp/snip-ocr.png";
            let ocr = &config.ocr_engine;
            format!(
                "set -euo pipefail; \
                {crop_base} '{tmp_file}' && \
                TEXT=$({ocr} '{tmp_file}' stdout 2>/dev/null) && \
                echo -n \"$TEXT\" | wl-copy && \
                notify-send \"Text Extracted\" \"$TEXT\" -a \"OCR\" && \
                rm -f '{tmp_file}'; {cleanup}"
            )
        }
        SnipAction::Record => {
            format!(
                "set -euo pipefail; \
                {cleanup}; \
                gpu-screen-recorder -w screen -f 60 -q ultra -a \"default_output\" -c mp4 -r 0 -o /tmp/recording.mp4 &"
            )
        }
        SnipAction::RecordWithSound => {
            format!(
                "set -euo pipefail; \
                {cleanup}; \
                gpu-screen-recorder -w screen -f 60 -q ultra -a \"default_output|default_input\" -c mp4 -r 0 -o /tmp/recording.mp4 &"
            )
        }
    }
}
