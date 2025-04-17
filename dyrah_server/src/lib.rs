use std::time::Instant;

use dyrah_shared::{TILE_SIZE, Vec2};

pub mod game;
pub mod map;
mod systems;

pub struct Player;
pub struct Creature;
pub struct Collider;

pub struct State {
    pub attacking: Option<u64>,
    pub following: Option<u64>,
    pub last_attack: Instant,
    pub last_move: Instant,
}

impl State {
    fn new() -> Self {
        Self {
            attacking: None,
            following: None,
            last_attack: Instant::now(),
            last_move: Instant::now(),
        }
    }
}

pub struct PlayerView {
    position: Vec2,
    radius: f32,
}

impl PlayerView {
    fn new(pos: Vec2) -> Self {
        Self {
            position: pos,
            radius: TILE_SIZE * 10.,
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
