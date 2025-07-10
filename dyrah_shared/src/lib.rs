pub mod components;
pub mod messages;

use glam::{IVec2, Vec2};

pub const TILE_SIZE: f32 = 32.0;

pub type NetId = u32;

pub fn tile_to_world(tile: IVec2) -> Vec2 {
    Vec2::new(tile.x as f32, tile.y as f32) * TILE_SIZE
}

pub fn world_to_tile(world: Vec2) -> IVec2 {
    IVec2::new(
        (world.x / TILE_SIZE).floor() as i32,
        (world.y / TILE_SIZE).floor() as i32,
    )
}
