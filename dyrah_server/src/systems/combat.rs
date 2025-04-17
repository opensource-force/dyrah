use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use dyrah_shared::{GameEvent, Health, Position, TILE_OFFSET, TILE_SIZE};
use rand::{Rng, rng};
use secs::{Entity, World};

use crate::{Creature, Player, State};

pub struct CombatSystem {}

impl CombatSystem {
    pub fn player(
        events: &mut Vec<GameEvent>,
        dead_entities: &mut Vec<(u64, u64)>,
    ) -> impl FnMut(&World) {
        move |w| {
            w.query(|player: Entity, _: &Player, state: &mut State| {
                if let Some(tgt) = state.attacking {
                    if Instant::now() - state.last_attack < Duration::from_millis(800) {
                        return;
                    }

                    if !w.is_attached::<Health>(tgt.into()) {
                        return;
                    }

                    let mut health = w.get_mut::<Health>(tgt.into()).unwrap();
                    health.points -= rng().random_range(5.0..20.0);

                    if health.points <= 0. {
                        state.attacking = None;
                        dead_entities.push((player.id(), tgt));

                        println!("Creature {} died.", tgt);
                        return;
                    }

                    state.last_attack = Instant::now();

                    events.push(GameEvent::EntityDamaged {
                        attacker: player.id(),
                        defender: tgt,
                        hp: health.points,
                    });
                }
            });
        }
    }

    pub fn creature(
        events: &mut Vec<GameEvent>,
        lobby: &mut HashMap<String, Entity>,
        dead_entities: &mut Vec<(u64, u64)>,
    ) -> impl FnMut(&World) {
        move |w| {
            w.query(
                |crea: Entity, _: &Creature, state: &mut State, pos: &Position| {
                    if Instant::now() - state.last_attack < Duration::from_secs(1) {
                        return;
                    }

                    for player in lobby.values() {
                        if !w.is_attached::<Health>(*player) {
                            continue;
                        }

                        let player_pos = w.get::<Position>(*player).unwrap();

                        if pos.vec.distance(player_pos.vec) > TILE_SIZE * 10.0 {
                            state.following = None;
                            continue;
                        }
                        state.following = Some(player.id());

                        if pos.vec.distance(player_pos.vec) > TILE_SIZE + TILE_OFFSET {
                            continue;
                        }

                        let mut player_health = w.get_mut::<Health>(*player).unwrap();
                        player_health.points -= rng().random_range(1.0..3.0);

                        if player_health.points <= 0. {
                            state.following = None;
                            dead_entities.push((crea.id(), player.id()));

                            println!("Player {} passed away.", player.id());
                            continue;
                        }

                        state.last_attack = Instant::now();

                        events.push(GameEvent::EntityDamaged {
                            attacker: crea.id(),
                            defender: player.id(),
                            hp: player_health.points,
                        });
                    }
                },
            );

            for (killer, victim) in dead_entities.drain(..) {
                w.despawn(victim.into());
                lobby.retain(|_, entity| entity.id() != victim);

                events.push(GameEvent::EntityDied { killer, victim });
            }
        }
    }
}
