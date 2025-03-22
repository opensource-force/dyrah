pub use glam::Vec2;
use serde::{Deserialize, Serialize};

pub mod map;

#[derive(Serialize, Deserialize, Default, Clone, Copy)]
pub struct Position {
    pub vec: Vec2,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { vec: Vec2 { x, y } }
    }

    pub fn from(vec: Vec2) -> Self {
        Self { vec }
    }
}

#[derive(Serialize, Deserialize, Default, Clone, Copy)]
pub struct TargetPosition {
    pub vec: Vec2,
}

impl TargetPosition {
    pub fn new(x: f32, y: f32) -> Self {
        Self { vec: Vec2 { x, y } }
    }

    pub fn from(vec: Vec2) -> Self {
        Self { vec }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Health {
    pub points: f32,
}

#[derive(Serialize, Deserialize)]
pub enum ServerMessage {
    CreatureBatchSpawned(Vec<(Vec2, f32)>),
    CreatureBatchMoved(Vec<(u64, Vec2)>),
    PlayerConnected { position: Vec2, hp: f32 },
    PlayerMoved { id: u64, position: Vec2 },
    EntityDamaged { id: u64, hp: f32 },
    EntityDied { id: u64 },
}

#[derive(Serialize, Deserialize)]
pub struct ClientInput {
    pub left: bool,
    pub up: bool,
    pub right: bool,
    pub down: bool,
    pub mouse_target_pos: Option<Vec2>,
    pub mouse_target: Option<u64>,
}

impl ClientInput {
    pub fn to_direction(&self) -> Vec2 {
        Vec2::new(
            (self.right as i8 - self.left as i8) as f32,
            (self.down as i8 - self.up as i8) as f32,
        )
    }
}

#[derive(Serialize, Deserialize)]
pub enum ClientMessage {
    PlayerUpdate { input: ClientInput },
}
