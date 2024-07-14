mod systems;
mod game;
mod map;

pub mod prelude {
    pub use super::game::*;
    pub use super::map::*;
}

use macroquad::prelude::*;
use shipyard::*;

#[derive(Unique)]
pub struct Camera(Camera2D);

#[derive(Component)]
pub struct Position(Vec2);
#[derive(Component)]
pub struct Velocity(Vec2);
#[derive(Component)]
pub struct Sprite {
    tex: Texture2D,
    frame: IVec2
}

#[derive(Unique)]
pub struct Player {
    pos: Position,
    vel: Velocity,
    spr: Sprite
}

#[derive(Component)]
pub struct Monster;