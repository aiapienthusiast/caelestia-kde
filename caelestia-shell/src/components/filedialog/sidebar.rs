//! FileDialog sidebar with standard XDG places bookmarks
//! Parity with `shell/components/filedialog/Sidebar.qml`.

use super::sizes::SIDEBAR_WIDTH;
use gpui::prelude::FluentBuilder;
use gpui::*;
use std::sync::Arc;

pub const STANDARD_PLACES: &[(&str, &str)] = &[
    ("Home", "home"),
    ("Downloads", "file_download"),
    ("Desktop", "desktop_windows"),
    ("Documents", "description"),
    ("Music", "music_note"),
    ("Pictures", "image"),
    ("Videos", "video_library"),
];

/// Sidebar bookmark list of common directories
#[derive(IntoElement)]
pub struct SidebarView {
    current_place: String,
    on_place_selected: Option<Arc<dyn Fn(&'static str) + Send + Sync>>,
}

impl SidebarView {
    pub fn new(current_place: impl Into<String>) -> Self {
        Self {
            current_place: current_place.into(),
            on_place_selected: None,
        }
    }

    pub fn on_place_selected<F: Fn(&'static str) + Send + Sync + 'static>(
        mut self,
        cb: F,
    ) -> Self {
        self.on_place_selected = Some(Arc::new(cb));
        self
    }
}

impl RenderOnce for SidebarView {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut places_col = div().flex().flex_col().gap_1();

        for &(place, _icon) in STANDARD_PLACES {
            let is_selected = self.current_place == place;
            let cb = self.on_place_selected.clone();

            places_col = places_col.child(
                div()
                    .px_3()
                    .py_2()
                    .rounded_full()
                    .cursor_pointer()
                    .when(is_selected, |d| {
                        d.bg(rgb(0x4a4458)).text_color(rgb(0xe8def8))
                    })
                    .when(!is_selected, |d| {
                        d.text_color(rgb(0xe6e1e5))
                            .hover(|h| h.bg(rgb(0x2a2832)))
                    })
                    .text_sm()
                    .on_mouse_down(MouseButton::Left, move |_, _, _| {
                        if let Some(handler) = &cb {
                            handler(place);
                        }
                    })
                    .child(place),
            );
        }

        div()
            .w(px(SIDEBAR_WIDTH))
            .flex()
            .flex_col()
            .p_3()
            .bg(rgb(0x1c1b20))
            .child(
                div()
                    .pb_3()
                    .text_base()
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0xffffff))
                    .child("Files"),
            )
            .child(places_col)
    }
}
