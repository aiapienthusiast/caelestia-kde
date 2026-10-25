//! BluetoothStatus Indicator
//! Parity with `shell/modules/bar/components/status/BluetoothStatus.qml`.

use crate::components::styled_text::StyledTextExt;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, rgba, Div, Styled};

pub struct BluetoothStatusWidget;

impl BluetoothStatusWidget {
    pub fn render(icon: &str, connected_count: usize, c: &CaelestiaColours) -> Div {
        let mut row = div()
            .flex()
            .flex_row()
            .items_center()
            .gap_1()
            .child(StyledTextExt::icon_size(div().child(icon.to_string()), 16.0).text_color(rgba(c.on_surface)));

        if connected_count > 1 {
            row = row.child(
                div()
                    .text_xs()
                    .text_color(rgba(c.on_surface))
                    .child(connected_count.to_string()),
            );
        }

        row
    }
}
