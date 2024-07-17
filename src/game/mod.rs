mod game;
mod map;
mod systems;

pub mod prelude {
    pub use super::game::*;
    pub use super::map::*;
}

use macroquad::prelude::*;
use shipyard::*;

pub struct WorldTime(f64);

#[derive(Unique)]
pub struct Camera(Camera2D);

#[derive(Component)]
pub struct Position(Vec2);
#[derive(Component)]
pub struct Velocity(Vec2);
#[derive(Component)]
pub struct Sprite {
    tex: Texture2D,
    frame: IVec2,
}
#[derive(Component)]
pub struct Moving(bool);
#[derive(Component)]
pub struct TargetPosition(Vec2);
#[derive(Component)]
pub struct Target(EntityId);
#[derive(Component)]
pub struct Health(f32);
#[derive(Component)]
pub struct Damage(f32);
#[derive(Component)]
pub struct Dead(Option<EntityId>);

#[derive(Unique)]
pub struct Player(EntityId);

#[derive(Component)]
pub struct Monster;
