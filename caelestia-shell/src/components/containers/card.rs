//! StyledCard
//! Parity with `shell/utils/Surface.qml` and M3 Card specifications.

use crate::domain::tokens::*;
use crate::infrastructure::theme::CaelestiaColours;
use gpui::{div, px, rgba, Div, Styled};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CardType {
    Elevated,
    Filled,
    Outlined,
}

pub struct StyledCard;

impl StyledCard {
    /// Renders an M3 Card container.
    pub fn render(
        card_type: CardType,
        radius_px: Option<f32>,
        c: &CaelestiaColours,
    ) -> Div {
        let r = radius_px.unwrap_or(ROUNDING_NORMAL);

        let bg_color = match card_type {
            CardType::Elevated => c.surface_container_low,
            CardType::Filled => c.surface_container_highest,
            CardType::Outlined => c.surface,
        };

        let mut card = div()
            .relative()
            .rounded(px(r))
            .bg(rgba(bg_color))
            .p(px(PADDING_NORMAL));

        match card_type {
            CardType::Elevated => {
                card = card.shadow_md();
            }
            CardType::Outlined => {
                card = card.border_1().border_color(rgba(c.outline_variant));
            }
            CardType::Filled => {}
        }

        card
    }
}
