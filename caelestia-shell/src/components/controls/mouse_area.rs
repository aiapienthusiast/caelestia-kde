/// Extended mouse interaction handler with notched scroll wheel accumulation (port of `CustomMouseArea.qml`).
#[derive(Clone)]
pub struct CustomMouseArea {
    pub scroll_accumulated_y: i32,
    pub threshold: i32,
}

impl CustomMouseArea {
    pub const STANDARD_NOTCH: i32 = 120;

    pub fn new() -> Self {
        Self {
            scroll_accumulated_y: 0,
            threshold: Self::STANDARD_NOTCH,
        }
    }

    pub fn threshold(mut self, threshold: i32) -> Self {
        self.threshold = threshold;
        self
    }

    /// Feeds a scroll wheel delta Y and returns the notched count if the threshold was crossed.
    pub fn handle_wheel_delta(&mut self, delta_y: i32) -> Option<i32> {
        if self.scroll_accumulated_y != 0 && delta_y.signum() != self.scroll_accumulated_y.signum() {
            self.scroll_accumulated_y = 0;
        }

        self.scroll_accumulated_y += delta_y;

        if self.scroll_accumulated_y.abs() >= self.threshold {
            let notches = self.scroll_accumulated_y / self.threshold;
            self.scroll_accumulated_y = 0;
            Some(notches)
        } else {
            None
        }
    }
}

impl Default for CustomMouseArea {
    fn default() -> Self {
        Self::new()
    }
}
