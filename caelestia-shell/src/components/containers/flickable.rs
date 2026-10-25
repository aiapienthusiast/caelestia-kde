//! Flickable
//! Parity with `shell/components/containers/StyledFlickable.qml`.

use gpui::{div, prelude::*, px, Div, ElementId, Stateful, Styled};

pub struct StyledFlickable;

impl StyledFlickable {
    /// Renders a scrollable viewport container that clips overflowing content
    /// and allows smooth scrolling.
    pub fn render(
        id: impl Into<ElementId>,
        max_height: Option<f32>,
        children: Vec<Div>,
    ) -> Stateful<Div> {
        let mut container = div()
            .id(id)
            .relative()
            .flex()
            .flex_col()
            .w_full()
            .overflow_y_scroll();

        if let Some(h) = max_height {
            container = container.max_h(px(h));
        }

        for child in children {
            container = container.child(child);
        }

        container
    }
}
