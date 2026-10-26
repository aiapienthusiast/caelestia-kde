//! Desktop icon grid layout and collision engine
//! Ported with 100% parity from `shell/modules/background/desktopicons/LayoutEngine.js`.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Cell span for a desktop item in grid cells
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Span {
    pub w: u32,
    pub h: u32,
}

impl Default for Span {
    fn default() -> Self {
        Self { w: 1, h: 1 }
    }
}

/// Grid coordinate position on the desktop
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GridPos {
    pub col: i32,
    pub row: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screen: Option<String>,
}

impl GridPos {
    pub fn new(col: i32, row: i32) -> Self {
        Self {
            col,
            row,
            screen: None,
        }
    }

    pub fn with_screen(col: i32, row: i32, screen: impl Into<String>) -> Self {
        Self {
            col,
            row,
            screen: Some(screen.into()),
        }
    }
}

/// Item metadata for sorting
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopItemMeta {
    pub key: String,
    pub name: String,
    pub kind: String, // "widget", "dir", "group", "file", etc.
    pub item_type: Option<String>,
    pub modified: Option<u64>,
    pub size: Option<u64>,
}

/// Pixel rectangle for hit testing
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PixelRect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

/// Direction for arrow-key navigation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavDirection {
    Left,
    Right,
    Up,
    Down,
}

/// Layout engine helper functions
pub struct LayoutEngine;

impl LayoutEngine {
    #[inline]
    pub fn cell_id(col: i32, row: i32) -> String {
        format!("{col},{row}")
    }

    pub fn span_of(spans: Option<&HashMap<String, Span>>, key: &str) -> Span {
        if let Some(map) = spans {
            if let Some(s) = map.get(key) {
                return Span {
                    w: s.w.max(1),
                    h: s.h.max(1),
                };
            }
        }
        Span::default()
    }

    pub fn in_grid(pos: &GridPos, cols: i32, rows: i32, span: Span) -> bool {
        pos.col >= 0
            && pos.row >= 0
            && pos.col + span.w as i32 <= cols
            && pos.row + span.h as i32 <= rows
    }

    pub fn mark_rect(
        occ: &mut HashMap<String, String>,
        pos: &GridPos,
        span: Span,
        key: &str,
    ) {
        for c in 0..span.w as i32 {
            for r in 0..span.h as i32 {
                occ.insert(Self::cell_id(pos.col + c, pos.row + r), key.to_string());
            }
        }
    }

    pub fn occupancy(
        positions: &HashMap<String, GridPos>,
        exclude_keys: Option<&[String]>,
        spans: Option<&HashMap<String, Span>>,
    ) -> HashMap<String, String> {
        let mut occ = HashMap::new();
        for (key, pos) in positions {
            if let Some(exclude) = exclude_keys {
                if exclude.iter().any(|k| k == key) {
                    continue;
                }
            }
            let span = Self::span_of(spans, key);
            Self::mark_rect(&mut occ, pos, span, key);
        }
        occ
    }

    pub fn rect_free(
        occ: &HashMap<String, String>,
        pos: &GridPos,
        span: Span,
    ) -> bool {
        for c in 0..span.w as i32 {
            for r in 0..span.h as i32 {
                if occ.contains_key(&Self::cell_id(pos.col + c, pos.row + r)) {
                    return false;
                }
            }
        }
        true
    }

    #[inline]
    pub fn order_index(pos: &GridPos, rows: i32) -> i32 {
        pos.col * rows + pos.row
    }

    pub fn ordered_keys(
        positions: &HashMap<String, GridPos>,
        rows: i32,
    ) -> Vec<String> {
        let mut keys: Vec<String> = positions.keys().cloned().collect();
        keys.sort_by_key(|k| {
            positions
                .get(k)
                .map(|p| Self::order_index(p, rows))
                .unwrap_or(i32::MAX)
        });
        keys
    }

    pub fn first_free(
        occ: &HashMap<String, String>,
        cols: i32,
        rows: i32,
        span: Span,
    ) -> GridPos {
        let h = (span.h as i32).min(rows);
        for i in 0..100_000 {
            let pos = GridPos::new(i / rows, i % rows);
            if pos.row + h > rows {
                continue;
            }
            if Self::rect_free(occ, &pos, Span { w: span.w, h: h as u32 }) {
                return pos;
            }
        }
        GridPos::new(cols, 0)
    }

    pub fn nearest_free(
        occ: &HashMap<String, String>,
        col: i32,
        row: i32,
        cols: i32,
        rows: i32,
        span: Span,
    ) -> GridPos {
        let mut best: Option<GridPos> = None;
        let mut best_dist = i64::MAX;

        for c in 0..=(cols - span.w as i32) {
            for r in 0..=(rows - span.h as i32) {
                let d = ((c - col) as i64).pow(2) + ((r - row) as i64).pow(2);
                if d > best_dist {
                    continue;
                }
                let pos = GridPos::new(c, r);
                if d == best_dist {
                    if let Some(ref current_best) = best {
                        if Self::order_index(&pos, rows) >= Self::order_index(current_best, rows) {
                            continue;
                        }
                    }
                }
                if !Self::rect_free(occ, &pos, span) {
                    continue;
                }
                best = Some(pos);
                best_dist = d;
            }
        }

        best.unwrap_or_else(|| Self::first_free(occ, cols, rows, span))
    }

    pub fn rects_overlap(a: &GridPos, sa: Span, b: &GridPos, sb: Span) -> bool {
        a.col < b.col + sb.w as i32
            && b.col < a.col + sa.w as i32
            && a.row < b.row + sb.h as i32
            && b.row < a.row + sa.h as i32
    }

    pub fn plan_move(
        positions: &HashMap<String, GridPos>,
        moving: &[String],
        anchor: &str,
        target: &GridPos,
        cols: i32,
        rows: i32,
        spans: Option<&HashMap<String, Span>>,
    ) -> HashMap<String, GridPos> {
        let mut out = positions.clone();
        let from = match positions.get(anchor) {
            Some(f) => f.clone(),
            None => return out,
        };

        let mut dc = target.col - from.col;
        let mut dr = target.row - from.row;

        for key in moving {
            if let Some(p) = positions.get(key) {
                let s = Self::span_of(spans, key);
                dc = dc.max(-p.col);
                dr = dr.max(-p.row);
                dc = dc.min((-p.col).max(cols - s.w as i32 - p.col));
                dr = dr.min((-p.row).max(rows - s.h as i32 - p.row));
            }
        }

        let mut occ = HashMap::new();
        let mut moved = Vec::new();

        for key in moving {
            if let Some(p) = positions.get(key) {
                let s = Self::span_of(spans, key);
                let new_pos = GridPos {
                    col: p.col + dc,
                    row: p.row + dr,
                    screen: p.screen.clone(),
                };
                Self::mark_rect(&mut occ, &new_pos, s, key);
                out.insert(key.clone(), new_pos);
                moved.push(key.clone());
            }
        }

        let mut displaced = Vec::new();
        for key in Self::ordered_keys(positions, rows) {
            if moving.iter().any(|m| m == &key) {
                continue;
            }
            let s = Self::span_of(spans, &key);
            let p = match positions.get(&key) {
                Some(pos) => pos,
                None => continue,
            };

            let overlaps = moved.iter().any(|m| {
                if let Some(mp) = out.get(m) {
                    let ms = Self::span_of(spans, m);
                    Self::rects_overlap(p, s, mp, ms)
                } else {
                    false
                }
            });

            if overlaps {
                displaced.push(key);
            } else {
                Self::mark_rect(&mut occ, p, s, &key);
            }
        }

        for key in displaced {
            if let Some(p) = positions.get(&key) {
                let s = Self::span_of(spans, &key);
                let cell = Self::nearest_free(&occ, p.col, p.row, cols, rows, s);
                let new_pos = GridPos {
                    col: cell.col,
                    row: cell.row,
                    screen: p.screen.clone(),
                };
                Self::mark_rect(&mut occ, &new_pos, s, &key);
                out.insert(key, new_pos);
            }
        }

        out
    }

    pub fn compact(
        keys: &[String],
        rows: i32,
        spans: Option<&HashMap<String, Span>>,
    ) -> HashMap<String, GridPos> {
        let mut out = HashMap::new();
        let mut occ = HashMap::new();

        for key in keys {
            let s = Self::span_of(spans, key);
            let pos = Self::first_free(&occ, i32::MAX, rows, s);
            Self::mark_rect(&mut occ, &pos, s, key);
            out.insert(key.clone(), pos);
        }

        out
    }

    pub fn plan_insert(
        positions: &HashMap<String, GridPos>,
        moving: &[String],
        target: &GridPos,
        rows: i32,
        spans: Option<&HashMap<String, Span>>,
    ) -> HashMap<String, GridPos> {
        let order = Self::ordered_keys(positions, rows);
        let target_index = Self::order_index(target, rows);

        let mut rest = Vec::new();
        let mut insert_at: Option<usize> = None;

        for key in &order {
            if moving.iter().any(|m| m == key) {
                continue;
            }
            if insert_at.is_none() {
                if let Some(p) = positions.get(key) {
                    if Self::order_index(p, rows) >= target_index {
                        insert_at = Some(rest.len());
                    }
                }
            }
            rest.push(key.clone());
        }

        let idx = insert_at.unwrap_or(rest.len());
        let movers: Vec<String> = order
            .into_iter()
            .filter(|k| moving.iter().any(|m| m == k))
            .collect();

        for (i, mover) in movers.into_iter().enumerate() {
            rest.insert(idx + i, mover);
        }

        Self::compact(&rest, rows, spans)
    }

    pub fn overflow_count(
        positions: &HashMap<String, GridPos>,
        cols: i32,
        rows: i32,
        spans: Option<&HashMap<String, Span>>,
    ) -> usize {
        positions
            .iter()
            .filter(|(key, pos)| !Self::in_grid(pos, cols, rows, Self::span_of(spans, key)))
            .count()
    }

    pub fn keep_and_place(
        positions: &HashMap<String, GridPos>,
        cols: i32,
        rows: i32,
        spans: Option<&HashMap<String, Span>>,
    ) -> HashMap<String, GridPos> {
        let mut out = HashMap::new();
        let mut outside = Vec::new();
        let mut occ = HashMap::new();

        for key in Self::ordered_keys(positions, rows) {
            if let Some(p) = positions.get(&key) {
                let s = Self::span_of(spans, &key);
                if Self::in_grid(p, cols, rows, s) && Self::rect_free(&occ, p, s) {
                    Self::mark_rect(&mut occ, p, s, &key);
                    out.insert(key, p.clone());
                } else {
                    outside.push(key);
                }
            }
        }

        for key in outside {
            if let Some(p) = positions.get(&key) {
                let s = Self::span_of(spans, &key);
                let target_c = p.col.min(cols - s.w as i32);
                let target_r = p.row.min(rows - s.h as i32);
                let cell = Self::nearest_free(&occ, target_c, target_r, cols, rows, s);
                let new_pos = GridPos {
                    col: cell.col,
                    row: cell.row,
                    screen: p.screen.clone(),
                };
                Self::mark_rect(&mut occ, &new_pos, s, &key);
                out.insert(key, new_pos);
            }
        }

        out
    }

    pub fn fit_into_grid(
        positions: &HashMap<String, GridPos>,
        cols: i32,
        rows: i32,
        spans: Option<&HashMap<String, Span>>,
    ) -> HashMap<String, GridPos> {
        let kept = Self::keep_and_place(positions, cols, rows, spans);
        let kept_over = Self::overflow_count(&kept, cols, rows, spans);
        if kept_over == 0 {
            return kept;
        }

        let packed = Self::compact(&Self::ordered_keys(positions, rows), rows, spans);
        if Self::overflow_count(&packed, cols, rows, spans) < kept_over {
            packed
        } else {
            kept
        }
    }

    pub fn sort_items(mut items: Vec<DesktopItemMeta>, sort_key: &str) -> Vec<DesktopItemMeta> {
        let rank = |it: &DesktopItemMeta| match it.kind.as_str() {
            "widget" => 0,
            "dir" | "group" => 1,
            _ => 2,
        };

        items.sort_by(|a, b| {
            let ra = rank(a);
            let rb = rank(b);
            if ra != rb {
                return ra.cmp(&rb);
            }

            match sort_key {
                "type" => {
                    let ta = a.item_type.as_deref().unwrap_or("");
                    let tb = b.item_type.as_deref().unwrap_or("");
                    ta.cmp(tb).then_with(|| a.name.cmp(&b.name))
                }
                "modified" => {
                    let ma = a.modified.unwrap_or(0);
                    let mb = b.modified.unwrap_or(0);
                    mb.cmp(&ma).then_with(|| a.name.cmp(&b.name))
                }
                "size" => {
                    let sa = a.size.unwrap_or(0);
                    let sb = b.size.unwrap_or(0);
                    sb.cmp(&sa).then_with(|| a.name.cmp(&b.name))
                }
                _ => a.name.cmp(&b.name),
            }
        });

        items
    }

    pub fn navigate(
        positions: &HashMap<String, GridPos>,
        from_key: &str,
        dir: NavDirection,
        spans: Option<&HashMap<String, Span>>,
    ) -> Option<String> {
        let from = positions.get(from_key)?;
        let fs = Self::span_of(spans, from_key);
        let fx = from.col as f64 + (fs.w as f64) / 2.0;
        let fy = from.row as f64 + (fs.h as f64) / 2.0;

        let mut best: Option<String> = None;
        let mut best_score = f64::INFINITY;

        for (key, p) in positions {
            if key == from_key {
                continue;
            }
            let s = Self::span_of(spans, key);
            let dc = (p.col as f64 + (s.w as f64) / 2.0) - fx;
            let dr = (p.row as f64 + (s.h as f64) / 2.0) - fy;

            let (primary, secondary) = match dir {
                NavDirection::Left => (-dc, dr.abs()),
                NavDirection::Right => (dc, dr.abs()),
                NavDirection::Up => (-dr, dc.abs()),
                NavDirection::Down => (dr, dc.abs()),
            };

            if primary <= 0.0 {
                continue;
            }

            let score = primary + secondary * 2.0;
            if score < best_score {
                best = Some(key.clone());
                best_score = score;
            }
        }

        best
    }

    pub fn range_between(
        positions: &HashMap<String, GridPos>,
        a: &str,
        b: &str,
        rows: i32,
    ) -> Vec<String> {
        let order = Self::ordered_keys(positions, rows);
        let ia = order.iter().position(|k| k == a);
        let ib = order.iter().position(|k| k == b);

        match (ia, ib) {
            (Some(idx_a), Some(idx_b)) => {
                let start = idx_a.min(idx_b);
                let end = idx_a.max(idx_b);
                order[start..=end].to_vec()
            }
            (None, Some(_)) => vec![b.to_string()],
            _ => Vec::new(),
        }
    }

    pub fn keys_in_rect(
        positions: &HashMap<String, GridPos>,
        rect: &PixelRect,
        cell_width: f64,
        cell_height: f64,
        inset: f64,
        spans: Option<&HashMap<String, Span>>,
    ) -> Vec<String> {
        let mut out = Vec::new();
        for (key, p) in positions {
            let s = Self::span_of(spans, key);
            let x = p.col as f64 * cell_width + inset;
            let y = p.row as f64 * cell_height + inset;
            let w = s.w as f64 * cell_width - inset * 2.0;
            let h = s.h as f64 * cell_height - inset * 2.0;

            if x < rect.x + rect.width
                && x + w > rect.x
                && y < rect.y + rect.height
                && y + h > rect.y
            {
                out.push(key.clone());
            }
        }
        out
    }

    pub fn occupant_at(
        positions: &HashMap<String, GridPos>,
        col: i32,
        row: i32,
        exclude: Option<&[String]>,
        spans: Option<&HashMap<String, Span>>,
    ) -> Option<String> {
        for (key, p) in positions {
            if let Some(ex) = exclude {
                if ex.iter().any(|k| k == key) {
                    continue;
                }
            }
            let s = Self::span_of(spans, key);
            if col >= p.col
                && col < p.col + s.w as i32
                && row >= p.row
                && row < p.row + s.h as i32
            {
                return Some(key.clone());
            }
        }
        None
    }
}
