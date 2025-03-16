use std::time::Instant;

use dyrah_shared::{Position, map::TILE_SIZE};

pub mod game;

pub struct Collider;
pub struct Creature {
    pub last_move: Instant,
}

pub struct PlayerView {
    position: Position,
    radius: f32,
}

impl Default for PlayerView {
    fn default() -> Self {
        Self {
            position: Position::default(),
            radius: TILE_SIZE * 20.,
        }
    }
}

impl PlayerView {
    fn contains(&self, x: f32, y: f32) -> bool {
        let distance = ((self.position.x - x).powi(2) + (self.position.y - y).powi(2)).sqrt();

        distance <= self.radius
    }
}
