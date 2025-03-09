use std::{
    collections::HashMap,
    thread,
    time::{Duration, Instant},
};

use bincode::{deserialize, serialize};
use dyrah_shared::{
    ClientMessage, Position, ServerMessage, TargetPosition,
    map::{TILE_SIZE, TiledMap},
};
use secs::World;
use wrym::{
    server::{Server, ServerConfig, ServerEvent},
    transport::Transport,
};

pub struct Game {
    server: Server<Transport>,
    lobby: HashMap<String, u64>,
    world: World,
    map: TiledMap,
}

impl Game {
    pub fn new() -> Self {
        let transport = Transport::new("127.0.0.1:8080");

        Self {
            server: Server::new(transport, ServerConfig::default()),
            lobby: HashMap::new(),
            world: World::default(),
            map: TiledMap::new("assets/map.json"),
        }
    }

    fn handle_events(&mut self) {
        while let Some(event) = self.server.recv_event() {
            match event {
                ServerEvent::ClientConnected(addr) => {
                    let player = self
                        .world
                        .spawn((Position { x: 0., y: 0. }, TargetPosition { x: 0., y: 0. }));
                    let player_id = player.id();

                    // sync existing players with new clients
                    self.world.query::<(&Position,)>(|entity, (pos,)| {
                        if player != entity {
                            let msg = ServerMessage::PlayerConnected {
                                id: entity.id(),
                                position: Position { x: pos.x, y: pos.y },
                            };

                            self.server
                                .send_reliable_to(&addr, &serialize(&msg).unwrap(), true);
                        }
                    });

                    let msg = ServerMessage::PlayerConnected {
                        id: player_id,
                        position: Position { x: 0., y: 0. },
                    };

                    self.server
                        .broadcast_reliable(&serialize(&msg).unwrap(), true);
                    self.lobby.insert(addr, player_id);
                }
                ServerEvent::MessageReceived(addr, bytes) => {
                    let client_msg = deserialize::<ClientMessage>(&bytes).unwrap();
                    let player_id = self.lobby.get(&addr).unwrap();

                    match client_msg {
                        ClientMessage::PlayerMove {
                            left,
                            up,
                            right,
                            down,
                        } => {
                            self.world.query::<(&mut Position,)>(|entity, (pos,)| {
                                if *player_id == entity.id() {
                                    let target_pos_x =
                                        pos.x + (right as i8 - left as i8) as f32 * TILE_SIZE;
                                    let target_pos_y =
                                        pos.y + (down as i8 - up as i8) as f32 * TILE_SIZE;

                                    if let Some((x, y)) =
                                        self.map.get_tile_center(target_pos_x, target_pos_y)
                                    {
                                        let offset = TILE_SIZE / 2.;
                                        let msg = ServerMessage::PlayerMoved {
                                            id: *player_id,
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
                            });
                        }
                    }
                }
                _ => {}
            }
        }
    }

    fn update(&mut self) {
        self.server.poll();
        self.handle_events();
    }

    pub fn run(&mut self, fps: u64) {
        let step = Duration::from_secs(1 / fps);
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
