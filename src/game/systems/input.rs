use super::*;
use crate::game::{Player, Position, Target, TargetPosition, Velocity};

pub struct InputSystem;

impl InputSystem {
    pub fn control_player(
        player: UniqueView<Player>,
        pos: View<Position>,
        viewport: UniqueView<Viewport>,
        mut vel: ViewMut<Velocity>,
        mut target: ViewMut<Target>,
        mut target_pos: ViewMut<TargetPosition>,
    ) {
        let player_vel = &mut vel[player.0];
        let player_pos = &pos[player.0];
        let player_target_pos = &mut target_pos[player.0];

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

        if player_vel.0 != Vec2::ZERO {
            player_target_pos.0 = player_pos.0 + player_vel.0 * TILE_SIZE;
        }

        if is_mouse_button_released(MouseButton::Left) {
            player_target_pos.0 = viewport.camera.screen_to_world(mouse_position().into());
        } else if is_mouse_button_released(MouseButton::Right) {
            let mouse_pos = viewport.camera.screen_to_world(mouse_position().into());

            for (id, pos) in (&pos).iter().with_id() {
                let monster_rect = Rect::new(
                    pos.0.x - TILE_OFFSET.x,
                    pos.0.y - TILE_OFFSET.y,
                    TILE_SIZE.x,
                    TILE_SIZE.y,
                );

                if monster_rect.contains(mouse_pos) {
                    target.add_component_unchecked(player.0, Target(id));
                }
            }
        }
    }
}
