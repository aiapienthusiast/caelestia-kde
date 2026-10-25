//! Screen Corners Edge Trigger Coordinator
//! Parity with `shell/modules/ScreenCorners.qml`.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScreenCorner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Debug, Clone, Default)]
pub struct ScreenCornersConfig {
    pub overview_enabled: bool,
    pub hover_top_left: bool,
    pub hover_top_right: bool,
    pub hover_bottom_left: bool,
    pub hover_bottom_right: bool,
}

#[derive(Debug, Clone, Default)]
pub struct ScreenCornersCoordinator {
    claimed_corners: Vec<ScreenCorner>,
}

impl ScreenCornersCoordinator {
    pub fn new() -> Self {
        Self {
            claimed_corners: Vec::new(),
        }
    }

    /// Computes the active claimed screen corners based on overview configuration.
    pub fn sync(&mut self, cfg: &ScreenCornersConfig) -> Vec<ScreenCorner> {
        let mut wanted = Vec::new();

        if cfg.overview_enabled {
            if cfg.hover_top_left {
                wanted.push(ScreenCorner::TopLeft);
            }
            if cfg.hover_top_right {
                wanted.push(ScreenCorner::TopRight);
            }
            if cfg.hover_bottom_left {
                wanted.push(ScreenCorner::BottomLeft);
            }
            if cfg.hover_bottom_right {
                wanted.push(ScreenCorner::BottomRight);
            }
        }

        self.claimed_corners = wanted.clone();
        wanted
    }

    pub fn is_claimed(&self, corner: ScreenCorner) -> bool {
        self.claimed_corners.contains(&corner)
    }
}
