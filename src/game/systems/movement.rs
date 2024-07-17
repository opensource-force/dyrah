use super::*;
use crate::game::{Moving, Player, Position, TargetPosition, Velocity};

pub struct MovementSystem;

impl MovementSystem {
    pub fn update(
        player: UniqueView<Player>,
        mut pos: ViewMut<Position>,
        vel: View<Velocity>,
        mut moving: ViewMut<Moving>,
        target_pos: View<TargetPosition>,
    ) {
        let player_pos = &mut pos[player.0];
        let player_target_pos = &target_pos[player.0];

        if moving[player.0].0 {
            if player_pos.0 == player_target_pos.0 {
                moving[player.0].0 = false;
            } else {
                let direction = player_target_pos.0 - player_pos.0;
                let dx = direction.x.abs();
                let dy = direction.y.abs();
                let vel = direction.signum();

                if player_pos.0.abs_diff_eq(player_target_pos.0, 1.0) {
                    player_pos.0 = player_target_pos.0;
                } else if dx > dy {
                    player_pos.0.x += vel.x;
                } else if dy > dx {
                    player_pos.0.y += vel.y;
                } else {
                    player_pos.0 += vel;
                }
            }
        }

        for (pos, vel) in (&mut pos, &vel).iter() {
            pos.0 += vel.0;
        }
    }

    pub fn move_player(
        player: UniqueView<Player>,
        mut map: UniqueViewMut<Map>,
        mut camera: UniqueViewMut<Viewport>,
        mut moving: ViewMut<Moving>,
        mut target_pos: ViewMut<TargetPosition>,
        pos: View<Position>,
    ) {
        if let Some(tile) = map.get_tile(target_pos[player.0].0) {
            if tile.walkable {
                moving[player.0].0 = true;
                target_pos[player.0].0 = tile.rect.center();
            }
        } else {
            moving[player.0].0 = false;
        }

        map.update(Rect::new(
            pos[player.0].0.x - screen_width() / 2.0 - TILE_SIZE.x,
            pos[player.0].0.y - screen_height() / 2.0 - TILE_SIZE.y,
            screen_width() + TILE_SIZE.x,
            screen_height() + TILE_SIZE.y,
        ));
        camera.update(pos[player.0].0);
    }
}
