use std::time::Instant;

use serde::{Deserialize, Serialize};

pub mod map;

pub struct Player;
pub struct Creature {
    pub last_move: Instant,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct TargetPosition {
    pub x: f32,
    pub y: f32,
}

#[derive(Serialize, Deserialize)]
pub enum ServerMessage {
    CreatureSpawned {
        position: Position,
    },
    CreatureMoved {
        id: u64,
        target_position: TargetPosition,
    },
    PlayerConnected {
        position: Position,
    },
    PlayerMoved {
        target_position: TargetPosition,
    },
}

#[derive(Serialize, Deserialize)]
pub enum ClientMessage {
    PlayerMove {
        left: bool,
        up: bool,
        right: bool,
        down: bool,
    },
}
