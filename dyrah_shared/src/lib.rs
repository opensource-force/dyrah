use serde::{Deserialize, Serialize};

pub mod map;

#[derive(Serialize, Deserialize)]
pub struct Position {
    pub x: f32,
    pub y: f32
}

#[derive(Serialize, Deserialize)]
pub enum ServerMessage {
    PlayerConnected { id: u64, pos: Position },
    PlayerMoved { id: u64, pos: Position }
}

#[derive(Serialize, Deserialize)]
pub enum ClientMessage {
    PlayerMove { left: bool, up: bool, right: bool, down: bool }
}