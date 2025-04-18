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

                    if let Some(path) = &mut tgt_pos.path {
                        if path.is_empty() {
                            tgt_pos.path = None;
                            return;
                        }

                        let next_pos = path[0];
                        if pos.vec.distance(next_pos) < 1.0 {
                            path.remove(0);
                            if path.is_empty() {
                                tgt_pos.path = None;
                                return;
                            }

                            tgt_pos.vec = path[0];
                            return;
                        }

                        if !map.is_walkable(next_pos, grid) {
                            if let Some(dest) = path.last().copied() {
                                if let Some(new_path) = map.find_path(pos.vec, dest, grid) {
                                    if !new_path.is_empty() {
                                        *path = new_path;
                                        tgt_pos.vec = path[0];
                                    } else {
                                        tgt_pos.path = None;
                                    }
                                } else {
                                    tgt_pos.path = None;
                                }
                            }

                            return;
                        }

                        tgt_pos.vec = next_pos;
                    }

                    if pos.vec.distance(tgt_pos.vec) >= 1.0 {
                        if let Some(tile) = map.get_tile(tgt_pos.vec, grid) {
                            drop(pos);
                            let mut pos = w.get_mut::<Position>(player).unwrap();
                            pos.vec = tile;

                            state.last_move = now;
                            player_view.position = pos.vec;

                            events.push(GameEvent::PlayerMoved {
                                id: player.id(),
                                position: pos.vec,
                                path: tgt_pos.path.clone(),
                            });
                        }
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
            let mut rng = rng();

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
                    vec2(
                        rng.random_range(-1..=1) as f32,
                        rng.random_range(-1..=1) as f32,
                    )
                };

                let next_pos = pos.vec + dir * TILE_SIZE;

                if !player_view.contains(next_pos) {
                    return;
                }

                if let Some(tile) = map.get_tile(next_pos, grid) {
                    drop(pos);
                    let mut pos = w.get_mut::<Position>(crea).unwrap();
                    pos.vec = tile;
                    state.last_move = now;
                    crea_moves.push((crea.id(), pos.vec));
                }
            });

            if !crea_moves.is_empty() {
                events.push(GameEvent::CreatureBatchMoved(crea_moves));
            }
        }
    }
}
