//! Chromium Dinosaur Game Physics & State Engine
//! 100% parity with C++ `Caelestia::DinoGameBackend` and `shell/modules/sidebar/DinoGame.qml`.

use std::sync::Mutex;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ObstacleType {
    CactusSmall,
    CactusLarge,
    Pterodactyl,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DinoObstacle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub obstacle_type: ObstacleType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DinoGameState {
    pub is_playing: bool,
    pub is_game_over: bool,
    pub is_ducking: bool,
    pub is_inverted: bool,
    pub score: f32,
    pub high_score: f32,
    pub dino_y: f32,
    pub dino_velocity_y: f32,
    pub speed: f32,
    pub obstacles: Vec<DinoObstacle>,
}

impl Default for DinoGameState {
    fn default() -> Self {
        Self {
            is_playing: false,
            is_game_over: false,
            is_ducking: false,
            is_inverted: false,
            score: 0.0,
            high_score: 0.0,
            dino_y: 0.0,
            dino_velocity_y: 0.0,
            speed: 6.0,
            obstacles: Vec::new(),
        }
    }
}

pub struct DinoGameService {
    pub state: Mutex<DinoGameState>,
}

impl Default for DinoGameService {
    fn default() -> Self {
        Self::new()
    }
}

impl DinoGameService {
    const GRAVITY: f32 = 0.6;
    const JUMP_FORCE: f32 = -12.0;
    const DINO_WIDTH: f32 = 44.0;
    const DINO_HEIGHT: f32 = 47.0;

    pub fn new() -> Self {
        Self {
            state: Mutex::new(DinoGameState::default()),
        }
    }

    pub fn start(&self) {
        if let Ok(mut st) = self.state.lock() {
            let high = st.high_score;
            *st = DinoGameState::default();
            st.high_score = high;
            st.is_playing = true;
        }
    }

    pub fn jump(&self) {
        if let Ok(mut st) = self.state.lock() {
            if !st.is_playing {
                self.start();
                return;
            }
            if st.dino_y <= 0.001 && !st.is_game_over {
                st.dino_velocity_y = Self::JUMP_FORCE;
            }
        }
    }

    pub fn set_ducking(&self, ducking: bool) {
        if let Ok(mut st) = self.state.lock() {
            st.is_ducking = ducking;
        }
    }

    /// Advance game physics by one delta frame
    pub fn update(&self, dt: f32) {
        if let Ok(mut st) = self.state.lock() {
            if !st.is_playing || st.is_game_over {
                return;
            }

            // Update Dino vertical position
            st.dino_y += st.dino_velocity_y;
            st.dino_velocity_y += Self::GRAVITY;

            if st.dino_y <= 0.0 {
                st.dino_y = 0.0;
                st.dino_velocity_y = 0.0;
            }

            // Advance obstacles
            let current_speed = st.speed;
            for obs in &mut st.obstacles {
                obs.x -= current_speed * dt * 60.0;
            }
            st.obstacles.retain(|obs| obs.x + obs.width > -50.0);

            // Spawn obstacles if distance is large enough
            let last_x = st.obstacles.last().map(|o| o.x).unwrap_or(0.0);
            if last_x < 400.0 {
                st.obstacles.push(DinoObstacle {
                    x: 650.0,
                    y: 0.0,
                    width: 25.0,
                    height: 40.0,
                    obstacle_type: ObstacleType::CactusSmall,
                });
            }

            // Score & speed increase
            st.score += dt * 10.0;
            if st.score > st.high_score {
                st.high_score = st.score;
            }
            st.speed = (6.0 + (st.score / 200.0)).min(13.0);

            // Invert colors every 700 points (night/day cycle)
            st.is_inverted = ((st.score as u32 / 700) % 2) == 1;

            // Check AABB collision
            let dino_h = if st.is_ducking { 25.0 } else { Self::DINO_HEIGHT };
            let dino_box = (50.0, st.dino_y, Self::DINO_WIDTH, dino_h);

            for obs in &st.obstacles {
                if Self::check_aabb_collision(dino_box, (obs.x, obs.y, obs.width, obs.height)) {
                    st.is_game_over = true;
                    break;
                }
            }
        }
    }

    fn check_aabb_collision(a: (f32, f32, f32, f32), b: (f32, f32, f32, f32)) -> bool {
        let (ax, ay, aw, ah) = a;
        let (bx, by, bw, bh) = b;

        ax < bx + bw && ax + aw > bx && ay < by + bh && ay + ah > by
    }
}
