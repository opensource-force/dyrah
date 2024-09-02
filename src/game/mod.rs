use std::{collections::HashMap, ops::{AddAssign, Div, Mul}};

use macroquad::{color::Color, math::{Rect, Vec2}, shapes::draw_rectangle_lines, window::{screen_height, screen_width}};
use map::TILE_SIZE;
use renet::ClientId;
use serde::{Deserialize, Serialize};
use thunderdome::Index;

pub mod server;
pub mod client;
mod world;
mod map;
mod camera;

#[derive(Default)]
pub struct Lobby {
    players: HashMap<EntityId, Index>,
    enemies: HashMap<EntityId, Index>
}

#[derive(Default, Clone, Copy, Serialize, Deserialize, Debug)]
pub struct Sprite {
    pub frame: Vec2D
}

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

impl Vec2D {
    pub fn new_rect(&self, size: Vec2) -> Rect {
        Rect::new(self.x, self.y, size.x, size.y)
    }

    pub fn draw_rect(&self, size: Vec2, color: Color) {
        draw_rectangle_lines(self.x, self.y, size.x, size.y, 2.0, color);
    }

    pub fn screen_rect(&self) -> Rect {
        Rect::new(
            self.x - screen_width() / 2.0 - TILE_SIZE.x * 2.0,
            self.y - screen_height() / 2.0 - TILE_SIZE.y * 2.0,
            screen_width() + TILE_SIZE.x * 2.0, screen_height() + TILE_SIZE.y * 2.0,
        )
    }
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