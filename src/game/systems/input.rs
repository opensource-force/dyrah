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

    pub fn handle_player(world: &mut World) {
        for (_, vel) in world.query_mut::<&mut Velocity>().with::<&Player>() {
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

    pub fn handle_monsters(world: &mut World) {
        if get_time() - storage::get::<WorldTime>().0 > 2.0 {
            for (_, vel) in world.query_mut::<&mut Velocity>().with::<&Monster>() {
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