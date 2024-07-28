mod camera;
mod server;
mod map;
mod systems;
mod client;

pub mod prelude {
    pub use super::camera::*;
    pub use super::server::*;
    pub use super::map::*;
    pub use super::client::Client;
}

use std::fmt;
use macroquad::prelude::*;
use shipyard::*;
use systems::prelude::*;

pub struct WorldTime(f64);

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
pub struct Dead;

#[derive(Unique)]
pub struct Player(EntityId);

#[derive(Component)]
pub struct Monster;

pub struct Workloads;

impl Workloads {
    pub fn events() -> Workload {
        (InputSystem::control_player, AiSystem::control_monsters).into_workload()
    }

    pub fn update() -> Workload {
        (
            MovementSystem::update,
            DamageSystem::attack_target,
            RemovalSystem::remove_dead,
        )
            .into_workload()
    }

    pub fn draw() -> Workload {
        (
            |_: AllStoragesViewMut| clear_background(SKYBLUE),
            RenderSystem::draw_camera,
            RenderSystem::draw_map,
            RenderSystem::draw_entities,
            RenderSystem::draw_player_target,
            RenderSystem::debug,
        )
            .into_sequential_workload()
    }
}


impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // The `f` value implements the `Write` trait, which is what the
        // write! macro is expecting. Note that this formatting ignores the
        // various flags provided to format strings.
        write!(f, "({}, {})", self.0.x, self.0.y)
    }
}

impl fmt::Display for Velocity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // The `f` value implements the `Write` trait, which is what the
        // write! macro is expecting. Note that this formatting ignores the
        // various flags provided to format strings.
        write!(f, "({}, {})", self.0.x, self.0.y)
    }
}

impl fmt::Display for Health {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // The `f` value implements the `Write` trait, which is what the
        // write! macro is expecting. Note that this formatting ignores the
        // various flags provided to format strings.
        write!(f, "({})", self.0)
    }
}
