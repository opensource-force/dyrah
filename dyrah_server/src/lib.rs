use std::time::Instant;

use dyrah_shared::{Vec2, map::TILE_SIZE};

pub mod game;

pub struct Player {
    pub attacking: Option<u64>,
    pub last_attack: Instant,
}

pub struct Collider;
pub struct Creature {
    pub following: Option<u64>,
    pub last_move: Instant,
    pub last_attack: Instant,
}

pub struct PlayerView {
    position: Vec2,
    radius: f32,
}

impl Default for PlayerView {
    fn default() -> Self {
        Self {
            position: Vec2::default(),
            radius: TILE_SIZE * 20.,
        }
    }
}

impl PlayerView {
    fn contains(&self, pos: Vec2) -> bool {
        let distance =
            ((self.position.x - pos.x).powi(2) + (self.position.y - pos.y).powi(2)).sqrt();

        distance <= self.radius
    }
}
