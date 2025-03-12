use std::{
    collections::{HashMap, VecDeque},
    thread,
    time::{Duration, Instant},
};

use bincode::{deserialize, serialize};
use dyrah_shared::{
    ClientMessage, Creature, Player, Position, ServerMessage, TargetPosition,
    map::{TILE_SIZE, TiledMap},
};
use rand::{Rng, rng};
use secs::{Entity, World};
use wrym::{
    server::{Server, ServerConfig, ServerEvent},
    transport::Transport,
};

use crate::Collider;

pub struct Game {
    server: Server<Transport>,
    client_messages: VecDeque<(String, ClientMessage)>,
    lobby: HashMap<String, Entity>,
    world: World,
    map: TiledMap,
}

impl Game {
    pub fn new() -> Self {
        let transport = Transport::new("127.0.0.1:8080");
        let world = World::default();
        let map = TiledMap::new("assets/map.json");
        let mut rng = rng();

        for _ in 0..50 {
            let pos_x = rng.random_range(0..map.width) as f32 * TILE_SIZE;
            let pos_y = rng.random_range(0..map.height) as f32 * TILE_SIZE;
            world.spawn((
                Creature {
                    last_move: Instant::now(),
                },
                Collider,
                Position { x: pos_x, y: pos_y },
                TargetPosition { x: pos_x, y: pos_y },
            ));
        }

        Self {
            server: Server::new(transport, ServerConfig::default()),
            client_messages: VecDeque::new(),
            lobby: HashMap::new(),
            world,
            map,
        }
    }

    fn handle_events(&mut self) {
        while let Some(event) = self.server.recv_event() {
            match event {
                ServerEvent::ClientConnected(addr) => {
                    self.world.query::<(&Creature, &Position)>(|_, (_, pos)| {
                        let msg = ServerMessage::CreatureSpawned { position: *pos };
                        self.server
                            .send_reliable_to(&addr, &serialize(&msg).unwrap(), false);
                    });

                    // sync existing players with new clients
                    self.world.query::<(&Player, &Position)>(|_, (_, pos)| {
                        let msg = ServerMessage::PlayerConnected {
                            position: Position { x: pos.x, y: pos.y },
                        };
                        self.server
                            .send_reliable_to(&addr, &serialize(&msg).unwrap(), true);
                    });

                    let player_pos = Position {
                        x: self.map.width as f32 / 2.,
                        y: self.map.height as f32 / 2.,
                    };
                    let player = self.world.spawn((
                        Player,
                        Collider,
                        player_pos,
                        TargetPosition {
                            x: player_pos.x,
                            y: player_pos.y,
                        },
                    ));
                    let msg = ServerMessage::PlayerConnected {
                        position: player_pos,
                    };

                    self.server
                        .broadcast_reliable(&serialize(&msg).unwrap(), true);
                    self.lobby.insert(addr, player);
                }
                ServerEvent::ClientDisconnected(_addr) => {
                    unimplemented!()
                }
                ServerEvent::MessageReceived(addr, bytes) => {
                    let client_msg = deserialize::<ClientMessage>(&bytes).unwrap();

                    self.client_messages.push_back((addr, client_msg));
                }
            }
        }
    }

    fn handle_client_messages(&mut self) {
        while let Some((addr, client_msg)) = self.client_messages.pop_front() {
            let player = self.lobby.get(&addr).unwrap();

            match client_msg {
                ClientMessage::PlayerMove { input } => {
                    let mut target_pos = self.world.get_mut::<TargetPosition>(*player).unwrap();
                    let pos = self.world.get::<Position>(*player).unwrap();
                    let (dx, dy) = input.to_direction();
                    (target_pos.x, target_pos.y) = (pos.x + dx * TILE_SIZE, pos.y + dy * TILE_SIZE);
                    let is_position_blocked = self.is_position_blocked(target_pos.x, target_pos.y);

                    drop(pos);

                    if self.map.is_walkable("props", target_pos.x, target_pos.y) {
                        if !is_position_blocked {
                            let mut pos = self.world.get_mut::<Position>(*player).unwrap();

                            if let Some((x, y)) =
                                self.map
                                    .get_tile_center("floor", target_pos.x, target_pos.y)
                            {
                                pos.x = x;
                                pos.y = y;

                                let msg = ServerMessage::PlayerMoved {
                                    id: player.id(),
                                    target_position: TargetPosition { x, y },
                                };
                                self.server.broadcast(&serialize(&msg).unwrap())
                            }
                        }
                    }
                }
            }
        }
    }

    fn is_position_blocked(&self, x: f32, y: f32) -> bool {
        let mut blocked = false;

        self.world.query::<(&Collider, &Position)>(|_, (_, pos)| {
            if (pos.x - x).abs() < TILE_SIZE && (pos.y - y).abs() < TILE_SIZE {
                blocked = true;
            }
        });

        blocked
    }

    fn update(&mut self) {
        self.server.poll();
        self.handle_events();
        self.handle_client_messages();
        let mut crea_updates = Vec::new();

        self.world
            .query::<(&mut Creature, &Position, &mut TargetPosition)>(
                |entity, (crea, pos, target_pos)| {
                    let now = Instant::now();

                    if now.duration_since(crea.last_move) >= Duration::from_secs(3) {
                        crea.last_move = now;
                        let mut rng = rng();
                        let (dx, dy) = (rng.random_range(-1..=1), rng.random_range(-1..=1));
                        target_pos.x = pos.x + dx as f32 * TILE_SIZE;
                        target_pos.y = pos.y + dy as f32 * TILE_SIZE;

                        if self.map.is_walkable("props", target_pos.x, target_pos.y) {
                            if !self.is_position_blocked(target_pos.x, target_pos.y) {
                                crea_updates.push(entity.id());
                            }
                        }
                    }
                },
            );

        for id in crea_updates {
            self.world.query::<(&mut Position, &mut TargetPosition)>(
                |entity, (pos, target_pos)| {
                    if entity.id() == id {
                        if let Some((x, y)) =
                            self.map
                                .get_tile_center("floor", target_pos.x, target_pos.y)
                        {
                            pos.x = x;
                            pos.y = y;

                            let msg = ServerMessage::CreatureMoved {
                                id,
                                target_position: TargetPosition { x, y },
                            };

                            self.server.broadcast(&serialize(&msg).unwrap());
                        }
                    }
                },
            );
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
