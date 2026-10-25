//! Workspaces Bar Widget
//! Complete parity with `shell/modules/bar/components/workspaces/Workspaces.qml`.

use crate::services::kwin::KWinService;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspacePill {
    pub index: usize,
    pub id: u32,
    pub name: String,
    pub active: bool,
    pub occupied: bool,
}

pub struct WorkspacesModel;

impl WorkspacesModel {
    /// Compute workspace pills matching Workspaces.qml logic
    pub fn compute_pills(
        kwin: &KWinService,
        shown_count: usize,
        show_unoccupied: bool,
        screen_name: Option<&str>,
    ) -> Vec<WorkspacePill> {
        let total_count = kwin.state.workspaces.len().max(1);
        let active_idx = kwin.state.active_index as usize;

        // Check occupied workspaces from window_list
        let mut occupied_map = vec![false; total_count];
        for win in &kwin.state.window_list {
            if let Some(screen) = screen_name {
                if !win.output.is_empty() && win.output != screen {
                    continue;
                }
            }
            for d in &win.desktops {
                if let Ok(idx) = d.parse::<usize>() {
                    if idx < total_count {
                        occupied_map[idx] = true;
                    }
                } else if let Some(idx) = kwin.index_for_id(d) {
                    if idx < total_count {
                        occupied_map[idx] = true;
                    }
                }
            }
        }

        let mut pills = Vec::new();
        let limit = shown_count.min(total_count);

        for i in 0..limit {
            let (id, name, active) = if let Some(ws) = kwin.state.workspaces.get(i) {
                (ws.id, ws.name.clone(), ws.active)
            } else {
                (i as u32, format!("{}", i + 1), i == active_idx)
            };

            let occupied = occupied_map.get(i).copied().unwrap_or(false);

            if show_unoccupied || occupied || active {
                pills.push(WorkspacePill {
                    index: i,
                    id,
                    name,
                    active,
                    occupied,
                });
            }
        }

        pills
    }
}
