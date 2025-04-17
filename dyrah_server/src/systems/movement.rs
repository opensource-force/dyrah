use std::time::{Duration, Instant};

use dyrah_shared::{GameEvent, Position, TILE_SIZE, TargetPosition, vec2};
use rand::{Rng, random_range, rng};
use secs::World;

use crate::{
    Creature, Player, PlayerView, State,
    map::{CollisionGrid, Map},
};

pub struct MovementSystem {}

impl MovementSystem {
    pub fn player(
        events: &mut Vec<GameEvent>,
        map: &Map,
        player_view: &mut PlayerView,
        grid: &CollisionGrid,
    ) -> impl FnMut(&World) {
        move |w| {
            w.query(
                |player, _: &Player, state: &mut State, tgt_pos: &mut TargetPosition| {
                    let now = Instant::now();
                    if now - state.last_move < Duration::from_millis(200) {
                        return;
                    }

                    let pos = w.get::<Position>(player).unwrap();

                    if pos.vec.distance(tgt_pos.vec) <= TILE_SIZE {
                        if let Some(path) = &mut tgt_pos.path {
                            if !path.is_empty() {
                                let next_pos = path[0];

                                if let Some(tile_pos) = map.tiled.world_to_tile(next_pos) {
                                    let x = tile_pos.x as usize;
                                    let y = tile_pos.y as usize;

                                    if grid.is_walkable(x, y) {
                                        tgt_pos.vec = path.remove(0);
                                    } else {
                                        if let Some(dest) = path.last().copied() {
                                            if let Some(new_path) =
                                                map.find_path(pos.vec, dest, &grid)
                                            {
                                                *path = new_path;
                                                if !path.is_empty() {
                                                    tgt_pos.vec = path.remove(0);
                                                }
                                            } else {
                                                tgt_pos.path = None;
                                                return;
                                            }
                                        }
                                    }
                                }
                            } else {
                                tgt_pos.path = None;
                                return;
                            }
                        }
                    }

                    if pos.vec.distance(tgt_pos.vec) < 1.0 {
                        return;
                    }

                    let dir = (tgt_pos.vec - pos.vec).normalize_or_zero();
                    if dir.x != 0.0 && dir.y != 0.0 {
                        return;
                    }

                    let next_pos = pos.vec + dir * TILE_SIZE;

                    if let Some(tile_center) = map.get_tile_center("floor", next_pos) {
                        drop(pos);
                        let mut pos = w.get_mut::<Position>(player).unwrap();

                        pos.vec = tile_center;
                        state.last_move = now;
                        player_view.position = pos.vec;

                        events.push(GameEvent::PlayerMoved {
                            id: player.id(),
                            position: pos.vec,
                            path: tgt_pos.path.clone(),
                        });
                    }
                },
            );
        }
    }

    pub fn creature(
        events: &mut Vec<GameEvent>,
        map: &Map,
        grid: &CollisionGrid,
        player_view: &PlayerView,
    ) -> impl FnMut(&World) {
        move |w| {
            let mut crea_moves = Vec::new();
            w.query(|crea, _: &Creature, state: &mut State| {
                let now = Instant::now();

                let delay = if state.following.is_some() {
                    Duration::from_millis(400)
                } else {
                    Duration::from_secs(random_range(1..=4))
                };

                if now - state.last_move < delay {
                    return;
                }

                let pos = w.get::<Position>(crea).unwrap();
                let dir = if let Some(tgt_id) = state.following {
                    let tgt = w.get::<Position>(tgt_id.into()).unwrap();
                    (tgt.vec - pos.vec).normalize_or_zero()
                } else {
                    let mut rng = rng();
                    vec2(
                        rng.random_range(-1..=1) as f32,
                        rng.random_range(-1..=1) as f32,
                    )
                };
                let next_pos = pos.vec + dir * TILE_SIZE;

                if let Some(tile_pos) = map.tiled.world_to_tile(next_pos) {
                    let x = tile_pos.x as usize;
                    let y = tile_pos.y as usize;

                    if !grid.is_walkable(x, y) || !player_view.contains(next_pos) {
                        return;
                    }

                    if let Some(tile_center) = map.get_tile_center("floor", next_pos) {
                        drop(pos);

                        let mut pos = w.get_mut::<Position>(crea).unwrap();
                        pos.vec = tile_center;

                        state.last_move = now;

                        crea_moves.push((crea.id(), pos.vec));
                    }
                }
            });

            if !crea_moves.is_empty() {
                events.push(GameEvent::CreatureBatchMoved(crea_moves));
            }
        }
    }
}
