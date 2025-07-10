use glam::{IVec2, Vec2};
use serde::{Deserialize, Serialize};

use crate::NetId;

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerMessage {
    PlayerSpawned { id: NetId, position: Vec2 },
    PlayerMoved { id: NetId, position: Vec2 },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientMessage {
    PlayerUpdate { input: ClientInput },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientInput {
    pub left: bool,
    pub up: bool,
    pub right: bool,
    pub down: bool,
    pub mouse_tile_pos: Option<IVec2>,
}

impl ClientInput {
    pub fn to_direction(&self) -> IVec2 {
        IVec2::new(
            (self.right as i32) - (self.left as i32),
            (self.down as i32) - (self.up as i32),
        )
    }
}
