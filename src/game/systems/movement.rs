use super::*;

pub struct MovementSystem;

impl MovementSystem {
    pub fn update(world: &mut World) {
        for (_, (pos, vel, moving, target)) in world.query_mut::<(
            &mut Position, &mut Velocity, &mut Moving, &mut TargetPosition
        )>() {
            if moving.0 {
                if pos.0 == target.0 {
                    moving.0 = false;
                } else {
                    vel.0 = (target.0 - pos.0).normalize();
                }
            }

            pos.0.x += vel.0.x;
            pos.0.y += vel.0.y;
        }
    }

    pub fn handle_player(world: &mut World, map: &mut Map, camera: &mut Camera2D) {
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