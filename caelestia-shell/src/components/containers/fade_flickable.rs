use gpui::*;

/// Flickable container with top/bottom edge alpha gradient fade (port of `VerticalFadeFlickable.qml`).
pub struct VerticalFadeFlickable {
    pub id: ElementId,
    pub fade_amount: f32,
    pub top_fade_opacity: f32,
    pub bottom_fade_opacity: f32,
    pub content_y: f32,
    pub content_height: f32,
    pub viewport_height: f32,
    pub children: Vec<Div>,
}

impl VerticalFadeFlickable {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            fade_amount: 0.2,
            top_fade_opacity: 1.0,
            bottom_fade_opacity: 1.0,
            content_y: 0.0,
            content_height: 0.0,
            viewport_height: 0.0,
            children: Vec::new(),
        }
    }

    pub fn fade_amount(mut self, amount: f32) -> Self {
        self.fade_amount = amount.clamp(0.0, 0.5);
        self
    }

    pub fn child(mut self, child: Div) -> Self {
        self.children.push(child);
        self
    }

    pub fn update_scroll(&mut self, content_y: f32, content_height: f32, viewport_height: f32) {
        self.content_y = content_y;
        self.content_height = content_height;
        self.viewport_height = viewport_height;

        let has_overflow = content_height > viewport_height;

        // Top edge fade is active when scrolled past the top (content_y > 0)
        self.top_fade_opacity = if has_overflow && content_y > 0.0 {
            0.0
        } else {
            1.0
        };

        // Bottom edge fade is active when content extends past the bottom
        let max_scroll = (content_height - viewport_height).max(0.0);
        self.bottom_fade_opacity = if has_overflow && content_y < max_scroll {
            0.0
        } else {
            1.0
        };
    }

    pub fn is_top_fade_active(&self) -> bool {
        self.top_fade_opacity < 0.5
    }

    pub fn is_bottom_fade_active(&self) -> bool {
        self.bottom_fade_opacity < 0.5
    }
}

impl IntoElement for VerticalFadeFlickable {
    type Element = Stateful<Div>;

    fn into_element(self) -> Self::Element {
        let mut container = div()
            .id(self.id)
            .w_full()
            .h_full()
            .overflow_y_scroll()
            .flex()
            .flex_col();

        for child in self.children {
            container = container.child(child);
        }

        container
    }
}
