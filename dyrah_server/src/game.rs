use std::collections::HashMap;

use bincode::{deserialize, serialize};
use glam::Vec2;
use secs::{Entity, World};
use wrym::{
    server::{Server, ServerConfig, ServerEvent},
    transport::Transport,
};

use dyrah_shared::{
    NetId,
    components::Player,
    messages::{ClientMessage, ServerMessage},
    tile_to_world,
};

use crate::components::{TargetTilePos, TilePos};

pub struct Game {
    server: Server<Transport>,
    lobby: HashMap<NetId, Entity>,
    world: World,
}

impl Game {
    pub fn new() -> Self {
        Self {
            server: Server::new(Transport::new("127.0.0.1:8080"), ServerConfig::default()),
            lobby: HashMap::new(),
            world: World::default(),
        }
    }

    pub fn handle_events(&mut self) {
        while let Some(event) = self.server.recv_event() {
            match event {
                ServerEvent::ClientConnected(id) => {
                    let addr = self.server.client_addr(id).unwrap();
                    println!("Client {} connected from {}", id, addr);

                    // sync existing players on new clients
                    for (&other_id, &player) in &self.lobby {
                        let tile_pos = self.world.get::<TilePos>(player).unwrap();
                        let msg = ServerMessage::PlayerSpawned {
                            id: other_id,
                            position: tile_to_world(tile_pos.vec),
                        };

                        self.server
                            .send_reliable_to(&addr, &serialize(&msg).unwrap(), true);
                    }

                    let player =
                        self.world
                            .spawn((Player, TilePos::default(), TargetTilePos::default()));
                    self.lobby.insert(id, player);

                    println!("Player {} spawned!", self.lobby.len());

                    let msg = ServerMessage::PlayerSpawned {
                        id,
                        position: Vec2::ZERO,
                    };
                    self.server
                        .broadcast_reliable(&serialize(&msg).unwrap(), true);
                }
                ServerEvent::ClientDisconnected(id) => {
                    println!("Client {} disconnected.", id);
                }
                ServerEvent::MessageReceived(id, bytes) => {
                    let ClientMessage::PlayerUpdate { input } = deserialize(&bytes).unwrap();

                    if let Some(&player) = self.lobby.get(&id) {
                        let mut target_pos = self.world.get_mut::<TargetTilePos>(player).unwrap();
                        let mut tile_pos = self.world.get_mut::<TilePos>(player).unwrap();

                        target_pos.vec += input.to_direction();
                        tile_pos.vec = target_pos.vec;

                        let msg = ServerMessage::PlayerMoved {
                            id,
                            position: tile_to_world(tile_pos.vec),
                        };
                        self.server.broadcast(&serialize(&msg).unwrap());
                    }
                }
            }
        }
    }

    pub fn update(&mut self, _dt: f32) {
        self.server.poll();
    }
}
