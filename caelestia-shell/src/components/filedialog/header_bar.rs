//! FileDialog top navigation header bar
//! Parity with `shell/components/filedialog/HeaderBar.qml`.

use gpui::prelude::FluentBuilder;
use gpui::*;
use std::sync::Arc;

/// Navigation header with Up button and breadcrumb path trail
#[derive(IntoElement)]
pub struct HeaderBarView {
    cwd: Vec<String>,
    on_up: Option<Arc<dyn Fn() + Send + Sync>>,
    on_segment_click: Option<Arc<dyn Fn(usize) + Send + Sync>>,
}

impl HeaderBarView {
    pub fn new(cwd: Vec<String>) -> Self {
        Self {
            cwd,
            on_up: None,
            on_segment_click: None,
        }
    }

    pub fn on_up<F: Fn() + Send + Sync + 'static>(mut self, cb: F) -> Self {
        self.on_up = Some(Arc::new(cb));
        self
    }

    pub fn on_segment_click<F: Fn(usize) + Send + Sync + 'static>(mut self, cb: F) -> Self {
        self.on_segment_click = Some(Arc::new(cb));
        self
    }
}

impl RenderOnce for HeaderBarView {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let can_go_up = self.cwd.len() > 1;
        let up_cb = self.on_up.clone();

        let mut breadcrumbs = div().flex().flex_row().items_center().gap_1();

        for (idx, segment) in self.cwd.iter().enumerate() {
            let is_last = idx == self.cwd.len() - 1;
            let seg_click = self.on_segment_click.clone();
            let seg_text = segment.clone();

            breadcrumbs = breadcrumbs.child(
                div()
                    .px_2()
                    .py_1()
                    .rounded_md()
                    .cursor_pointer()
                    .when(is_last, |d| d.bg(rgb(0x3e3c48)).text_color(rgb(0xffffff)))
                    .when(!is_last, |d| {
                        d.text_color(rgb(0xd0bcff))
                            .hover(|h| h.bg(rgb(0x2a2832)))
                    })
                    .text_sm()
                    .on_mouse_down(MouseButton::Left, move |_, _, _| {
                        if let Some(cb) = &seg_click {
                            cb(idx);
                        }
                    })
                    .child(seg_text),
            );

            if !is_last {
                breadcrumbs = breadcrumbs.child(
                    div()
                        .text_xs()
                        .text_color(rgb(0x706c7a))
                        .child("/"),
                );
            }
        }

        div()
            .flex()
            .flex_row()
            .items_center()
            .gap_3()
            .p_3()
            .bg(rgb(0x1e1c24))
            .child(
                div()
                    .w_8()
                    .h_8()
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded_md()
                    .when(can_go_up, |d| {
                        d.bg(rgb(0x2a2832))
                            .text_color(rgb(0xd0bcff))
                            .cursor_pointer()
                            .on_mouse_down(MouseButton::Left, move |_, _, _| {
                                if let Some(cb) = &up_cb {
                                    cb();
                                }
                            })
                    })
                    .when(!can_go_up, |d| {
                        d.bg(rgb(0x1e1c24)).text_color(rgb(0x504c5a))
                    })
                    .text_sm()
                    .child("↑"),
            )
            .child(
                div()
                    .flex_1()
                    .p_1()
                    .rounded_md()
                    .bg(rgb(0x25232c))
                    .child(breadcrumbs),
            )
    }
}
