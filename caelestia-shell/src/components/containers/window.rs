use gpui::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum WindowLayerNamespace {
    #[default]
    Panel,
    Desktop,
    Overlay,
}

/// Base layer-shell window frame with screen attachment and namespace routing (port of `StyledWindow.qml`).
pub struct StyledWindow {
    pub name: String,
    pub screen_name: String,
    pub is_desktop_widget: bool,
    pub namespace: WindowLayerNamespace,
    pub children: Vec<Div>,
}

impl StyledWindow {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            screen_name: String::new(),
            is_desktop_widget: false,
            namespace: WindowLayerNamespace::Panel,
            children: Vec::new(),
        }
    }

    pub fn screen_name(mut self, screen: impl Into<String>) -> Self {
        self.screen_name = screen.into();
        self
    }

    pub fn desktop_widget(mut self, is_desktop: bool) -> Self {
        self.is_desktop_widget = is_desktop;
        self.namespace = if is_desktop {
            WindowLayerNamespace::Desktop
        } else {
            WindowLayerNamespace::Panel
        };
        self
    }

    pub fn namespace(mut self, ns: WindowLayerNamespace) -> Self {
        self.namespace = ns;
        self
    }

    pub fn child(mut self, child: Div) -> Self {
        self.children.push(child);
        self
    }
}

impl IntoElement for StyledWindow {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        let mut window = div()
            .w_full()
            .h_full()
            .bg(gpui::transparent_black());

        for child in self.children {
            window = window.child(child);
        }

        window
    }
}
