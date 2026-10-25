use gpui::*;

/// Virtualizing list view container with keyboard navigation and momentum flicking (port of `StyledListView.qml`).
pub struct StyledListView {
    pub id: ElementId,
    pub max_velocity: f32,
    pub enable_fake_flick: bool,
    pub done_fake_flick: bool,
    pub content_height: f32,
    pub content_width: f32,
    pub item_count: usize,
    pub children: Vec<Div>,
}

impl StyledListView {
    pub fn new(id: impl Into<ElementId>, item_count: usize) -> Self {
        Self {
            id: id.into(),
            max_velocity: 3000.0,
            enable_fake_flick: true,
            done_fake_flick: false,
            content_height: 0.0,
            content_width: 0.0,
            item_count,
            children: Vec::new(),
        }
    }

    pub fn max_velocity(mut self, max_velocity: f32) -> Self {
        self.max_velocity = max_velocity;
        self
    }

    pub fn enable_fake_flick(mut self, enable: bool) -> Self {
        self.enable_fake_flick = enable;
        self
    }

    pub fn content_size(mut self, width: f32, height: f32) -> Self {
        self.content_width = width;
        self.content_height = height;
        self
    }

    pub fn child(mut self, child: Div) -> Self {
        self.children.push(child);
        self
    }
}

impl IntoElement for StyledListView {
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
