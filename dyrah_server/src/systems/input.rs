use dyrah_shared::{ClientInput, Position, TILE_SIZE, TargetPosition};
use secs::{Entity, World};

use crate::{
    State,
    map::{CollisionGrid, Map},
};

pub struct InputSystem {}

impl InputSystem {
    pub fn player(
        player: Entity,
        input: ClientInput,
        map: &Map,
        grid: &CollisionGrid,
    ) -> impl Fn(&World) {
        move |w| {
            let pos = w.get::<Position>(player).unwrap();
            let mut tgt_pos = w.get_mut::<TargetPosition>(player).unwrap();
            let mut state = w.get_mut::<State>(player).unwrap();

            if let Some(next_pos) = input.mouse_target_pos {
                if let Some(path) = map.find_path(pos.vec, next_pos, &grid) {
                    tgt_pos.path = Some(path);
                    if !tgt_pos.path.as_ref().unwrap().is_empty() {
                        tgt_pos.vec = tgt_pos.path.as_mut().unwrap().remove(0);
                    }
                }
            } else {
                let dir = input.to_direction();
                let next_pos = pos.vec + dir * TILE_SIZE;

                if let Some(tile_pos) = map.tiled.world_to_tile(next_pos) {
                    let x = tile_pos.x as usize;
                    let y = tile_pos.y as usize;

                    if grid.is_walkable(x, y) {
                        tgt_pos.vec = next_pos;
                        tgt_pos.path = None;
                    }
                }
            }

            if let Some(tgt) = input.mouse_target {
                state.attacking = Some(tgt);
            }
        }
    }
}
