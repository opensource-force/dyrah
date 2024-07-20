mod game;
mod map;
mod camera;
mod systems;

pub mod prelude {
    pub use super::game::*;
    pub use super::map::*;
    pub use super::camera::*;
}

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
pub struct Dead(());

#[derive(Unique)]
pub struct Player(EntityId);

#[derive(Component)]
pub struct Monster;

pub struct Workloads;

impl Workloads {
    pub fn events() -> Workload {
        (
            InputSystem::control_player,
            AiSystem::control_monsters
        ).into_workload()
    }
    
    pub fn update() -> Workload {
        (
            MovementSystem::update,
            DamageSystem::attack_target,
            RemovalSystem::remove_dead
        ).into_workload()
    }
    
    pub fn draw() -> Workload {
        (
            |_: AllStoragesViewMut| clear_background(SKYBLUE),
            RenderSystem::draw_map,
            RenderSystem::draw_entities,
            RenderSystem::draw_camera,
            RenderSystem::debug,
            RenderSystem::draw_player_target
        ).into_workload()
    }
}
