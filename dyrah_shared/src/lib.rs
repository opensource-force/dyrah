use serde::{Deserialize, Serialize};

pub mod map;

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
    PlayerConnected {
        id: u64,
        position: Position,
    },
    PlayerMoved {
        id: u64,
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
