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
            world.spawn((
                Creature {
                    last_move: Instant::now(),
                },
                Position {
                    x: rng.random_range(0..map.width) as f32 * TILE_SIZE,
                    y: rng.random_range(0..map.height) as f32 * TILE_SIZE,
                },
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

                    let player_pos = Position {
                        x: self.map.width as f32 / 2.,
                        y: self.map.height as f32 / 2.,
                    };
                    let player = self.world.spawn((Player, player_pos));

                    // sync existing players with new clients
                    self.world
                        .query::<(&Player, &Position)>(|entity, (_, pos)| {
                            if entity != player {
                                let msg = ServerMessage::PlayerConnected {
                                    position: Position { x: pos.x, y: pos.y },
                                };
                                self.server.send_reliable_to(
                                    &addr,
                                    &serialize(&msg).unwrap(),
                                    true,
                                );
                            }
                        });

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
                ClientMessage::PlayerMove {
                    left,
                    up,
                    right,
                    down,
                } => {
                    if let Some(mut pos) = self.world.get_mut::<Position>(*player) {
                        let target_pos_x = pos.x + (right as i8 - left as i8) as f32 * TILE_SIZE;
                        let target_pos_y = pos.y + (down as i8 - up as i8) as f32 * TILE_SIZE;

                        if let Some((x, y)) = self.map.get_tile_center(target_pos_x, target_pos_y) {
                            let msg = ServerMessage::PlayerMoved {
                                target_position: TargetPosition { x, y },
                            };

                            pos.x = x;
                            pos.y = y;

                            self.server.broadcast(&serialize(&msg).unwrap())
                        }
                    }
                }
            }
        }
    }

    fn update(&mut self) {
        self.server.poll();
        self.handle_events();
        self.handle_client_messages();

        self.world
            .query::<(&mut Creature, &mut Position)>(|entity, (crea, pos)| {
                let now = Instant::now();

                if now.duration_since(crea.last_move) >= Duration::from_secs(2) {
                    crea.last_move = now;
                    let dx = rng().random_range(-1..=1) as f32 * TILE_SIZE;
                    let dy = rng().random_range(-1..=1) as f32 * TILE_SIZE;

                    if dx != 0. || dy != 0. {
                        let (target_pos_x, target_pos_y) = (pos.x + dx, pos.y + dy);

                        if let Some((x, y)) = self.map.get_tile_center(target_pos_x, target_pos_y) {
                            let msg = ServerMessage::CreatureMoved {
                                id: entity.id(),
                                target_position: TargetPosition { x, y },
                            };

                            pos.x = x;
                            pos.y = y;

                            self.server.broadcast(&serialize(&msg).unwrap());
                        }
                    }
                }
            });
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
