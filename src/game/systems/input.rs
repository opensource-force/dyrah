use super::*;
use crate::game::Player;

pub struct InputSystem;

impl InputSystem {
    pub fn control_player(mut player: UniqueViewMut<Player>) {
        player.vel.0 = if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
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

        if player.vel.0 != Vec2::ZERO {
            player.target_pos.0 = player.pos.0 + player.vel.0 * TILE_SIZE;
        }
    }
}