//! Window Information Details
//! Parity with `shell/modules/windowinfo/Details.qml`.

use crate::infrastructure::theme::CaelestiaColours;
use crate::services::kwin::WindowItem;
use gpui::{div, prelude::*, rgba, Div, Styled};

pub struct WindowInfoDetails;

impl WindowInfoDetails {
    pub fn render(window: Option<&WindowItem>, c: &CaelestiaColours) -> Div {
        let (title, app_id, address) = if let Some(w) = window {
            (w.title.as_str(), w.app_id.as_str(), w.address.as_str())
        } else {
            ("No Window Selected", "N/A", "N/A")
        };

        div()
            .flex()
            .flex_col()
            .gap_1()
            .child(
                div()
                    .text_base()
                    .font_weight(gpui::FontWeight::BOLD)
                    .text_color(rgba(c.on_surface))
                    .child(title.to_string()),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(rgba(c.on_surface_variant))
                    .child(format!("App ID: {}", app_id)),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(rgba(c.outline))
                    .child(format!("Address: {}", address)),
            )
    }
}
