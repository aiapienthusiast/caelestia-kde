//! FileDialog bottom action buttons and filter bar
//! Parity with `shell/components/filedialog/DialogButtons.qml`.

use gpui::prelude::FluentBuilder;
use gpui::*;
use std::sync::Arc;

/// Bottom action bar displaying active filter and Select/Cancel buttons
#[derive(IntoElement)]
pub struct DialogButtonsView {
    filter_label: String,
    filters: Vec<String>,
    select_folder: bool,
    selection_valid: bool,
    on_select: Option<Arc<dyn Fn() + Send + Sync>>,
    on_cancel: Option<Arc<dyn Fn() + Send + Sync>>,
}

impl DialogButtonsView {
    pub fn new(
        filter_label: impl Into<String>,
        filters: Vec<String>,
        select_folder: bool,
        selection_valid: bool,
    ) -> Self {
        Self {
            filter_label: filter_label.into(),
            filters,
            select_folder,
            selection_valid,
            on_select: None,
            on_cancel: None,
        }
    }

    pub fn on_select<F: Fn() + Send + Sync + 'static>(mut self, cb: F) -> Self {
        self.on_select = Some(Arc::new(cb));
        self
    }

    pub fn on_cancel<F: Fn() + Send + Sync + 'static>(mut self, cb: F) -> Self {
        self.on_cancel = Some(Arc::new(cb));
        self
    }

    pub fn filter_display_text(&self) -> String {
        if self.select_folder {
            "Folders".to_string()
        } else if self.filters.contains(&"*".to_string()) || self.filters.is_empty() {
            format!("{} (*)", self.filter_label)
        } else {
            let extensions = self
                .filters
                .iter()
                .map(|f| format!("*.{}", f))
                .collect::<Vec<_>>()
                .join(", ");
            format!("{} ({})", self.filter_label, extensions)
        }
    }
}

impl RenderOnce for DialogButtonsView {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let filter_text = self.filter_display_text();
        let select_text = if self.select_folder {
            "Select Folder"
        } else {
            "Select"
        };
        let is_valid = self.selection_valid;
        let select_cb = self.on_select.clone();
        let cancel_cb = self.on_cancel.clone();

        div()
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .p_3()
            .bg(rgb(0x1e1c24))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap_2()
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0xa09cb0))
                            .child("Filter:"),
                    )
                    .child(
                        div()
                            .px_3()
                            .py_1()
                            .rounded_md()
                            .bg(rgb(0x2a2832))
                            .text_sm()
                            .text_color(rgb(0xffffff))
                            .child(filter_text),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap_2()
                    .child(
                        div()
                            .px_4()
                            .py_1()
                            .rounded_full()
                            .bg(if is_valid { rgb(0xd0bcff) } else { rgb(0x3e3c46) })
                            .text_color(if is_valid { rgb(0x381e72) } else { rgb(0x706c7a) })
                            .text_sm()
                            .font_weight(FontWeight::SEMIBOLD)
                            .cursor_pointer()
                            .when(is_valid, |d| {
                                d.on_mouse_down(MouseButton::Left, move |_, _, _| {
                                    if let Some(cb) = &select_cb {
                                        cb();
                                    }
                                })
                            })
                            .child(select_text),
                    )
                    .child(
                        div()
                            .px_4()
                            .py_1()
                            .rounded_full()
                            .bg(rgb(0x2a2832))
                            .text_color(rgb(0xd0bcff))
                            .text_sm()
                            .cursor_pointer()
                            .on_mouse_down(MouseButton::Left, move |_, _, _| {
                                if let Some(cb) = &cancel_cb {
                                    cb();
                                }
                            })
                            .child("Cancel"),
                    ),
            )
    }
}
