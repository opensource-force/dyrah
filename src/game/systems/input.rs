use super::*;

pub struct InputSystem;

impl InputSystem {
    pub fn update(world: &mut World, map: &Map) {
        for (_, (pos, vel, moving, target)) in world.query_mut::<(
            &Position, &Velocity, &mut Moving, &mut TargetPosition
        )>() {
            if vel.0 != Vec2::ZERO {
                if let Some(tile) = map.get_tile(pos.0 + vel.0 * TILE_SIZE) {
                    if tile.walkable {
                        moving.0 = true;
                        target.0 = tile.rect.center();
                    }
                }
            }
        }
    }

    pub fn keyboard_controller<T>(world: &mut World) where T: Component {
        for (_, vel) in world.query_mut::<&mut Velocity>().with::<&T>() {
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
        }
    }

    pub fn mouse_controller<T>(world: &mut World, map: &Map, camera: &Camera2D) where T: Component {
        for (_, (moving, target)) in world.query_mut::<(
            &mut Moving, &mut TargetPosition
        )>().with::<&Player>() {
            let mouse_pos = camera.screen_to_world(mouse_position().into());
            if is_mouse_button_pressed(MouseButton::Left) {
                if let Some(tile) = map.get_tile(mouse_pos) {
                    moving.0 = true;
                    target.0 = tile.rect.center();
                }
            }
        }
    }

    pub fn ai_controller<T>(world: &mut World) where T: Component {
        if get_time() - storage::get::<WorldTime>().0 > 2.0 {
            for (_, vel) in world.query_mut::<&mut Velocity>().with::<&T>() {
                vel.0 = match rand::gen_range(0, 7) {
                    0 => vec2(0.0, -1.0),
                    1 => vec2(-1.0, 0.0),
                    2 => vec2(0.0, 1.0),
                    3 => vec2(1.0, 0.0),
                    _ => Vec2::ZERO
                };
            }

            storage::store(WorldTime(get_time()));
        }
    }
}