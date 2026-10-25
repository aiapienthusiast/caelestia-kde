//! Overview Workspace Indicator Model
//! Complete parity with `shell/modules/overview/WorkspaceIndicator.qml` and
//! `components/workspaces/ActiveIndicator.qml`.

use crate::services::kwin::KWinState;
use serde::{Deserialize, Serialize};

/// Visual representation of a workspace pill in the overview top strip
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspacePill {
    pub id: u32,
    pub index: usize,
    pub name: String,
    pub active: bool,
    pub occupied: bool,
    pub window_count: usize,
}

/// Dynamic workspace indicator state for the overview bar
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceIndicatorModel {
    pub pills: Vec<WorkspacePill>,
    pub active_index: usize,
    pub swipe_offset: f64,
    pub is_swiping: bool,
    pub scale_factor: f32,
    pub required_width: f32,
}

impl WorkspaceIndicatorModel {
    /// Compute indicator pills from current KWin state
    pub fn compute(kwin_state: &KWinState, screen_name: Option<&str>, max_width: f32) -> Self {
        let count = kwin_state.workspaces.len().max(1);

        // Track occupied workspaces and per-workspace window counts
        let mut occupied_map = std::collections::HashMap::new();
        let mut count_map = std::collections::HashMap::new();

        for win in &kwin_state.window_list {
            if let Some(screen) = screen_name {
                if !win.output.is_empty() && win.output != screen {
                    continue;
                }
            }

            for d in &win.desktops {
                if let Ok(id) = d.parse::<u32>() {
                    occupied_map.insert(id, true);
                    *count_map.entry(id).or_insert(0usize) += 1;
                }
                // Also check matching UUID
                if let Some(pos) = kwin_state.workspaces.iter().position(|ws| ws.uuid == *d) {
                    let id = (pos + 1) as u32;
                    occupied_map.insert(id, true);
                    *count_map.entry(id).or_insert(0usize) += 1;
                }
            }
        }

        let mut pills = Vec::with_capacity(count);
        for (i, ws) in kwin_state.workspaces.iter().enumerate() {
            let wid = ws.id;
            let is_occupied = *occupied_map.get(&wid).unwrap_or(&false);
            let win_count = *count_map.get(&wid).unwrap_or(&0);

            pills.push(WorkspacePill {
                id: wid,
                index: i,
                name: ws.name.clone(),
                active: ws.active,
                occupied: is_occupied,
                window_count: win_count,
            });
        }

        let active_index = pills.iter().position(|p| p.active).unwrap_or(0);

        let swipe_offset = if let Some(screen) = screen_name {
            kwin_state.swipe_offset_by_output.get(screen).copied().unwrap_or(kwin_state.swipe_offset)
        } else {
            kwin_state.swipe_offset
        };

        // Width calculations matching WorkspaceIndicator.qml
        let base_scale: f32 = 1.0;
        let unscaled_width = (count as f32 + 1.0) * 200.0 + (count as f32) * 8.0;
        let scale_factor = if (unscaled_width * base_scale) > max_width && max_width > 0.0 {
            max_width / unscaled_width
        } else {
            base_scale
        };
        let required_width = unscaled_width * scale_factor;

        Self {
            pills,
            active_index,
            swipe_offset,
            is_swiping: swipe_offset.abs() > 0.001,
            scale_factor,
            required_width,
        }
    }
}
