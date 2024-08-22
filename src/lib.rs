pub mod net;
pub mod game;

use std::ops::{AddAssign, Div, Mul};

use macroquad::{color::Color, math::Vec2, shapes::draw_rectangle_lines};
use renet::ClientId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
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

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
pub struct Vec2D {
    x: f32,
    y: f32
}

impl From<Vec2D> for Vec2 {
    fn from(vec: Vec2D) -> Self {
        Self { x: vec.x, y: vec.y }
    }
}

impl From<Vec2> for Vec2D {
    fn from(vec: Vec2) -> Self {
        Self { x: vec.x, y: vec.y }
    }
}

impl Vec2D {
    fn draw_rect(&self, size: Vec2, color: Color) {
        draw_rectangle_lines(self.x, self.y, size.x, size.y, 2.0, color);
    }
}

impl AddAssign for Vec2D {
    fn add_assign(&mut self, vec: Vec2D) {
        self.x += vec.x;
        self.y += vec.y;
    }
}

impl Mul<Vec2D> for Vec2D {
    type Output = Vec2D;

    fn mul(self, other: Vec2D) -> Vec2D {
        Vec2D {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl Div<Vec2D> for Vec2D {
    type Output = Self;

    fn div(self, other: Vec2D) -> Self::Output {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ClientInput {
    pub left: bool,
    pub up: bool,
    pub down: bool,
    pub right: bool,
    pub mouse_target_pos: Option<Vec2D>,
    pub mouse_target: Option<EntityId>
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
        pos: Vec2D
    },
    PlayerDelete {
        id: EntityId
    },
    PlayerUpdate {
        id: EntityId,
        pos: Vec2D,
        target: EntityId
    },
    EnemyCreate {
        id: EntityId,
        pos: Vec2D
    }
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

impl From<ClientChannel> for u8 {
    fn from(channel_id: ClientChannel) -> Self {
        match channel_id {
            ClientChannel::ClientInput => 0,
            ClientChannel::ClientMessages => 1
        }
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