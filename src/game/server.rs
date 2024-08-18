use std::collections::VecDeque;

use crate::{game::world::EntityKind, net::server::Server, ClientMessages, EntityId, Position, ServerMessages};

use super::{map::TILE_SIZE, world::World};

pub struct Game {
    server: Server,
    msg_queue: VecDeque<ServerMessages>,
    world: World
}

impl Game {
    pub fn new() -> Self {
        let mut world = World::default();

        for i in 1..4 {
            world.spawn_entity_at(
                EntityId::from_raw(i),
                EntityKind::Enemy,
                Position { x: i as f32 * TILE_SIZE.x, y: i as f32 * TILE_SIZE.y }
            );
        }

        Self {
            server: Server::new("127.0.0.1:6667".parse().unwrap()),
            msg_queue: VecDeque::new(),
            world
        }
    }

    pub fn update(&mut self) {
        if let Some(client_id) = self.server.on_client_connect() {
            println!("Client {} connected.", client_id);

            for (id, other_entity) in &self.world.entities {
                let msg =  match other_entity.kind {
                    EntityKind::Player => ServerMessages::PlayerCreate {
                        id: *id,
                        pos: other_entity.pos,
                    },
                    EntityKind::Enemy => ServerMessages::EnemyCreate {
                        id: *id,
                        pos: other_entity.pos,
                    }
                };
                self.server.send(client_id, msg);
            }

            let player = self.world.spawn_entity(client_id.into(), EntityKind::Player);
            let msg = ServerMessages::PlayerCreate { id: client_id.into(), pos: player.pos };
            self.msg_queue.push_back(msg);
        } else if let Some((client_id, reason)) = self.server.on_client_disconnect() {
            println!("Client {} disconnected: {}", client_id, reason);
                    
            self.world.despawn_entity(client_id.into());

            let msg = ServerMessages::PlayerDelete { id: client_id.into() };
            self.msg_queue.push_back(msg);
        }

        while let Some((client_id, input)) = self.server.get_client_input() {
            let player = self.world.entities.get_mut(&client_id.into()).unwrap();
            let x = (input.right as i8 - input.left as i8) as f32;
            let y = (input.down as i8 - input.up as i8) as f32;

            if let Some(mouse_pos) = input.mouse_pos {
                player.pos = mouse_pos;
            } else {
                player.pos += Position { x, y };
            }

            let msg = &ServerMessages::PlayerUpdate {
                id: client_id.into(),
                pos: player.pos
            };
            self.server.broadcast(msg);
        }

        while let Some((client_id, client_msg)) = self.server.get_client_msg() {
            match client_msg {
                ClientMessages::PlayerCommand { id } => {
                    todo!()
                }
            }
        }

        if let Some(msg) = self.msg_queue.pop_front() {
            self.server.broadcast(msg);
        }

        self.server.update(20);
    }
}