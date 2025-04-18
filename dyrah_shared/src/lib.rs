use serde::{Deserialize, Serialize};

pub mod map;

pub use glam::{Vec2, vec2};

pub const TILE_SIZE: f32 = 32.;
pub const TILE_OFFSET: f32 = 16.;
pub const SPRITE_SIZE: f32 = 64.;

#[derive(Serialize, Deserialize, Default, Clone, Copy)]
pub struct Position {
    pub vec: Vec2,
}

impl Position {
    pub fn new(vec: Vec2) -> Self {
        Self { vec }
    }
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct TargetPosition {
    pub vec: Vec2,
    pub path: Option<Vec<Vec2>>,
}

impl TargetPosition {
    pub fn new(vec: Vec2) -> Self {
        Self { vec, path: None }
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
    PlayerConnected {
        position: Vec2,
        hp: f32,
    },
    PlayerMoved {
        id: u64,
        position: Vec2,
        path: Option<Vec<Vec2>>,
    },
    EntityDamaged {
        attacker: u64,
        defender: u64,
        hp: f32,
    },
    EntityDied {
        killer: u64,
        victim: u64,
    },
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

#[derive(Debug, Clone)]
pub enum GameEvent {
    PlayerMoved {
        id: u64,
        position: Vec2,
        path: Option<Vec<Vec2>>,
    },
    EntityDamaged {
        attacker: u64,
        defender: u64,
        hp: f32,
    },
    CreatureBatchMoved(Vec<(u64, Vec2)>),
    EntityDied {
        killer: u64,
        victim: u64,
    },
}
