use super::*;
use crate::game::{Camera, Player, Position};

pub struct InputSystem;

impl InputSystem {
    pub fn control_player(
        mut player: UniqueViewMut<Player>,
        positions: View<Position>,
        camera: UniqueView<Camera>
    ) {
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

        if is_mouse_button_released(MouseButton::Left) {
            player.target_pos.0 = camera.0.screen_to_world(mouse_position().into());
        } else if is_mouse_button_released(MouseButton::Right) {
            let mouse_pos = camera.0.screen_to_world(mouse_position().into());

            for (id, pos) in (&positions).iter().with_id() {
                let monster_rect = Rect::new(pos.0.x - TILE_OFFSET.x, pos.0.y - TILE_OFFSET.y, TILE_SIZE.x, TILE_SIZE.y);

                if monster_rect.contains(mouse_pos) {
                    player.target.0 = Some(id);
                }
            }
        }
    }

    
}