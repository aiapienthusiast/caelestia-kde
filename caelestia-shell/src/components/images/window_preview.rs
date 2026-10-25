//! PipeWire live window screencast preview component
//! Parity with `shell/components/images/WindowPreview.qml`.

use gpui::prelude::FluentBuilder;
use gpui::*;

/// Window screencast stream receiver metadata
#[derive(Clone, Debug)]
pub struct WindowStreamState {
    pub address: String,
    pub active: bool,
    pub stream_available: bool,
    pub node_id: Option<u32>,
    pub object_serial: Option<u64>,
    pub fallback_icon: String,
    pub source_aspect: f32,
}

impl Default for WindowStreamState {
    fn default() -> Self {
        Self {
            address: String::new(),
            active: true,
            stream_available: false,
            node_id: None,
            object_serial: None,
            fallback_icon: "application-x-executable".to_string(),
            source_aspect: 16.0 / 9.0,
        }
    }
}

impl WindowStreamState {
    pub fn new(address: impl Into<String>) -> Self {
        Self {
            address: address.into(),
            ..Self::default()
        }
    }

    pub fn with_fallback_icon(mut self, icon: impl Into<String>) -> Self {
        self.fallback_icon = icon.into();
        self
    }

    pub fn with_stream(mut self, node_id: u32, serial: u64) -> Self {
        self.node_id = Some(node_id);
        self.object_serial = Some(serial);
        self.stream_available = true;
        self
    }

    pub fn fitted_dimensions(&self, width: f32, height: f32) -> (f32, f32) {
        let safe_height = height.max(1.0);
        let current_aspect = width / safe_height;

        if self.source_aspect > current_aspect {
            let fitted_height = width / self.source_aspect;
            (width, fitted_height)
        } else {
            let fitted_width = height * self.source_aspect;
            (fitted_width, height)
        }
    }
}

/// Element rendering a window preview stream or fallback icon
#[derive(IntoElement)]
pub struct WindowPreviewView {
    state: WindowStreamState,
}

impl WindowPreviewView {
    pub fn new(state: WindowStreamState) -> Self {
        Self { state }
    }
}

impl RenderOnce for WindowPreviewView {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let has_stream = self.state.stream_available;
        let addr = self.state.address.clone();
        let fallback = self.state.fallback_icon.clone();

        div()
            .w_full()
            .h_full()
            .flex()
            .items_center()
            .justify_center()
            .overflow_hidden()
            .bg(rgb(0x18171d))
            .when(has_stream, |d| {
                d.child(
                    div()
                        .text_xs()
                        .text_color(rgb(0xb0c4de))
                        .child(format!("📺 Live Stream [PipeWire #{:?}] ({})", self.state.node_id, addr)),
                )
            })
            .when(!has_stream, |d| {
                d.child(
                    div()
                        .text_xs()
                        .text_color(rgb(0x706c7a))
                        .child(format!("🔲 Fallback [{}] ({})", fallback, addr)),
                )
            })
    }
}
