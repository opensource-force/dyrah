use serde::{Deserialize, Serialize};

pub mod map;

pub struct Player;
pub struct Creature;

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
    CreatureSpawned { position: Position },
    CreatureMoved { position: Position },
    PlayerConnected { position: Position },
    PlayerMoved { target_position: TargetPosition },
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
