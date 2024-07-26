use macroquad::{input::{is_key_down, KeyCode}, math::{vec2, Vec2}};
use shipyard::{UniqueViewMut, ViewMut};

use crate::{Player, Velocity};

pub struct InputSystem;

impl InputSystem {
    pub fn control_player(
        player: UniqueViewMut<Player>,
        mut velocities: ViewMut<Velocity>
    ) {
        let player_vel = &mut velocities[player.0];
        
        player_vel.0 = if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            vec2(0.0, -1.0)
        } else if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            vec2(-1.0, 0.0)
        } else if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            vec2(0.0, 1.0)
        } else if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            vec2(1.0, 0.0)
        } else {
            Vec2::ZERO
        };
    }
}