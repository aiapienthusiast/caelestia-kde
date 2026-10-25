//! ToggleRow
//! Parity with `shell/components/controls/ToggleRow.qml`.

use crate::components::controls::list_row::ListRow;
use crate::components::controls::switch::StyledSwitch;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{Div, SharedString};

pub struct ToggleRow;

impl ToggleRow {
    /// Renders a toggle row with title, description, optional leading icon, and an M3 switch.
    pub fn render(
        title: impl Into<SharedString>,
        description: Option<impl Into<SharedString>>,
        leading_icon: Option<&'static str>,
        is_checked: bool,
        is_hovered: bool,
        is_pressed: bool,
        is_disabled: bool,
        c: &CaelestiaColours,
    ) -> Div {
        let switch_widget = StyledSwitch::render(
            is_checked,
            is_hovered,
            is_pressed,
            is_disabled,
            c,
        );

        ListRow::render(
            title,
            description,
            leading_icon,
            Some(switch_widget),
            is_hovered,
            is_pressed,
            is_disabled,
            c,
        )
    }
}
