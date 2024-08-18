pub mod net;
pub mod game;

use std::ops::AddAssign;

use macroquad::math::Vec2;
use renet::ClientId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
pub struct Position {
    pub x: f32,
    pub y: f32
}

impl From<Position> for Vec2 {
    fn from(pos: Position) -> Self {
        Self { x: pos.x, y: pos.y }
    }
}

impl From<Vec2> for Position {
    fn from(pos: Vec2) -> Self {
        Self { x: pos.x, y: pos.y }
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, pos: Position) {
        self.x += pos.x;
        self.y += pos.y;
    }
}

#[derive(Serialize, Deserialize)]
pub enum ClientMessages {
    PlayerCommand {
        id: EntityId
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerMessages {
    PlayerCreate {
        id: EntityId,
        pos: Position
    },
    PlayerDelete {
        id: EntityId
    },
    PlayerUpdate {
        id: EntityId,
        pos: Position
    },
    EnemyCreate {
        id: EntityId,
        pos: Position
    }
}


#[derive(Serialize, Deserialize)]
pub struct ClientInput {
    pub left: bool,
    pub up: bool,
    pub down: bool,
    pub right: bool,
    pub mouse_pos: Option<Position>
}


#[derive(Serialize, Deserialize)]
pub enum ClientChannel {
    ClientMessages,
    ClientInput
}

pub enum ServerChannel {
    ServerMessages,
    NetworkedEntities
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityId(u64);

impl EntityId {
    pub fn from_raw(id: u64) -> Self { Self(id) }
    pub fn raw(&self) -> u64 { self.0 }
}

impl From<ClientId> for EntityId {
    fn from(id: ClientId) -> Self {
        Self::from_raw(id.raw())
    }
}

impl From<EntityId> for ClientId {
    fn from(id: EntityId) -> Self {
        Self::from_raw(id.raw())
    }
}

impl From<ServerChannel> for u8 {
    fn from(channel_id: ServerChannel) -> Self {
        match channel_id {
            ServerChannel::NetworkedEntities => 0,
            ServerChannel::ServerMessages => 1
        }
    }
}

impl From<ClientChannel> for u8 {
    fn from(channel_id: ClientChannel) -> Self {
        match channel_id {
            ClientChannel::ClientInput => 0,
            ClientChannel::ClientMessages => 1
        }
    }
}