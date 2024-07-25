use super::*;
use crate::game::{Moving, Player, Position, TargetPosition};

pub struct MovementSystem;

impl MovementSystem {
    pub fn update(
        player: UniqueView<Player>,
        mut map: UniqueViewMut<Map>,
        mut camera: UniqueViewMut<Viewport>,
        mut positions: ViewMut<Position>,
        mut movings: ViewMut<Moving>,
        mut target_positions: ViewMut<TargetPosition>,
    ) {
        map.update(Rect::new(
            positions[player.0].0.x - screen_width() / 2.0 - TILE_SIZE.x,
            positions[player.0].0.y - screen_height() / 2.0 - TILE_SIZE.y,
            screen_width() + TILE_SIZE.x,
            screen_height() + TILE_SIZE.y,
        ));
        camera.update(positions[player.0].0, screen_width(), screen_height());

        for (pos, mov, target_pos) in (&mut positions, &mut movings, &mut target_positions).iter() {
            if let Some(tile) = map.get_tile(target_pos.0) {
                if tile.walkable {
                    mov.0 = true;
                    target_pos.0 = tile.rect.center();
                }
            } else {
                mov.0 = false;
            }

            if mov.0 {
                if pos.0 == target_pos.0 {
                    mov.0 = false;
                } else {
                    let direction = target_pos.0 - pos.0;
                    let dx = direction.x.abs();
                    let dy = direction.y.abs();
                    let vel = direction.signum();

                    if pos.0.abs_diff_eq(target_pos.0, 1.0) {
                        pos.0 = target_pos.0;
                    } else if dx > dy {
                        pos.0.x += vel.x;
                    } else if dy > dx {
                        pos.0.y += vel.y;
                    } else {
                        pos.0 += vel;
                    }
                }
            }
        }
    }
}
