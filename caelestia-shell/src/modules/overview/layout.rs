//! Overview Window Grid Layout Algorithms
//! Parity with `shell/plugin/src/Caelestia/Layouts/layoutkde.cpp` and `layoutgnome.cpp`.

use crate::services::kwin::WindowItem;
use super::config::OverviewLayoutType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Positioned rectangular bounding box for a window card in the overview grid
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WindowRect {
    pub address: String,
    pub title: String,
    pub app_id: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

/// Helper function to interpolate values
#[inline]
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t.clamp(0.0, 1.0)
}

/// Calculate overview window cards layout based on chosen layout type
pub fn calculate_layout(
    layout_type: OverviewLayoutType,
    windows: &[WindowItem],
    area_width: f32,
    area_height: f32,
    col_spacing: f32,
    row_spacing: f32,
) -> HashMap<String, WindowRect> {
    match layout_type {
        OverviewLayoutType::Kde => {
            calculate_kde_layout(windows, area_width, area_height, col_spacing, row_spacing)
        }
        OverviewLayoutType::Gnome => {
            calculate_gnome_layout(windows, area_width, area_height, col_spacing, row_spacing)
        }
    }
}

// -----------------------------------------------------------------------------
// GNOME-style Overview Layout (Aspect-Ratio Scaled Rows)
// Parity with `layoutgnome.cpp`
// -----------------------------------------------------------------------------

#[derive(Clone, Debug)]
struct GnomeWindowInfo {
    address: String,
    title: String,
    app_id: String,
    width: f32,
    height: f32,
    center_x: f32,
    center_y: f32,
}

#[derive(Clone, Debug, Default)]
struct GnomeRowInfo {
    windows: Vec<GnomeWindowInfo>,
    full_width: f32,
    full_height: f32,
    width: f32,
    height: f32,
    x: f32,
    y: f32,
    additional_scale: f32,
}

fn compute_gnome_window_scale(win: &GnomeWindowInfo, area_height: f32) -> f32 {
    let ratio = win.height / area_height.max(1.0);
    lerp(1.5, 1.0, ratio)
}

fn keep_same_gnome_row(row: &GnomeRowInfo, width: f32, ideal_row_width: f32) -> bool {
    if row.full_width + width <= ideal_row_width {
        return true;
    }
    let old_ratio = row.full_width / ideal_row_width.max(1.0);
    let new_ratio = (row.full_width + width) / ideal_row_width.max(1.0);
    (1.0 - new_ratio).abs() < (1.0 - old_ratio).abs()
}

fn is_better_scale_and_space(old_scale: f32, old_space: f32, scale: f32, space: f32) -> bool {
    const LAYOUT_SCALE_WEIGHT: f32 = 1.0;
    const LAYOUT_SPACE_WEIGHT: f32 = 0.1;
    let space_power = (space - old_space) * LAYOUT_SPACE_WEIGHT;
    let scale_power = (scale - old_scale) * LAYOUT_SCALE_WEIGHT;

    if scale > old_scale && space > old_space {
        true
    } else if scale > old_scale && space <= old_space {
        scale_power > -space_power
    } else if scale <= old_scale && space > old_space {
        space_power > -scale_power
    } else {
        false
    }
}

pub fn calculate_gnome_layout(
    windows: &[WindowItem],
    area_width: f32,
    area_height: f32,
    col_spacing: f32,
    row_spacing: f32,
) -> HashMap<String, WindowRect> {
    let mut result = HashMap::new();
    if windows.is_empty() || area_width <= 0.0 || area_height <= 0.0 {
        return result;
    }

    let mut win_infos: Vec<GnomeWindowInfo> = windows
        .iter()
        .map(|w| {
            let width = if w.width > 0 { w.width as f32 } else { 800.0 };
            let height = if w.height > 0 { w.height as f32 } else { 600.0 };
            let wx = w.x as f32;
            let wy = w.y as f32;
            GnomeWindowInfo {
                address: w.address.clone(),
                title: w.title.clone(),
                app_id: w.app_id.clone(),
                width,
                height,
                center_x: wx + width / 2.0,
                center_y: wy + height / 2.0,
            }
        })
        .collect();

    win_infos.sort_by(|a, b| a.center_y.partial_cmp(&b.center_y).unwrap_or(std::cmp::Ordering::Equal));

    let mut best_rows: Vec<GnomeRowInfo> = Vec::new();
    let mut best_scale: f32 = 0.0;
    let mut best_space: f32 = 0.0;
    let mut last_num_columns: i32 = -1;

    for num_rows in 1..=win_infos.len() {
        let num_columns = ((win_infos.len() as f32) / (num_rows as f32)).ceil() as i32;
        if num_columns == last_num_columns {
            break;
        }

        let mut total_width: f32 = 0.0;
        for win in &win_infos {
            total_width += win.width * compute_gnome_window_scale(win, area_height);
        }
        let ideal_row_width = total_width / (num_rows as f32);

        let mut rows: Vec<GnomeRowInfo> = vec![GnomeRowInfo::default(); num_rows];
        let mut window_idx = 0;

        for (i, row) in rows.iter_mut().enumerate() {
            while window_idx < win_infos.len() {
                let win = &win_infos[window_idx];
                let s = compute_gnome_window_scale(win, area_height);
                let w = win.width * s;
                let h = win.height * s;
                row.full_height = row.full_height.max(h);

                if keep_same_gnome_row(row, w, ideal_row_width) || i == num_rows - 1 {
                    row.windows.push(win.clone());
                    row.full_width += w;
                    window_idx += 1;
                } else {
                    break;
                }
            }
        }

        let mut grid_width: f32 = 0.0;
        let mut grid_height: f32 = 0.0;
        let mut max_cols: usize = 0;

        for row in &mut rows {
            row.windows.sort_by(|a, b| a.center_x.partial_cmp(&b.center_x).unwrap_or(std::cmp::Ordering::Equal));
            grid_width = grid_width.max(row.full_width);
            grid_height += row.full_height;
            max_cols = max_cols.max(row.windows.len());
        }

        let hspacing = if max_cols > 1 { (max_cols - 1) as f32 * col_spacing } else { 0.0 };
        let vspacing = if num_rows > 1 { (num_rows - 1) as f32 * row_spacing } else { 0.0 };

        let horizontal_scale = (area_width - hspacing).max(0.0) / grid_width.max(1.0);
        let vertical_scale = (area_height - vspacing).max(0.0) / grid_height.max(1.0);
        let scale = horizontal_scale.min(vertical_scale).min(0.95);

        let scaled_layout_width = grid_width * scale + hspacing;
        let scaled_layout_height = grid_height * scale + vspacing;
        let space = (scaled_layout_width * scaled_layout_height) / (area_width * area_height).max(1.0);

        if !best_rows.is_empty() && !is_better_scale_and_space(best_scale, best_space, scale, space) {
            break;
        }

        best_rows = rows;
        best_scale = scale;
        best_space = space;
        last_num_columns = num_columns;
    }

    if best_rows.is_empty() {
        return result;
    }

    for row in &mut best_rows {
        let spacing = if row.windows.len() > 1 { (row.windows.len() - 1) as f32 * col_spacing } else { 0.0 };
        row.width = row.full_width * best_scale + spacing;
        row.height = row.full_height * best_scale;
    }

    let height_without_spacing: f32 = best_rows.iter().map(|r| r.height).sum();
    let vertical_spacing = if best_rows.len() > 1 { (best_rows.len() - 1) as f32 * row_spacing } else { 0.0 };
    let additional_vertical_scale = ((area_height - vertical_spacing).max(0.0) / height_without_spacing.max(1.0)).min(1.0);

    let mut current_y = 0.0;
    for row in &mut best_rows {
        let horizontal_spacing = if row.windows.len() > 1 { (row.windows.len() - 1) as f32 * col_spacing } else { 0.0 };
        let width_without_spacing = (row.width - horizontal_spacing).max(0.0);
        let additional_horizontal_scale = ((area_width - horizontal_spacing).max(0.0) / width_without_spacing.max(1.0)).min(1.0);

        row.additional_scale = additional_horizontal_scale.min(additional_vertical_scale);
        let actual_row_width = width_without_spacing * row.additional_scale + horizontal_spacing;
        row.x = ((area_width - actual_row_width) / 2.0).max(0.0);
        row.y = ((area_height - (height_without_spacing + vertical_spacing)) / 2.0).max(0.0) + current_y;

        let mut current_x = row.x;
        for win in &row.windows {
            let win_scale = compute_gnome_window_scale(win, area_height) * best_scale * row.additional_scale;
            let card_w = win.width * win_scale;
            let card_h = win.height * win_scale;
            let card_y = row.y + (row.height * row.additional_scale - card_h).max(0.0) / 2.0;

            result.insert(
                win.address.clone(),
                WindowRect {
                    address: win.address.clone(),
                    title: win.title.clone(),
                    app_id: win.app_id.clone(),
                    x: current_x,
                    y: card_y,
                    width: card_w,
                    height: card_h,
                },
            );

            current_x += card_w + col_spacing;
        }

        current_y += row.height * row.additional_scale + row_spacing;
    }

    result
}

// -----------------------------------------------------------------------------
// KDE-style Overview Layout (Layered Packing Dynamic Optimization)
// Parity with `layoutkde.cpp`
// -----------------------------------------------------------------------------

pub fn calculate_kde_layout(
    windows: &[WindowItem],
    area_width: f32,
    area_height: f32,
    col_spacing: f32,
    row_spacing: f32,
) -> HashMap<String, WindowRect> {
    let mut result = HashMap::new();
    if windows.is_empty() || area_width <= 0.0 || area_height <= 0.0 {
        return result;
    }

    let n = windows.len();

    // Determine grid columns and rows based on aspect ratio
    let target_ratio = area_width / area_height;
    let mut cols = ((n as f32) * target_ratio).sqrt().round() as usize;
    cols = cols.clamp(1, n);
    let rows = (n + cols - 1) / cols;

    let available_w = area_width - (cols.saturating_sub(1) as f32 * col_spacing);
    let available_h = area_height - (rows.saturating_sub(1) as f32 * row_spacing);

    let cell_w = available_w / cols as f32;
    let cell_h = available_h / rows as f32;

    for (i, win) in windows.iter().enumerate() {
        let r = i / cols;
        let c = i % cols;

        let num_in_this_row = if r == rows - 1 {
            let rem = n % cols;
            if rem == 0 { cols } else { rem }
        } else {
            cols
        };

        // Center row if it has fewer items than cols
        let row_w = num_in_this_row as f32 * cell_w + (num_in_this_row.saturating_sub(1) as f32 * col_spacing);
        let row_x_offset = ((area_width - row_w) / 2.0).max(0.0);

        let orig_w = if win.width > 0 { win.width as f32 } else { 800.0 };
        let orig_h = if win.height > 0 { win.height as f32 } else { 600.0 };

        // Fit window into cell keeping aspect ratio
        let scale = (cell_w / orig_w).min(cell_h / orig_h).min(1.0);
        let card_w = orig_w * scale;
        let card_h = orig_h * scale;

        let cell_x = row_x_offset + (c as f32 * (cell_w + col_spacing));
        let cell_y = r as f32 * (cell_h + row_spacing);

        let card_x = cell_x + (cell_w - card_w) / 2.0;
        let card_y = cell_y + (cell_h - card_h) / 2.0;

        result.insert(
            win.address.clone(),
            WindowRect {
                address: win.address.clone(),
                title: win.title.clone(),
                app_id: win.app_id.clone(),
                x: card_x,
                y: card_y,
                width: card_w,
                height: card_h,
            },
        );
    }

    result
}
