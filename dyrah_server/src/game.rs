use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    thread,
    time::{Duration, Instant},
};

use bincode::{deserialize, serialize};
use dyrah_shared::{
    ClientMessage, Health, Position, ServerMessage, TILE_OFFSET, TILE_SIZE, TargetPosition, Vec2,
    vec2,
};
use rand::{Rng, random_range, rng};
use secs::{Entity, World};
use wrym::{
    server::{Server, ServerConfig, ServerEvent},
    transport::Transport,
};

use crate::{Collider, Creature, Player, PlayerView, map::Map};

pub struct Game {
    server: Server<Transport>,
    lobby: HashMap<String, Entity>,
    world: World,
    player_view: Arc<RwLock<PlayerView>>,
    map: Map,
    dead_entities: Vec<(u64, u64)>,
}

impl Game {
    pub fn new() -> Self {
        let transport = Transport::new("127.0.0.1:8080");
        let world = World::default();
        let map = Map::new("assets/map.json");
        let mut rng = rng();

        for _ in 0..299 {
            let pos = vec2(
                rng.random_range(0..map.tiled.width) as f32 * TILE_SIZE,
                rng.random_range(0..map.tiled.height) as f32 * TILE_SIZE,
            );

            world.spawn((
                Creature::default(),
                Collider,
                Position::new(pos),
                TargetPosition::new(pos),
                Health {
                    points: rng.random_range(50.0..80.),
                },
            ));
        }

        Self {
            server: Server::new(transport, ServerConfig::default()),
            lobby: HashMap::new(),
            world,
            player_view: Arc::new(RwLock::new(PlayerView::new(
                map.get_spawn("player").unwrap(),
            ))),
            map,
            dead_entities: Vec::new(),
        }
    }

    fn handle_events(&mut self) {
        while let Some(event) = self.server.recv_event() {
            match event {
                ServerEvent::ClientConnected(addr) => {
                    println!("Client connected from {}", addr);

                    let mut crea_spawns = Vec::new();

                    self.world
                        .query::<(&Creature, &Position, &Health)>(|_, (_, pos, health)| {
                            crea_spawns.push((pos.vec, health.points));
                        });

                    if !crea_spawns.is_empty() {
                        println!("Spawned {} creatures.", crea_spawns.len());

                        let msg = ServerMessage::CreatureBatchSpawned(crea_spawns);
                        self.server
                            .send_reliable_to(&addr, &serialize(&msg).unwrap(), false);
                    }

                    self.world
                        .query::<(&Player, &Position, &Health)>(|_, (_, pos, health)| {
                            let msg = ServerMessage::PlayerConnected {
                                position: pos.vec,
                                hp: health.points,
                            };
                            self.server
                                .send_reliable_to(&addr, &serialize(&msg).unwrap(), true);
                        });

                    let spawn_pos = self.map.get_spawn("player").unwrap();
                    let player_health = Health { points: 100. };
                    let player = self.world.spawn((
                        Player::default(),
                        Collider,
                        Position::new(spawn_pos),
                        TargetPosition::new(spawn_pos),
                        player_health,
                    ));
                    self.lobby.insert(addr, player);

                    println!("Player {} spawned.", self.lobby.len());

                    let msg = ServerMessage::PlayerConnected {
                        position: spawn_pos,
                        hp: player_health.points,
                    };
                    self.server
                        .broadcast_reliable(&serialize(&msg).unwrap(), true);
                }
                ServerEvent::ClientDisconnected(addr) => {
                    println!("Client {} disconnected.", addr);
                }
                ServerEvent::MessageReceived(addr, bytes) => {
                    let msg = deserialize::<ClientMessage>(&bytes).unwrap();
                    self.handle_client_messages(&addr, msg);
                }
            }
        }
    }

    fn is_position_blocked(&self, vec: Vec2) -> bool {
        let mut blocked = false;

        if !self.map.is_walkable("colliders", vec) {
            blocked = true;
            return blocked;
        }

        self.world.query::<(&Collider, &Position)>(|_, (_, pos)| {
            if pos.vec.distance(vec) < TILE_SIZE {
                blocked = true;
                return;
            }
        });

        blocked
    }

    fn handle_client_messages(&mut self, addr: &str, msg: ClientMessage) {
        match msg {
            ClientMessage::PlayerUpdate { input } => {
                if let Some(player) = self.lobby.get(addr) {
                    self.world
                        .query::<(&mut Player, &Position, &mut TargetPosition)>(
                            |p, (state, pos, tgt_pos)| {
                                if p == *player {
                                    let is_walkable = |p| !self.is_position_blocked(p);

                                    if let Some(next_pos) = input.mouse_target_pos {
                                        if let Some(path) =
                                            self.map.find_path(pos.vec, next_pos, is_walkable)
                                        {
                                            tgt_pos.path = Some(path);
                                            tgt_pos.vec = tgt_pos.path.as_mut().unwrap().remove(0);
                                        }
                                    } else {
                                        let dir = input.to_direction();
                                        let next_pos = pos.vec + dir * TILE_SIZE;

                                        if is_walkable(next_pos) {
                                            tgt_pos.vec = next_pos;
                                            tgt_pos.path = None;
                                        }
                                    }

                                    if let Some(tgt) = input.mouse_target {
                                        state.attacking = Some(tgt);
                                    }
                                }
                            },
                        );
                }
            }
        }
    }

    fn update(&mut self) {
        self.server.poll();
        self.handle_events();

        self.world
            .query::<(&mut Player, &mut TargetPosition)>(|player, (state, tgt_pos)| {
                let now = Instant::now();
                if now - state.last_move < Duration::from_millis(200) {
                    return;
                }

                let pos = self.world.get::<Position>(player).unwrap();

                if pos.vec.distance(tgt_pos.vec) <= TILE_SIZE {
                    if let Some(path) = &mut tgt_pos.path {
                        if !path.is_empty() {
                            let is_walkable = |p| !self.is_position_blocked(p);
                            let next_pos = path[0];

                            if is_walkable(next_pos) {
                                tgt_pos.vec = path.remove(0);
                            } else {
                                if let Some(dest) = path.last().copied() {
                                    if let Some(new_path) =
                                        self.map.find_path(pos.vec, dest, is_walkable)
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

                if let Some(tile_center) = self.map.get_tile_center("floor", next_pos) {
                    drop(pos);
                    let mut pos = self.world.get_mut::<Position>(player).unwrap();
                    pos.vec = tile_center;

                    state.last_move = now;
                    self.player_view.write().unwrap().position = pos.vec;

                    self.server.broadcast(
                        &serialize(&ServerMessage::PlayerMoved {
                            id: player.id(),
                            position: pos.vec,
                            path: tgt_pos.path.clone(),
                        })
                        .unwrap(),
                    );
                }
            });

        let mut crea_moves = Vec::new();
        self.world.query::<(&mut Creature,)>(|crea, (state,)| {
            let now = Instant::now();

            let delay = if state.following.is_some() {
                Duration::from_millis(400)
            } else {
                Duration::from_secs(random_range(1..=4))
            };

            if now - state.last_move < delay {
                return;
            }

            let pos = self.world.get::<Position>(crea).unwrap();
            let dir = if let Some(tgt_id) = state.following {
                let tgt = self.world.get::<Position>(tgt_id.into()).unwrap();
                (tgt.vec - pos.vec).normalize_or_zero()
            } else {
                let mut rng = rng();
                vec2(
                    rng.random_range(-1..=1) as f32,
                    rng.random_range(-1..=1) as f32,
                )
            };
            let next_pos = pos.vec + dir * TILE_SIZE;

            if self.is_position_blocked(next_pos)
                || !self.player_view.read().unwrap().contains(next_pos)
            {
                return;
            }

            if let Some(tile_center) = self.map.get_tile_center("floor", next_pos) {
                drop(pos);

                let mut pos = self.world.get_mut::<Position>(crea).unwrap();
                pos.vec = tile_center;

                state.last_move = now;

                crea_moves.push((crea.id(), pos.vec));
            }
        });

        if !crea_moves.is_empty() {
            let msg = ServerMessage::CreatureBatchMoved(crea_moves);
            self.server.broadcast(&serialize(&msg).unwrap());
        }

        self.world.query::<(&mut Player,)>(|player, (state,)| {
            if let Some(tgt) = state.attacking {
                if Instant::now() - state.last_attack < Duration::from_millis(800) {
                    return;
                }

                if !self.world.is_attached::<Health>(tgt.into()) {
                    return;
                }

                let mut health = self.world.get_mut::<Health>(tgt.into()).unwrap();
                health.points -= rng().random_range(5.0..20.0);

                if health.points <= 0. {
                    state.attacking = None;
                    self.dead_entities.push((player.id(), tgt));

                    println!("Creature {} died.", tgt);
                    return;
                }

                state.last_attack = Instant::now();

                let msg = ServerMessage::EntityDamaged {
                    attacker: player.id(),
                    defender: tgt,
                    hp: health.points,
                };
                self.server
                    .broadcast_reliable(&serialize(&msg).unwrap(), true);
            }
        });

        self.world
            .query::<(&mut Creature, &Position)>(|crea, (state, pos)| {
                if Instant::now() - state.last_attack < Duration::from_secs(1) {
                    return;
                }

                for player in self.lobby.values() {
                    if !self.world.is_attached::<Health>(*player) {
                        continue;
                    }

                    let player_pos = self.world.get::<Position>(*player).unwrap();

                    if pos.vec.distance(player_pos.vec) > TILE_SIZE * 10.0 {
                        state.following = None;
                        continue;
                    }
                    state.following = Some(player.id());

                    if pos.vec.distance(player_pos.vec) > TILE_SIZE + TILE_OFFSET {
                        continue;
                    }

                    let mut player_health = self.world.get_mut::<Health>(*player).unwrap();
                    player_health.points -= rng().random_range(1.0..3.0);

                    if player_health.points <= 0. {
                        state.following = None;
                        self.dead_entities.push((crea.id(), player.id()));

                        println!("Player {} passed away.", player.id());
                        continue;
                    }

                    state.last_attack = Instant::now();

                    let msg = ServerMessage::EntityDamaged {
                        attacker: crea.id(),
                        defender: player.id(),
                        hp: player_health.points,
                    };
                    self.server
                        .broadcast_reliable(&serialize(&msg).unwrap(), false);
                }
            });

        for (killer, victim) in self.dead_entities.drain(..) {
            self.world.despawn(victim.into());
            self.lobby.retain(|_, entity| entity.id() != victim);

            let msg = ServerMessage::EntityDied { killer, victim };
            self.server
                .broadcast_reliable(&serialize(&msg).unwrap(), false);
        }
    }

    pub fn run(&mut self, fps: u64) {
        let step = Duration::from_secs_f64(1. / fps as f64);
        let mut previous_time = Instant::now();
        let mut lag = Duration::ZERO;

        loop {
            let now = Instant::now();
            let elapsed = now - previous_time;

            previous_time = now;
            lag += elapsed;

            while lag >= step {
                self.update();
                self.world.run_systems();

                lag -= step;
            }

            thread::sleep(step);
        }
    }
}
