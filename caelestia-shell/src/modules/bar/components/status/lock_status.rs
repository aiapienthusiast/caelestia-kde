//! LockStatus Indicator
//! Parity with `shell/modules/bar/components/status/LockStatus.qml`.

use crate::components::icon::{IconSize, MaterialIcon};
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, prelude::*, Div};

pub struct LockStatusWidget;

impl LockStatusWidget {
    pub fn render(caps_active: bool, num_active: bool, c: &CaelestiaColours) -> Div {
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap_1()
            .child(MaterialIcon::with_size(
                "keyboard_capslock",
                IconSize::Small,
                if caps_active { c.primary } else { c.outline },
            ))
            .child(MaterialIcon::with_size(
                "pin",
                IconSize::Small,
                if num_active { c.primary } else { c.outline },
            ))
    }
}
