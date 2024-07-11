use super::*;

pub struct MovementSystem;

impl MovementSystem {
    pub fn update(world: &mut World, map: &mut Map, camera: &mut Camera2D) {
        for (_, (pos, vel, moving, target_pos)) in world.query_mut::<(
            &mut Position, &mut Velocity, &mut Moving, &mut TargetPosition
        )>() {
            if let Some(tile) = map.get_tile(target_pos.0) {
                if tile.walkable {
                    moving.0 = true;
                    target_pos.0 = tile.rect.center();
                }
            } else {
                moving.0 = false;
            }

            if moving.0 {
                if pos.0 == target_pos.0 {
                    moving.0 = false;
                } else {
                    let direction = target_pos.0 - pos.0;
                    let dx = direction.x.abs();
                    let dy = direction.y.abs();
                    vel.0 = direction.signum();

                    if pos.0.abs_diff_eq(target_pos.0, 1.0) {
                        pos.0 = target_pos.0;
                    } else if dx > dy {
                        pos.0.x += vel.0.x;
                    } else if dy > dx {
                        pos.0.y += vel.0.y;
                    } else {
                        pos.0 += vel.0;
                    }
                }
            }
        }

        for (_, pos) in world.query_mut::<&Position>().with::<&Player>() {
            storage::store(PlayerView(Rect::new(
                pos.0.x - screen_width() / 2.0 - TILE_SIZE.x,
                pos.0.y - screen_height() / 2.0 - TILE_SIZE.y,
                screen_width() + TILE_SIZE.x,
                screen_height() + TILE_SIZE.y
            )));
            
            map.update(storage::get::<PlayerView>().0);
            camera.target = pos.0;
        }
    }
}