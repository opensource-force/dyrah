use std::{
    collections::HashMap,
    thread,
    time::{Duration, Instant},
};

use bincode::{deserialize, serialize};
use dyrah_shared::{
    ClientMessage, Health, Position, ServerMessage, TargetPosition, Vec2,
    map::{TILE_SIZE, TiledMap},
};
use rand::{Rng, random_range, rng};
use secs::{Entity, World};
use wrym::{
    server::{Server, ServerConfig, ServerEvent},
    transport::Transport,
};

use crate::{Collider, Creature, Player, PlayerView};

pub struct Game {
    server: Server<Transport>,
    lobby: HashMap<String, Entity>,
    world: World,
    map: TiledMap,
    dead_entities: Vec<u64>,
}

impl Game {
    pub fn new() -> Self {
        let transport = Transport::new("127.0.0.1:8080");
        let mut world = World::default();
        let map = TiledMap::new("assets/map.json");
        let mut rng = rng();

        world.add_resource(PlayerView::default());

        for _ in 0..300 {
            let pos = Vec2::new(
                rng.random_range(0..map.width) as f32 * TILE_SIZE,
                rng.random_range(0..map.height) as f32 * TILE_SIZE,
            );

            let now = Instant::now();
            world.spawn((
                Creature {
                    attacking: None,
                    last_move: now,
                    last_attack: now,
                },
                Collider,
                Position::from(pos),
                TargetPosition::from(pos),
                Health {
                    points: rng.random_range(50.0..80.),
                },
            ));
        }

        Self {
            server: Server::new(transport, ServerConfig::default()),
            lobby: HashMap::new(),
            world,
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

                    let player_pos =
                        Vec2::new(self.map.width as f32 / 2., self.map.height as f32 / 2.);
                    let player_health = Health { points: 100. };
                    let player = self.world.spawn((
                        Player {
                            attacking: None,
                            last_attack: Instant::now(),
                        },
                        Collider,
                        Position::from(player_pos),
                        TargetPosition::new(player_pos.x, player_pos.y),
                        player_health,
                    ));
                    self.lobby.insert(addr, player);

                    println!("Player {} spawned.", self.lobby.len());

                    let msg = ServerMessage::PlayerConnected {
                        position: player_pos,
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
                let player = self.lobby.get(addr).unwrap();
                let pos = self.world.get::<Position>(*player).unwrap();
                let tgt_pos = if let Some(pos) = input.mouse_target_pos {
                    pos
                } else {
                    let dir = input.to_direction();
                    pos.vec + dir * TILE_SIZE
                };

                if let Some(tgt) = input.mouse_target {
                    let mut player_state = self.world.get_mut::<Player>(*player).unwrap();
                    player_state.attacking = Some(tgt);
                }

                if self.is_position_blocked(tgt_pos.into()) {
                    return;
                }

                if let Some(tile_center) = self.map.get_tile_center("base", tgt_pos.into()) {
                    let mut target_pos = self.world.get_mut::<TargetPosition>(*player).unwrap();
                    target_pos.vec = tile_center;
                }
            }
        }
    }

    fn update(&mut self) {
        self.server.poll();
        self.handle_events();

        let mut player_view = self.world.get_resource_mut::<PlayerView>().unwrap();

        self.world
            .query::<(&mut Player, &mut Position, &TargetPosition)>(
                |entity, (_, pos, target_pos)| {
                    pos.vec = target_pos.vec;

                    if pos.vec.distance(target_pos.vec) < TILE_SIZE {
                        player_view.position = pos.vec;

                        let msg = ServerMessage::PlayerMoved {
                            id: entity.id(),
                            position: pos.vec,
                        };
                        self.server.broadcast(&serialize(&msg).unwrap())
                    }
                },
            );

        let mut crea_moves = Vec::new();

        self.world.query::<(&mut Creature, &mut TargetPosition)>(
            |entity, (crea_state, target_pos)| {
                let now = Instant::now();
                if now - crea_state.last_move < Duration::from_secs(random_range(2..=4)) {
                    return;
                }
                crea_state.last_move = now;

                let pos = self.world.get::<Position>(entity).unwrap();

                target_pos.vec += if let Some(tgt) = crea_state.attacking {
                    let player_pos = self.world.get::<Position>(tgt.into()).unwrap();
                    (player_pos.vec - pos.vec).signum() * TILE_SIZE
                } else {
                    let mut rng = rng();
                    let dir = Vec2::new(
                        rng.random_range(-1..=1) as f32,
                        rng.random_range(-1..=1) as f32,
                    );
                    dir * TILE_SIZE
                };

                if self.is_position_blocked(target_pos.vec) || !player_view.contains(target_pos.vec)
                {
                    return;
                }

                if let Some(tile_center) = self.map.get_tile_center("base", target_pos.vec) {
                    drop(pos);

                    let mut pos = self.world.get_mut::<Position>(entity).unwrap();
                    pos.vec = tile_center;

                    crea_moves.push((entity.id(), pos.vec));
                }
            },
        );

        drop(player_view);

        if !crea_moves.is_empty() {
            let msg = ServerMessage::CreatureBatchMoved(crea_moves);
            self.server.broadcast(&serialize(&msg).unwrap());
        }

        self.world.query::<(&mut Player,)>(|_, (player_state,)| {
            if let Some(tgt) = player_state.attacking {
                if Instant::now() - player_state.last_attack < Duration::from_millis(800) {
                    return;
                }

                if !self.world.is_attached::<Health>(tgt.into()) {
                    return;
                }

                let mut health = self.world.get_mut::<Health>(tgt.into()).unwrap();
                health.points -= rng().random_range(5.0..20.);

                if health.points <= 0. {
                    player_state.attacking = None;
                    self.dead_entities.push(tgt);

                    println!("Creature {} died.", tgt);
                    return;
                }

                player_state.last_attack = Instant::now();

                let msg = ServerMessage::EntityDamaged {
                    id: tgt,
                    hp: health.points,
                };
                self.server
                    .broadcast_reliable(&serialize(&msg).unwrap(), true);
            }
        });

        self.world
            .query::<(&mut Creature, &Position)>(|_, (crea_state, pos)| {
                if Instant::now() - crea_state.last_attack < Duration::from_secs(1) {
                    return;
                }

                for player in self.lobby.values() {
                    if !self.world.is_attached::<Health>(*player) {
                        return;
                    }

                    let player_pos = self.world.get::<Position>(*player).unwrap();

                    if pos.vec.distance(player_pos.vec) < TILE_SIZE * 5. {
                        crea_state.attacking = Some(player.id());

                        if pos.vec.distance(player_pos.vec) > TILE_SIZE {
                            return;
                        }

                        let mut player_health = self.world.get_mut::<Health>(*player).unwrap();
                        player_health.points -= rng().random_range(2.0..5.);

                        if player_health.points <= 0. {
                            crea_state.attacking = None;
                            self.dead_entities.push(player.id());

                            println!("Player {} passed away.", player.id());
                            return;
                        }

                        crea_state.last_attack = Instant::now();

                        let msg = ServerMessage::EntityDamaged {
                            id: player.id(),
                            hp: player_health.points,
                        };
                        self.server
                            .broadcast_reliable(&serialize(&msg).unwrap(), false);
                    }
                }
            });

        for id in self.dead_entities.drain(..) {
            self.world.despawn(id.into());

            let msg = ServerMessage::EntityDied { id };
            self.server
                .broadcast_reliable(&serialize(&msg).unwrap(), true);
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
