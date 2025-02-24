use std::{collections::HashMap, thread, time::{Duration, Instant}};

use bincode::{deserialize, serialize};
use secs::prelude::World;
use wrym::{server::{Server, ServerConfig, ServerEvent}, transport::LaminarTransport};

use super::{ClientMessage, Position, ServerMessage};

pub struct Game {
    server: Server<LaminarTransport>,
    lobby: HashMap<String, u64>,
    world: World
}

impl Game {
    pub fn new() -> Self {
        let transport = LaminarTransport::new("127.0.0.1:8080");

        Self {
            server: Server::new(transport, ServerConfig::default()),
            lobby: HashMap::new(),
            world: World::default()
        }
    }

    fn handle_events(&mut self) {
        while let Some(event) = self.server.recv_event() {
            match event {
                ServerEvent::ClientConnected(addr) => {
                    let player = self.world.spawn((Position { x: 0., y: 0. },));
                    let player_id = player.to_bits();

                    // sync existing players with new clients
                    for (entity, (pos,)) in self.world.query::<(&Position,)>() {
                        if player != entity {
                            let msg = ServerMessage::PlayerConnected {
                                id: entity.to_bits(),
                                pos: Position { x: pos.x, y: pos.y }
                            };
                            
                            self.server.send_reliable_to(&addr, &serialize(&msg).unwrap(), true);
                        }
                    }

                    let msg = ServerMessage::PlayerConnected {
                        id: player_id,
                        pos: Position { x: 0., y: 0. }
                    };

                    self.server.broadcast_reliable(&serialize(&msg).unwrap(), true);
                    self.lobby.insert(addr.clone(), player_id);
                }
                ServerEvent::MessageReceived(addr, bytes) => {
                    let client_msg = deserialize::<ClientMessage>(&bytes).unwrap();
                    let player_id = self.lobby.get(&addr).unwrap();

                    match client_msg {
                        ClientMessage::PlayerMove { left, up, right, down } => {
                            for (entity, (pos,)) in self.world.query::<(&mut Position,)>() {
                                if *player_id == entity.to_bits() {
                                    pos.x += (right as i8 - left as i8) as f32;
                                    pos.y += (down as i8 - up as i8) as f32;

                                    let msg = ServerMessage::PlayerMoved {
                                        id: *player_id,
                                        pos: Position { x: pos.x, y: pos.y }
                                    };
        
                                    self.server.broadcast(&serialize(&msg).unwrap())
                                }
                            };
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
            let current_time = Instant::now();
            let elapsed_time = current_time - previous_time;

            previous_time = current_time;
            lag += elapsed_time;

            while lag >= step {
                self.update();
                self.world.run_systems();

                lag -= step;
            }

            thread::sleep(step);
        }
    }
}