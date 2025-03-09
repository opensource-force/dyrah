use std::{
    collections::HashMap,
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
                Creature,
                Position {
                    x: rng.random_range(0..map.width) as f32 * TILE_SIZE,
                    y: rng.random_range(0..map.height) as f32 * TILE_SIZE,
                },
            ));
        }

        Self {
            server: Server::new(transport, ServerConfig::default()),
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

                    let player = self.world.spawn((Player, Position { x: 0., y: 0. }));
                    let msg = ServerMessage::PlayerConnected {
                        position: Position { x: 0., y: 0. },
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
                    let player = self.lobby.get(&addr).unwrap();

                    match client_msg {
                        ClientMessage::PlayerMove {
                            left,
                            up,
                            right,
                            down,
                        } => {
                            if let Some(mut pos) = self.world.get_mut::<Position>(*player) {
                                let target_pos_x =
                                    pos.x + (right as i8 - left as i8) as f32 * TILE_SIZE;
                                let target_pos_y =
                                    pos.y + (down as i8 - up as i8) as f32 * TILE_SIZE;

                                if let Some((x, y)) =
                                    self.map.get_tile_center(target_pos_x, target_pos_y)
                                {
                                    let offset = TILE_SIZE / 2.;
                                    let msg = ServerMessage::PlayerMoved {
                                        target_position: TargetPosition {
                                            x: target_pos_x,
                                            y: target_pos_y,
                                        },
                                    };

                                    pos.x = x - offset;
                                    pos.y = y - offset;

                                    self.server.broadcast(&serialize(&msg).unwrap())
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn update(&mut self) {
        self.server.poll();
        self.handle_events();
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
