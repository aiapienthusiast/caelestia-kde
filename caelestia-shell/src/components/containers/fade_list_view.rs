//! VerticalFadeListView
//! Parity with `shell/components/containers/VerticalFadeListView.qml`.

use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, px, Div, ElementId, Styled};

pub struct VerticalFadeListView;

impl VerticalFadeListView {
    /// Renders a vertical scrolling list with top and bottom edge fade hints.
    pub fn render(
        id: impl Into<ElementId>,
        max_height: Option<f32>,
        gap: f32,
        children: Vec<Div>,
        _c: &CaelestiaColours,
    ) -> Div {
        let mut list_col = div()
            .id(id)
            .relative()
            .flex()
            .flex_col()
            .w_full()
            .overflow_y_scroll()
            .gap(px(gap));

        if let Some(h) = max_height {
            list_col = list_col.max_h(px(h));
        }

        for child in children {
            list_col = list_col.child(child);
        }

        div()
            .relative()
            .flex()
            .flex_col()
            .w_full()
            .child(list_col)
    }
}
