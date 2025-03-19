use serde::{Deserialize, Serialize};

pub mod map;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Default)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct TargetPosition {
    pub x: f32,
    pub y: f32,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Health {
    pub points: f32,
}

#[derive(Serialize, Deserialize)]
pub enum ServerMessage {
    CreatureBatchSpawned(Vec<(Position, Health)>),
    CreatureBatchMoved(Vec<(u64, Position)>),
    PlayerConnected { position: Position, health: Health },
    PlayerMoved { id: u64, position: Position },
    CreatureDamaged { id: u64, health: Health },
    EntityDied { id: u64 },
}

#[derive(Serialize, Deserialize)]
pub struct ClientInput {
    pub left: bool,
    pub up: bool,
    pub right: bool,
    pub down: bool,
    pub mouse_target_pos: Option<Position>,
    pub mouse_target: Option<u64>,
}

impl ClientInput {
    pub fn to_direction(&self) -> (f32, f32) {
        (
            (self.right as i8 - self.left as i8) as f32,
            (self.down as i8 - self.up as i8) as f32,
        )
    }
}

#[derive(Serialize, Deserialize)]
pub enum ClientMessage {
    PlayerUpdate { input: ClientInput },
}
