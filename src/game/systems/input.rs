use super::*;

pub struct InputSystem;

impl InputSystem {
    pub fn keyboard_controller<T>(world: &mut World) where T: Component {
        for (_, (pos, vel, target_pos)) in world.query_mut::<(
            &Position, &mut Velocity, &mut TargetPosition
        )>().with::<&T>() {
            vel.0 = if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
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

            if vel.0 != Vec2::ZERO {
                target_pos.0 = pos.0 + vel.0 * TILE_SIZE;
            }
        }
    }

    pub fn mouse_controller<T>(world: &mut World, camera: &Camera2D) where T: Component {
        for (_, (target_pos, target)) in world.query::<(
            &mut TargetPosition, &mut Target
        )>().with::<&Player>().iter() {
            if is_mouse_button_released(MouseButton::Left) {
                target_pos.0 = camera.screen_to_world(mouse_position().into());
            } else if is_mouse_button_released(MouseButton::Right) {
                let mouse_pos = camera.screen_to_world(mouse_position().into());

                for (monster, pos) in world.query::<&Position>().with::<&Monster>().iter() {
                    let monster_rect = Rect::new(pos.0.x - TILE_OFFSET.x, pos.0.y - TILE_OFFSET.y, TILE_SIZE.x, TILE_SIZE.y);

                    if monster_rect.contains(mouse_pos) {
                        target.0 = Some(monster);
                    }
                }
            }
        }
    }

    pub fn ai_controller<T>(world: &mut World) where T: Component {
        if get_time() - storage::get::<WorldTime>().0 > 1.0 {
            for (_, (pos, vel, target_pos)) in world.query_mut::<(
                &Position, &mut Velocity, &mut TargetPosition
            )>().with::<&T>() {
                vel.0 = match rand::gen_range(0, 7) {
                    0 => vec2(0.0, -1.0),
                    1 => vec2(-1.0, 0.0),
                    2 => vec2(0.0, 1.0),
                    3 => vec2(1.0, 0.0),
                    _ => Vec2::ZERO
                };

                if vel.0 != Vec2::ZERO {
                    target_pos.0 = pos.0 + vel.0 * TILE_SIZE;
                }
            }

            storage::store(WorldTime(get_time()));
        }
    }
}